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
    fn do_update(&mut self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {}
}

pub(crate) struct DeleteLineStep {
    pub(crate) line_index: usize,
}

impl UpdateStep for DeleteLineStep {
    fn do_update(&mut self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {}
}
