//! Describes a single line of the terminal user interface, which includes some number of segments.

use crate::segment::Segment;
use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::TTYInterface;
use crate::utility::{move_cursor_exact, render_line, move_cursor_by, clear_line};
use crate::result::{Result, TTYError};

/// A line in the terminal interface.
pub struct Line {
    pub(crate) segments: Vec<Segment>,
}

impl Line {
    /// Create a new line with the given list of segments.
    pub fn new(segments: Vec<Segment>) -> Line {
        Line { segments }
    }

    /// Returns the character position for the specified segment within the line.
    pub(crate) fn get_segment_start(&self, segment_index: usize) -> u16 {
        let mut segment_start = 0;
        for i in 0..segment_index {
            segment_start += self.segments[i].text.len();
        }
        segment_start as u16
    }
}

/// Describes a staged line change operation (either update or insert).
pub(crate) struct SetLineStep {
    pub(crate) line_index: usize,
    pub(crate) line: Option<Line>,
}

impl UpdateStep for SetLineStep {
    fn do_update(&mut self, interface: &mut TTYInterface, update_cursor: &mut CursorPosition) -> Result<()> {
        if self.line_index > interface.state.lines.len() {
            return Err(TTYError::LineOutOfBounds);
        }

        // Update or insert the line into the interface state
        let line = self.line.take().expect("SetLineStep is missing a Line");
        if self.line_index == interface.state.lines.len() {
            interface.state.lines.push(line);
        } else {
            interface.state.lines[self.line_index] = line;
        }

        // Render appended/updated line
        move_cursor_exact(interface.writer, update_cursor, 0, self.line_index as u16)?;
        render_line(interface.writer, update_cursor, &interface.state.lines[self.line_index])?;

        Ok(())
    }
}

/// Describes a staged line deletion.
pub(crate) struct DeleteLineStep {
    pub(crate) line_index: usize,
}

impl UpdateStep for DeleteLineStep {
    fn do_update(&mut self, interface: &mut TTYInterface, update_cursor: &mut CursorPosition) -> Result<()> {
        if self.line_index > interface.state.lines.len() - 1 {
            return Err(TTYError::LineOutOfBounds);
        }

        // If the cursor isn't on this line, move it here
        let line_y: u16 = self.line_index as u16;
        if update_cursor.y != line_y {
            move_cursor_exact(interface.writer, update_cursor, 0, line_y)?;
        }

        // Shift lines >line_index down and render them; clear last (now shifted up) line
        interface.state.lines.remove(self.line_index);
        for i in self.line_index..interface.state.lines.len() {
            render_line(interface.writer, update_cursor, &interface.state.lines[i])?;
            move_cursor_by(interface.writer, update_cursor, 0, 1)?;
        }
        clear_line(interface.writer)?;

        Ok(())
    }
}
