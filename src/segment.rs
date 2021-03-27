use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;
use crate::line::Line;

pub struct Segment {
    pub text: String,
    // TODO: color
    // TODO: style
}

pub(crate) struct SetSegmentStep {
    pub(crate) line_index: usize,
    pub(crate) segment_index: usize,
    pub(crate) segment: Option<Segment>,
}

impl UpdateStep for SetSegmentStep {
    fn do_update(&mut self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {
        if self.line_index > state.lines.len() - 1 {
            // TODO: throw error, line not added/set
        }

        if self.segment_index > state.lines[self.line_index].segments.len() {
            // TODO: throw error, leaves gap in segments
        }

        state.lines[self.line_index].segments[self.segment_index] = self.segment.take().unwrap();
        // TODO: rerender this segment
        // TODO: if segment length differed, rerender segments after
    }
}

pub(crate) struct DeleteSegmentStep {
    pub(crate) line_index: usize,
    pub(crate) segment_index: usize,
}

impl UpdateStep for DeleteSegmentStep {
    fn do_update(&mut self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {
        if self.line_index > state.lines.len() - 1 {
            // TODO: throw error, line not added/set
        }

        if self.segment_index > state.lines[self.line_index].segments.len() - 1 {
            // TODO: throw error, segment doesn't exist
        }
    }
}
