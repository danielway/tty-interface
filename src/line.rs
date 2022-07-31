use crate::layout::{LineLayout, SegmentLayout};
use crate::position::AbsolutePosition;
use crate::segment::{Segment, SegmentId};
use crate::terminal::Terminal;
use crate::{Error, Result};
use std::collections::HashMap;
use std::mem::swap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A unique line identifier.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct LineId(usize);

/// The greatest provisioned line identifier.
static ID_VALUE: AtomicUsize = AtomicUsize::new(0);

impl LineId {
    /// Create a new, unique line identifier.
    fn new() -> Self {
        Self(ID_VALUE.fetch_add(1, Ordering::Relaxed))
    }
}

/// A line composed of text segments. Accumulates changes until applied by its parent `Interface`.
/// May be obtained from the `Interface` API.
#[derive(Debug)]
pub struct Line {
    /// This line's immutable, unique identifier.
    identifier: LineId,

    /// The currently-rendered state.
    current: LineState,

    /// If changed, the state with queued and unapplied changes.
    alternate: Option<LineState>,

    /// The segment value store. Ordering information is stored in the state.
    segments: HashMap<SegmentId, Segment>,
}

impl Line {
    /// Create a new line with no text segments.
    pub(crate) fn new() -> Line {
        Line {
            identifier: LineId::new(),
            current: LineState::default(),
            alternate: None,
            segments: HashMap::new(),
        }
    }

    /// This line's unique identifier.
    pub fn identifier(&self) -> LineId {
        self.identifier
    }

    /// This line's text segment identifiers. Reflects the latest state, even if staged changes have
    /// not yet been applied to the interface.
    pub fn segment_ids(&self) -> &Vec<SegmentId> {
        match self.alternate {
            Some(ref alternate) => &alternate.segments,
            None => &self.current.segments,
        }
    }

    /// This line's text segments. Reflects the latest state, even if staged changes have not yet
    /// been applied to the interface.
    pub fn segments(&self) -> Vec<&Segment> {
        self.segment_ids()
            .iter()
            .map(|id| self.segments.get(id).unwrap())
            .collect()
    }

    /// Retrieves several segment references by their identifiers.
    pub fn get_segments(&self, ids: &Vec<SegmentId>) -> Result<Vec<&Segment>> {
        ids.iter()
            .map(|id| self.segments.get(id).ok_or(Error::SegmentIdInvalid))
            .collect()
    }

    /// Retrieves a segment reference by its identifier.
    pub fn get_segment(&self, id: &SegmentId) -> Result<&Segment> {
        self.segments.get(&id).ok_or(Error::SegmentIdInvalid)
    }

    /// Retrieves a mutable segment reference by its identifier.
    pub fn get_segment_mut(&mut self, id: &SegmentId) -> Result<&mut Segment> {
        self.segments.get_mut(&id).ok_or(Error::SegmentIdInvalid)
    }

    /// Determines the specified segment's index in this line.
    pub fn get_segment_index(&self, id: &SegmentId) -> Result<usize> {
        self.segment_ids()
            .iter()
            .position(|oth| oth == id)
            .ok_or(Error::SegmentIdInvalid)
    }

    /// Appends a new text segment to this line. The text segment addition will be staged until
    /// changes are applied for this line's `Interface`. Note that the returned segment may have
    /// other changes staged against it for the same update.
    pub fn add_segment(&mut self) -> &mut Segment {
        let segment_id = self.create_segment();

        let alternate = self.get_alternate_mut();
        alternate.segments.push(segment_id);

        self.get_segment_mut(&segment_id).unwrap()
    }

    /// Inserts a new text segment in this line at a specified index. The segment insertion will be
    /// staged until changes are applied for this line's `Interface`. Note that the returned
    /// segment may have other changes staged against it for the same update.
    pub fn insert_segment(&mut self, index: usize) -> Result<&mut Segment> {
        if index > self.get_alternate().segments.len() {
            return Err(Error::SegmentOutOfBounds);
        }

        let segment_id = self.create_segment();

        let alternate = self.get_alternate_mut();
        alternate.segments.insert(index, segment_id);

        Ok(self.get_segment_mut(&segment_id).unwrap())
    }

    /// Removes a text segment with the specified identifier from this line. The segment removal
    /// will be staged until changes are applied for this line's `Interface`.
    pub fn remove_segment(&mut self, id: &SegmentId) -> Result<()> {
        let index = self.get_segment_index(id)?;

        let alternate = self.get_alternate_mut();
        alternate.segments.remove(index);

        Ok(())
    }

    /// Removes a text segment from this line at the specified index. The segment removal will be
    /// staged until changes are applied for this line's `Interface`.
    pub fn remove_segment_at(&mut self, index: usize) -> Result<SegmentId> {
        let alternate = self.get_alternate_mut();

        if index > alternate.segments.len() - 1 {
            return Err(Error::SegmentOutOfBounds);
        }

        let segment_id = alternate.segments.remove(index);

        Ok(segment_id)
    }

    /// Whether this line or its text segments have any staged changes.
    pub fn has_changes(&self) -> bool {
        self.alternate.is_some()
            || self
                .current
                .segments
                .iter()
                .map(|id| self.segments.get(id).unwrap())
                .any(|segment| segment.has_changes())
    }

    /// Removes the specified text segment from this line and returns a clone of it.
    pub(crate) fn take_segment(&mut self, segment_id: &SegmentId) -> Result<Segment> {
        let segment = self.get_segment(segment_id)?;
        let clone = segment.clone();
        self.remove_segment(segment_id)?;
        Ok(clone)
    }

    /// Inserts the provided text segment in this line at a specified index. The segment insertion
    /// will be staged until changes are applied for this line's `Interface`. Note that the returned
    /// segment may have other changes staged against it for the same update.
    pub(crate) fn append_given_segment(&mut self, segment: Segment) -> &mut Segment {
        let segment_id = segment.identifier();
        self.segments.insert(segment_id, segment);

        let alternate = self.get_alternate_mut();
        alternate.segments.push(segment_id);

        self.get_segment_mut(&segment_id).unwrap()
    }

    /// Render this at the specified `location`. Unless `force_render`, assumes the previous state
    /// is still valid and performs an optimized render applying staged updates if available. If
    /// `force_render`, the line and its segments are fully re-rendered.
    pub(crate) fn update(
        &mut self,
        terminal: &mut Terminal,
        location: AbsolutePosition,
        force_render: bool,
    ) -> Result<LineLayout> {
        if !force_render && !self.has_changes() {
            return if self.current.segments.is_empty() {
                Ok(LineLayout::new(self.identifier, Vec::default()))
            } else {
                Ok(self.current.layout.clone().unwrap())
            };
        }

        let previous_layout = self.current.layout.as_ref();
        let previous_end = previous_layout.and_then(|l| l.end_position());

        let segment_layouts = if self.has_alternate() {
            let alternate = self.swap_alternate();
            self.prune_segments(&alternate.segments);
            self.render(terminal, location, force_render, &alternate.segments)?
        } else {
            let current_segments = self.current.segments.clone();
            self.render(terminal, location, force_render, &current_segments)?
        };

        let new_layout = LineLayout::new(self.identifier, segment_layouts);

        let new_end = new_layout.end_position();

        let line_is_shorter = match (previous_end, new_end) {
            (Some(previous), Some(new)) => {
                new.row() == previous.row() && new.column() < previous.column()
            }
            (Some(_), None) => true,
            _ => false,
        };

        if force_render || line_is_shorter {
            if let Some(position) = new_layout.end_position() {
                terminal.move_cursor(position)?;
            }
            terminal.clear_rest_of_line()?;
        }

        self.current.layout = Some(new_layout.clone());

        Ok(new_layout)
    }

    /// Renders the current line state at the `location`. Attempts to optimize the update using the
    /// current state or `previous_segments`, if available. If `force_render`, no optimizations are
    /// made and the line is fully re-rendered.
    fn render(
        &mut self,
        terminal: &mut Terminal,
        location: AbsolutePosition,
        mut force_render: bool,
        previous_segments: &Vec<SegmentId>,
    ) -> Result<Vec<SegmentLayout>> {
        let mut segment_layouts = Vec::new();

        terminal.move_cursor(location)?;

        let mut segment_cursor = location;
        for (index, segment_id) in self.current.segments.iter().enumerate() {
            let mut previous_layout = None;
            if previous_segments.get(index) == Some(segment_id) {
                let current_segments = self.current.layout.as_ref().unwrap().segments();
                previous_layout = Some(current_segments[index].clone());
            }

            if previous_layout.is_none() {
                force_render = true;
            }

            let segment = self.segments.get_mut(segment_id).unwrap();

            let segment_layout =
                if !force_render && previous_layout.is_some() && !segment.has_changes() {
                    previous_layout.clone().unwrap()
                } else {
                    segment.update(terminal, segment_cursor, force_render)?
                };

            if let Some(last_part) = segment_layout.parts().last() {
                segment_cursor = last_part.end_position();
            }

            if let Some(previous_layout) = previous_layout {
                if previous_layout.end_position() != segment_layout.end_position() {
                    force_render = true;
                }
            } else {
                force_render = true;
            }

            segment_layouts.push(segment_layout);
        }

        Ok(segment_layouts)
    }

    /// Prune any segment values whose IDs are no longer included in this line.
    fn prune_segments(&mut self, segment_ids: &Vec<SegmentId>) {
        let segment_id_iter = segment_ids.iter();
        let removed_segment_ids = segment_id_iter.filter(|id| !self.current.segments.contains(id));

        removed_segment_ids.for_each(|id| {
            self.segments.remove(id);
        });
    }

    /// Create a new segment, add it to the map, and return its identifier. Note that the returned
    /// identifier still needs to be ordered in state.
    fn create_segment(&mut self) -> SegmentId {
        let segment = Segment::new();
        let segment_id = segment.identifier();
        self.segments.insert(segment_id, segment);
        segment_id
    }

    /// Whether this line has an alternate state, which may contain added or removed text segments.
    fn has_alternate(&self) -> bool {
        self.alternate.is_some()
    }

    /// Get a reference to this line's alternate state, creating it and dirtying the line if
    /// necessary.
    fn get_alternate(&mut self) -> &LineState {
        if self.alternate.is_none() {
            self.alternate = Some(self.current.clone());
        }

        &self.alternate.as_ref().unwrap()
    }

    /// Get a mutable reference to this line's alternate state, creating it and dirtying the line
    /// if necessary.
    fn get_alternate_mut(&mut self) -> &mut LineState {
        if self.alternate.is_none() {
            self.alternate = Some(self.current.clone());
        }

        self.alternate.as_mut().unwrap()
    }

    /// Swaps this line's alternate state out for its current and returns the previous-current
    /// state. This line must have an alternate for this to be called.
    fn swap_alternate(&mut self) -> LineState {
        let mut alternate = self.alternate.take().unwrap();
        swap(&mut self.current, &mut alternate);
        alternate
    }
}

/// The line's segment ordering and, if rendered, layout.
#[derive(Clone, Debug)]
struct LineState {
    /// Segment ordering. Note that these IDs must also be present in the `Line`'s `segments` map.
    segments: Vec<SegmentId>,

    /// This line's layout on screen, if it has been rendered.
    layout: Option<LineLayout>,
}

impl Default for LineState {
    /// An empty state with no segments or layout.
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            layout: None,
        }
    }
}
