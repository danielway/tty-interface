use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;

pub struct Segment {
    text: String,
    // TODO: color
    // TODO: style
}

struct SegmentUpdate {
    line_index: u16,
    segment_index: u16,
    segment: Option<Segment>,
}

impl UpdateStep for SegmentUpdate {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {}
}
