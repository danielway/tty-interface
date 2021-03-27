use crate::segment::Segment;
use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;

pub struct Line {
    segments: Option<Vec<Segment>>,
}

struct LineUpdate {
    line_index: u16,
    line: Option<Line>,
}

impl UpdateStep for LineUpdate {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {}
}
