use crate::format::{Color, Style};
use crate::layout::{PartLayout, SegmentLayout};
use crate::position::AbsolutePosition;
use crate::terminal::Terminal;
use crate::text::{TextParameters, TextParametersWithLayout};
use crate::Result;
use std::mem::swap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A unique segment identifier.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct SegmentId(usize);

/// The greatest provisioned segment identifier.
static ID_VALUE: AtomicUsize = AtomicUsize::new(0);

impl SegmentId {
    /// Create a new, unique segment identifier.
    fn new() -> Self {
        Self(ID_VALUE.fetch_add(1, Ordering::Relaxed))
    }
}

/// A segment of text which may have formatting. Accumulates changes until applied by its parent
/// `Line`. May be obtained from the `Line` API.
#[derive(Debug)]
pub struct Segment {
    /// This segment's immutable, unique identifier.
    identifier: SegmentId,

    /// The currently-rendered state.
    current: SegmentState,

    /// If changed, the state with queued and unapplied changes.
    alternate: Option<SegmentState>,
}

impl Segment {
    /// Create a new segment with no content or formatting.
    pub(crate) fn new() -> Segment {
        Segment {
            identifier: SegmentId::new(),
            current: SegmentState::default(),
            alternate: None,
        }
    }

    /// Create an exact clone of this segment and its states.
    pub(crate) fn clone(&self) -> Segment {
        Segment {
            identifier: self.identifier.clone(),
            current: self.current.clone(),
            alternate: self.alternate.clone(),
        }
    }

    /// This segment's unique identifier.
    pub fn identifier(&self) -> SegmentId {
        self.identifier
    }

    /// This segment's text. Reflects the latest state, even if staged changes have not yet been
    /// applied to the interface.
    pub fn text(&self) -> Option<&String> {
        let latest_state = self.get_latest_state();
        latest_state.text.as_ref()
    }

    /// This segment's color. Reflects the latest state, even if staged changes have not yet been
    /// applied to the interface.
    pub fn color(&self) -> Option<&Color> {
        let latest_state = self.get_latest_state();
        latest_state.color.as_ref()
    }

    /// This segment's styles. Reflects the latest state, even if staged changes have not yet been
    /// applied to the interface.
    pub fn styles(&self) -> &Vec<Style> {
        let latest_state = self.get_latest_state();
        latest_state.styles.as_ref()
    }

    /// Update this segment's text. The new text will be staged until applied for this `Interface`.
    pub fn set_text(&mut self, text: &str) {
        let alternate = self.get_alternate_mut();
        alternate.text = Some(String::from(text));
    }

    /// Update this segment's color. The new color is staged until applied for this `Interface`.
    pub fn set_color(&mut self, color: Color) {
        let alternate = self.get_alternate_mut();
        alternate.color = Some(color);
    }

    /// Clear this segment's color. The reset is staged until applied for this `Interface`.
    pub fn reset_color(&mut self) {
        let alternate = self.get_alternate_mut();
        alternate.color = None;
    }

    /// Update this segment's styles. The new styles are staged until applied for this `Interface`.
    pub fn set_styles(&mut self, styles: Vec<Style>) {
        let alternate = self.get_alternate_mut();
        alternate.styles = styles;
    }

    /// Clear this segment's styles. The reset is staged until applied for this `Interface`.
    pub fn reset_styles(&mut self) {
        let alternate = self.get_alternate_mut();
        alternate.styles = Vec::new();
    }

    /// Whether this segment has any staged changes.
    pub fn has_changes(&self) -> bool {
        self.alternate.is_some()
    }

    /// Render this segment at the specified `location`, including any staged changes. Assumes any
    /// previous render is still valid and attempts to optimize updates. If `force_render`, the
    /// segment will be rendered without any optimization.
    pub(crate) fn update(
        &mut self,
        terminal: &mut Terminal,
        location: AbsolutePosition,
        force_render: bool,
    ) -> Result<SegmentLayout> {
        let mut previous_state = None;
        if self.alternate.is_some() {
            previous_state = Some(self.swap_alternate());
        }

        if force_render {
            previous_state = None;
        }

        let segment_layout = self.render(terminal, location, previous_state.as_ref())?;

        self.current.layout = Some(segment_layout.parts().clone());

        Ok(segment_layout)
    }

    /// Renders the specified state for this segment at `location`. If provided, renders the state
    /// diff'd against the previous state to minimize the amount of text being written.
    fn render(
        &self,
        terminal: &mut Terminal,
        location: AbsolutePosition,
        previous_state: Option<&SegmentState>,
    ) -> Result<SegmentLayout> {
        Ok(if let Some(parameters) = self.get_current_parameters() {
            let previous_parameters = self.get_previous_parameters(previous_state);
            let layouts =
                terminal.render_segment(location, parameters, previous_parameters.as_ref())?;
            SegmentLayout::new(self.identifier, layouts)
        } else {
            SegmentLayout::default()
        })
    }

    /// Retrieves parameters for the current state.
    fn get_current_parameters(&self) -> Option<TextParameters> {
        Some(TextParameters::new(
            self.current.text.as_ref()?,
            self.current.color,
            self.current.styles.clone(),
        ))
    }

    /// Given an optional state, extracts as many parameters as possible.
    fn get_previous_parameters<'a>(
        &self,
        previous_state: Option<&'a SegmentState>,
    ) -> Option<TextParametersWithLayout<'a>> {
        Some(TextParametersWithLayout::new(
            TextParameters::new(
                previous_state?.text.as_ref()?,
                previous_state?.color,
                previous_state?.styles.clone(),
            ),
            previous_state?.layout.as_ref()?.clone(),
        ))
    }

    /// Get a mutable reference to this segment's alternate state, creating it and dirtying the
    /// segment if necessary.
    fn get_alternate_mut(&mut self) -> &mut SegmentState {
        if self.alternate.is_none() {
            self.alternate = Some(self.current.clone());
        }

        self.alternate.as_mut().unwrap()
    }

    /// Get a reference to the most-recent state, whether current or staged alternate.
    fn get_latest_state(&self) -> &SegmentState {
        match self.alternate {
            Some(ref state) => state,
            None => &self.current,
        }
    }

    /// Swaps this segment's alternate state out for its current and returns the previous-current
    /// state. This segment must have an alternate for this to be called.
    fn swap_alternate(&mut self) -> SegmentState {
        let mut alternate = self.alternate.take().unwrap();
        swap(&mut self.current, &mut alternate);
        alternate
    }
}

/// The segment's text and formatting.
#[derive(Clone, Debug)]
struct SegmentState {
    /// This segment's text. If unspecified, the segment is not rendered.
    text: Option<String>,

    /// This segment's optional foreground color.
    color: Option<Color>,

    /// This segment's optional formats.
    styles: Vec<Style>,

    /// This segment's layout on screen, if it has been rendered.
    layout: Option<Vec<PartLayout>>,
}

impl Default for SegmentState {
    /// An empty state with no text, color, or styles.
    fn default() -> Self {
        Self {
            text: None,
            color: None,
            styles: Vec::new(),
            layout: None,
        }
    }
}
