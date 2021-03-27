use crate::segment::Segment;
use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;

pub struct Line {
    segments: Option<Vec<Segment>>,
}

struct SetLineStep {
    line_index: usize,
    line: Option<Line>,
}

impl UpdateStep for SetLineStep {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {}
}

struct DeleteLineStep {
    line_index: usize,
}

impl UpdateStep for DeleteLineStep {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {}
}
