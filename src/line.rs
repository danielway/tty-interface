use crate::segment::Segment;
use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;
use crate::utility::{move_cursor_exact, render_line, move_cursor_by, clear_line};

pub struct Line {
    pub segments: Vec<Segment>,
}

pub(crate) struct SetLineStep {
    pub(crate) line_index: usize,
    pub(crate) line: Option<Line>,
}

impl UpdateStep for SetLineStep {
    fn do_update(&mut self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {
        if self.line_index > state.lines.len() {
            // TODO: throw error, there's a line gap, invalid state
        } else if self.line_index == state.lines.len() {
            state.lines.push(self.line.take().unwrap());
            move_cursor_exact(cursor, 0, self.line_index as u16);
            render_line(cursor, &state.lines.last().unwrap());
        } else {
            state.lines[self.line_index] = self.line.take().unwrap();
            // TODO: clear updated line
            // TODO: render updated line
        }
    }
}

pub(crate) struct DeleteLineStep {
    pub(crate) line_index: usize,
}

impl UpdateStep for DeleteLineStep {
    fn do_update(&mut self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {
        if self.line_index > state.lines.len() - 1 {
            // TODO: throw error, line doesn't exist
        }

        // If the cursor isn't on this line, move it here
        let line_y: u16 = self.line_index as u16;
        if update_cursor.y != line_y {
            move_cursor_exact(update_cursor, 0, line_y);
        }

        // Shift lines >line_index down and render them; clear last (now shifted up) line
        state.lines.remove(self.line_index);
        for i in self.line_index..state.lines.len() {
            render_line(cursor, &state.lines[i]);
            move_cursor_by(cursor, 0, 1);
        }
        clear_line();
    }
}
