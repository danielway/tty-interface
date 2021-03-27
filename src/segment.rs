use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;

pub struct Segment {
    text: String,
    // TODO: color
    // TODO: style
}

struct SegmentUpdate {
    line_index: usize,
    segment_index: usize,
    segment: Option<Segment>,
}

impl UpdateStep for SegmentUpdate {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {}
}
