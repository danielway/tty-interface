use crate::config::Configuration;
use crate::device::Device;
use crate::layout::{InterfaceLayout, LineLayout};
use crate::line::{Line, LineId};
use crate::mode::RenderMode;
use crate::position::{AbsolutePosition, Position};
use crate::segment::SegmentId;
use crate::terminal::Terminal;
use crate::{Error, Result};
use std::collections::HashMap;
use std::mem::swap;

/// A terminal interface (TUI) which efficiently handles updates and layout.
#[derive(Debug)]
pub struct Interface<'a> {
    /// The currently-rendered state.
    current: InterfaceState,

    /// If changed, the state with queued and unapplied changes.
    alternate: Option<InterfaceState>,

    /// Coordinates calls to the terminal with a specified text render strategy.
    terminal: Terminal<'a>,

    /// The line value store. Ordering information is stored in the state.
    lines: HashMap<LineId, Line>,

    /// Whether to attempt optimizations when rendering.
    allow_optimization: bool,
}

impl<'a> Interface<'a> {
    /// A new interface using the specified output device.
    pub fn new(writer: &'a mut dyn std::io::Write) -> Result<Interface<'a>> {
        Self::new_with_configuration(writer, Configuration::default())
    }

    /// A new interface using the specified output device and configuration options.
    pub fn new_with_configuration(
        writer: &'a mut dyn std::io::Write,
        configuration: Configuration,
    ) -> Result<Interface<'a>> {
        let mut terminal = Terminal::new(
            Device::new(writer),
            configuration.cursor_mode(),
            configuration.render_mode(),
        )?;

        if configuration.render_mode() == RenderMode::Full {
            terminal.reset()?;
        }

        Ok(Interface {
            current: InterfaceState::default(),
            alternate: None,
            terminal,
            lines: HashMap::new(),
            allow_optimization: true,
        })
    }

    /// The interface's cursor position. Reflects the latest state, even if staged changes have not
    /// yet been applied to the interface.
    pub fn cursor(&self) -> Option<Position> {
        match self.alternate {
            Some(ref alternate) => alternate.cursor,
            None => self.current.cursor,
        }
    }

    /// The interface's line identifiers. Reflects the latest state, even if staged changes have not
    /// yet been applied to the interface.
    pub fn line_ids(&self) -> &Vec<LineId> {
        match self.alternate {
            Some(ref alternate) => &alternate.lines,
            None => &self.current.lines,
        }
    }

    /// The interface's lines. Reflects the latest state, even if staged changes have not yet been
    /// applied to the interface.
    pub fn lines(&self) -> Vec<&Line> {
        self.line_ids()
            .iter()
            .map(|id| self.lines.get(id).unwrap())
            .collect()
    }

    /// Retrieves several line references by their identifiers.
    pub fn get_lines(&self, ids: &Vec<LineId>) -> Result<Vec<&Line>> {
        ids.iter()
            .map(|id| self.lines.get(id).ok_or(Error::LineIdInvalid))
            .collect()
    }

    /// Retrieves a line reference by its identifier.
    pub fn get_line(&self, id: &LineId) -> Result<&Line> {
        self.lines.get(&id).ok_or(Error::LineIdInvalid)
    }

    /// Retrieves a mutable line reference by its identifier.
    pub fn get_line_mut(&mut self, id: &LineId) -> Result<&mut Line> {
        self.lines.get_mut(&id).ok_or(Error::LineIdInvalid)
    }

    /// Determines the specified line's index in this interface.
    pub fn get_line_index(&self, id: &LineId) -> Result<usize> {
        self.line_ids()
            .iter()
            .position(|oth| oth == id)
            .ok_or(Error::LineIdInvalid)
    }

    /// Sets the cursor position. The cursor update will be staged until changes are applied.
    pub fn set_cursor(&mut self, cursor: Position) {
        let alternate = self.get_alternate_mut();
        alternate.cursor = Some(cursor);
    }

    /// Hides the cursor. The cursor update will be staged until changes are applied.
    pub fn hide_cursor(&mut self) {
        let alternate = self.get_alternate_mut();
        alternate.cursor = None;
    }

    /// Updates whether to allow optimizations when rendering.
    pub fn set_optimization(&mut self, allow_optimization: bool) {
        self.allow_optimization = allow_optimization;
    }

    /// Appends a new line to this interface. The line addition will be staged until changes are
    /// applied. Note that the returned line may have other changes staged against it for the same
    /// update.
    pub fn add_line(&mut self) -> &mut Line {
        let line_id = self.create_line();

        let alternate = self.get_alternate_mut();
        alternate.lines.push(line_id);

        self.get_line_mut(&line_id).unwrap()
    }

    /// Inserts a new line in this interface at a specified index. The line addition will be
    /// staged until changes are applied. Note that the returned line may have other changes staged
    /// against it for the same update.
    pub fn insert_line(&mut self, index: usize) -> Result<&mut Line> {
        if index > self.get_alternate().lines.len() {
            return Err(Error::LineOutOfBounds);
        }

        let line_id = self.create_line();

        let alternate = self.get_alternate_mut();
        alternate.lines.insert(index, line_id);

        Ok(self.get_line_mut(&line_id).unwrap())
    }

    /// Removes a line with the specified identifier from the interface. The line removal will be
    /// staged until changes are applied.
    pub fn remove_line(&mut self, id: &LineId) -> Result<()> {
        let index = self.get_line_index(id)?;

        let alternate = self.get_alternate_mut();
        alternate.lines.remove(index);

        Ok(())
    }

    /// Removes a line from this interface at the specified index. The line removal will be staged
    /// until changes are applied.
    pub fn remove_line_at(&mut self, index: usize) -> Result<LineId> {
        let alternate = self.get_alternate_mut();

        if index >= alternate.lines.len() {
            return Err(Error::LineOutOfBounds);
        }

        let line_id = alternate.lines.remove(index);

        Ok(line_id)
    }

    /// Move a segment from one line to the specified index in another.
    pub fn move_segment(
        &mut self,
        segment_id: &SegmentId,
        from_line_id: &LineId,
        to_line_id: &LineId,
    ) -> Result<()> {
        let from_line = self.get_line_mut(from_line_id)?;
        let segment = from_line.take_segment(segment_id)?;
        let to_line = self.get_line_mut(to_line_id)?;
        to_line.append_given_segment(segment);
        Ok(())
    }

    /// Whether this line or its segments have any staged changes.
    pub fn has_changes(&self) -> bool {
        self.alternate.is_some()
            || self
                .current
                .lines
                .iter()
                .map(|id| self.lines.get(&id).unwrap())
                .any(|line| line.has_changes())
    }

    /// Applies any staged changes for this interface and its constituent lines and segments.
    /// Returns the rendered-layout.
    pub fn apply_changes(&mut self) -> Result<InterfaceLayout> {
        let terminal_size = self.terminal.size()?;

        let mut force_render = !self.allow_optimization;
        if let Some(ref layout) = self.current.layout {
            if terminal_size != layout.terminal_size() {
                self.terminal.reset()?;
                force_render = true;
            }
        }

        if !self.has_changes() && !force_render {
            return Ok(self.current.layout.clone().unwrap());
        }

        let previous_length = match self.current.layout {
            Some(ref current_layout) => current_layout.rendered_line_count(),
            None => 0,
        };

        self.terminal.hide_cursor()?;

        let line_layouts = if self.has_alternate() {
            let alternate = self.swap_alternate();
            self.prune_lines(&alternate.lines);
            self.render(force_render, &alternate.lines)?
        } else {
            let current_lines = self.current.lines.clone();
            self.render(force_render, &current_lines)?
        };

        let new_layout = InterfaceLayout::new(line_layouts, terminal_size);

        let new_length = new_layout.rendered_line_count();

        self.clear_overflow(previous_length, new_length)?;

        match self.current.cursor {
            Some(position) => match position {
                Position::Absolute(absolute) => {
                    self.terminal.move_cursor(absolute)?;
                    self.terminal.show_cursor()?;
                }
                Position::Relative(relative) => {
                    if let Some(absolute) = new_layout.get_absolute_position(relative) {
                        self.terminal.move_cursor(absolute)?;
                        self.terminal.show_cursor()?;
                    } else {
                        return Err(Error::CursorPositionInvalid);
                    }
                }
            },
            None => self.terminal.hide_cursor()?,
        }

        self.current.layout = Some(new_layout.clone());

        self.terminal.flush()?;

        Ok(new_layout)
    }

    /// Advances the cursor to the end of the interface.
    pub fn advance_to_end(&mut self) -> Result<()> {
        let line_count = self.current.layout.as_ref().unwrap().rendered_line_count();
        self.terminal
            .move_cursor(AbsolutePosition::new(0, line_count))
    }

    /// Renders the current interface state at the `location`. Attempts to optimize the update using
    /// the current state or `previous_lines`, if available. If `force_render`, no optimizations are
    /// made and the interface is fully re-rendered.
    fn render(
        &mut self,
        mut force_render: bool,
        previous_lines: &Vec<LineId>,
    ) -> Result<Vec<LineLayout>> {
        let mut line_layouts = Vec::new();

        let mut line_cursor = AbsolutePosition::default();
        for (index, line_id) in self.current.lines.iter().enumerate() {
            let mut previous_layout = None;
            if previous_lines.get(index) == Some(line_id) {
                let current_lines = self.current.layout.as_ref().unwrap().lines();
                previous_layout = Some(current_lines[index].clone());
            }

            if previous_layout.is_none() {
                force_render = true;
            }

            let line = self.lines.get_mut(line_id).unwrap();

            let line_layout = if !force_render && previous_layout.is_some() && !line.has_changes() {
                previous_layout.clone().unwrap()
            } else {
                line.update(&mut self.terminal, line_cursor, force_render)?
            };

            line_cursor = line_cursor.add_rows(line_layout.wrapped_line_count() as i16);

            if let Some(previous_layout) = previous_layout {
                if previous_layout.wrapped_line_count() != line_layout.wrapped_line_count() {
                    force_render = true;
                }
            } else {
                force_render = true;
            }

            line_layouts.push(line_layout);
        }

        Ok(line_layouts)
    }

    /// If the previous interface was longer than the current, clears any overhanging lines.
    fn clear_overflow(&mut self, previous_length: u16, new_length: u16) -> Result<()> {
        let length_difference = previous_length as i16 - new_length as i16;

        if length_difference > 0 {
            for line in new_length..previous_length {
                let line_position = AbsolutePosition::new(0, line);
                self.terminal.move_cursor(line_position)?;
                self.terminal.clear_line()?;
            }
        }
        Ok(())
    }

    /// Prune any line values whose IDs are no longer included in this interface.
    fn prune_lines(&mut self, line_ids: &Vec<LineId>) {
        let line_id_iter = line_ids.iter();
        let removed_line_ids = line_id_iter.filter(|id| !self.current.lines.contains(id));

        removed_line_ids.for_each(|id| {
            self.lines.remove(id);
        });
    }

    /// Create a new line, add it to the map, and return its identifier. Note that the returned
    /// identifier still needs to be ordered in state.
    fn create_line(&mut self) -> LineId {
        let line = Line::new();
        let line_id = line.identifier();
        self.lines.insert(line_id, line);
        line_id
    }

    /// Whether this interface has an alternate state, which may contain added or removed lines.
    fn has_alternate(&self) -> bool {
        self.alternate.is_some()
    }

    /// Get a reference to this interface's alternate state, creating it and dirtying the interface
    /// if necessary.
    fn get_alternate(&mut self) -> &InterfaceState {
        if self.alternate.is_none() {
            self.alternate = Some(self.current.clone());
        }

        &self.alternate.as_ref().unwrap()
    }

    /// Get a mutable reference to this interface's alternate state, creating it and dirtying the
    /// interface if necessary.
    fn get_alternate_mut(&mut self) -> &mut InterfaceState {
        if self.alternate.is_none() {
            self.alternate = Some(self.current.clone());
        }

        self.alternate.as_mut().unwrap()
    }

    /// Swaps this interface's alternate state out for its current and returns the previous-current
    /// state. This interface must have an alternate for this to be called.
    fn swap_alternate(&mut self) -> InterfaceState {
        let mut alternate = self.alternate.take().unwrap();
        swap(&mut self.current, &mut alternate);
        alternate
    }
}

/// The interface's cursor, line, and, if rendered, layout information.
#[derive(Clone, Debug)]
struct InterfaceState {
    /// The desired cursor position for this state. For the working/current cursor position, see
    /// the `Terminal.cursor()`.
    cursor: Option<Position>,

    /// Line ordering. Note that these IDs must also be present in the `Interface`'s `lines` map.
    lines: Vec<LineId>,

    /// This interface's layout on screen, if it has been rendered.
    layout: Option<InterfaceLayout>,
}

impl Default for InterfaceState {
    /// An empty state with no cursor position, lines or layout.
    fn default() -> Self {
        InterfaceState {
            cursor: None,
            lines: Vec::new(),
            layout: None,
        }
    }
}
