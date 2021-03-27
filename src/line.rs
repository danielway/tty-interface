use crate::segment::Segment;
use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;

pub struct Line {
    segments: Option<Vec<Segment>>,
}

pub(crate) struct SetLineStep {
    pub(crate) line_index: usize,
    pub(crate) line: Option<Line>,
}

impl UpdateStep for SetLineStep {
    fn do_update(&mut self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {
        if state.lines.len() <= self.line_index + 1 {
            // TODO: throw error, there's a line gap, invalid state
        } else if state.lines.len() == self.line_index {
            state.lines.push(self.line.take().unwrap());
            // TODO: render new line
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
        state.lines.remove(self.line_index);
        for i in self.line_index..state.lines.len() {
            // TODO: render shifted line
        }
        // TODO: clear previous last line
    }
}
