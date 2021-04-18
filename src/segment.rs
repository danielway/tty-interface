//! Describes a particular segment within a line of the interface.

use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::TTYInterface;
use crate::utility::{clear_rest_of_line, move_cursor_exact, render_segment};
use crate::result::{Result, TTYError};

/// Contains ANSI control sequences for formatting a segment's text.
#[derive(Clone)]
pub struct SegmentFormatting {
    /// Applied immediately before the segment's text.
    pub(crate) pre: String,
    /// Applied immediately after the segment's text.
    pub(crate) post: String,
}

impl SegmentFormatting {
    /// Create new segment formatting with the specified control sequences.
    pub fn new(pre: String, post: String) -> SegmentFormatting {
        SegmentFormatting { pre, post }
    }
}

/// Represents a segment of text within a line of the interface.
pub struct Segment {
    /// The segment's displayed text.
    pub(crate) text: String,
    /// Optionally, any formatting control sequences.
    pub(crate) format: Option<SegmentFormatting>,
}

impl Segment {
    /// Create a segment with the specified text.
    pub fn new(text: String) -> Segment {
        Segment { text, format: None }
    }

    /// Create a segment with the specified text and formatting.
    pub fn new_formatted(text: String, format: SegmentFormatting) -> Segment {
        Segment { text, format: Some(format) }
    }
}

/// Describes a staged segment update.
pub(crate) struct SetSegmentStep {
    pub(crate) line_index: usize,
    pub(crate) segment_index: usize,
    pub(crate) segment: Option<Segment>,
}

impl UpdateStep for SetSegmentStep {
    fn do_update(&mut self, interface: &mut TTYInterface, update_cursor: &mut CursorPosition) -> Result<()> {
        if self.line_index > interface.state.lines.len() - 1 {
            return Err(TTYError::LineOutOfBounds);
        }

        if self.segment_index > interface.state.lines[self.line_index].segments.len() {
            return Err(TTYError::SegmentOutOfBounds);
        }

        // Determine if the updated segment has a different text length
        let diff_length = interface.state.lines[self.line_index].segments[self.segment_index].text.len()
            != self.segment.as_ref().unwrap().text.len();

        // Update the interface state
        interface.state.lines[self.line_index].segments[self.segment_index] = self.segment.take().unwrap();

        // Handle rendering the segment
        let segment_start = interface.state.lines[self.line_index].get_segment_start(self.segment_index);
        move_cursor_exact(interface.writer, update_cursor, segment_start, self.line_index as u16)?;
        render_segment(interface.writer, update_cursor, &interface.state.lines[self.line_index].segments[self.segment_index])?;

        // If the segment's length differed, we need to re-render segments to the right of this one
        if diff_length {
            clear_rest_of_line(interface.writer)?;
            for segment in &interface.state.lines[self.line_index].segments[self.segment_index+1..] {
                render_segment(interface.writer, update_cursor, segment)?;
            }
        }

        Ok(())
    }
}

/// Describes a staged segment deletion
pub(crate) struct DeleteSegmentStep {
    pub(crate) line_index: usize,
    pub(crate) segment_index: usize,
}

impl UpdateStep for DeleteSegmentStep {
    fn do_update(&mut self, interface: &mut TTYInterface, update_cursor: &mut CursorPosition) -> Result<()> {
        if self.line_index > interface.state.lines.len() - 1 {
            return Err(TTYError::LineOutOfBounds);
        }

        if self.segment_index > interface.state.lines[self.line_index].segments.len() - 1 {
            return Err(TTYError::SegmentOutOfBounds);
        }

        // Identify the start of this segment and clear to the right
        let segment_start = interface.state.lines[self.line_index].get_segment_start(self.segment_index);
        move_cursor_exact(interface.writer, update_cursor, segment_start, self.line_index as u16)?;
        clear_rest_of_line(interface.writer)?;

        // Update the interface's state by removing this segment
        interface.state.lines[self.line_index].segments.remove(self.segment_index);

        // Re-render segments to the right of this one
        for segment in &interface.state.lines[self.line_index].segments[self.segment_index..] {
            render_segment(interface.writer, update_cursor, segment)?;
        }

        Ok(())
    }
}
