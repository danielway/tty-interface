use crate::segment::Segment;
use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;
use crate::utility::{move_cursor_exact, render_line, move_cursor_by, clear_line};
use termion::raw::RawTerminal;
use std::io::StdoutLock;
use crate::result::{Result, TTYError};

pub struct Line {
    pub segments: Vec<Segment>,
}

impl Line {
    /// Returns the character position for the specified segment within the line.
    pub(crate) fn get_segment_start(&self, segment_index: usize) -> u16 {
        let mut segment_start = 0;
        for i in 0..segment_index {
            segment_start += self.segments[i].text.len();
        }
        segment_start as u16
    }
}

pub(crate) struct SetLineStep {
    pub(crate) line_index: usize,
    pub(crate) line: Option<Line>,
}

impl UpdateStep for SetLineStep {
    fn do_update(&mut self, stdout: &mut RawTerminal<StdoutLock>, state: &mut InterfaceState,
                 update_cursor: &mut CursorPosition) -> Result<()> {
        if self.line_index > state.lines.len() {
            return Err(TTYError::LineOutOfBounds);
        }

        let line = self.line.take().expect("SetLineStep is missing a Line");
        if self.line_index == state.lines.len() {
            state.lines.push(line);
        } else {
            state.lines[self.line_index] = line;
        }

        // Render appended/updated line
        move_cursor_exact(stdout, update_cursor, 0, self.line_index as u16)?;
        render_line(stdout, update_cursor, &state.lines[self.line_index])?;

        Ok(())
    }
}

pub(crate) struct DeleteLineStep {
    pub(crate) line_index: usize,
}

impl UpdateStep for DeleteLineStep {
    fn do_update(&mut self, stdout: &mut RawTerminal<StdoutLock>, state: &mut InterfaceState,
                 update_cursor: &mut CursorPosition) -> Result<()> {
        if self.line_index > state.lines.len() - 1 {
            return Err(TTYError::LineOutOfBounds);
        }

        // If the cursor isn't on this line, move it here
        let line_y: u16 = self.line_index as u16;
        if update_cursor.y != line_y {
            move_cursor_exact(stdout, update_cursor, 0, line_y)?;
        }

        // Shift lines >line_index down and render them; clear last (now shifted up) line
        state.lines.remove(self.line_index);
        for i in self.line_index..state.lines.len() {
            render_line(stdout, update_cursor, &state.lines[i])?;
            move_cursor_by(stdout, update_cursor, 0, 1)?;
        }
        clear_line(stdout)?;

        Ok(())
    }
}
