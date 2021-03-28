use crate::cursor::CursorPosition;
use crate::update::UpdateStep;
use crate::interface::InterfaceState;
use crate::line::Line;
use crate::utility::{clear_rest_of_line, move_cursor_exact, render_segment};

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

        let diff_length = state.lines[self.line_index].segments[self.segment_index].text.len()
            != self.segment.as_ref().unwrap().text.len();

        state.lines[self.line_index].segments[self.segment_index] = self.segment.take().unwrap();

        let segment_start = state.lines[self.line_index].get_segment_start(self.segment_index);
        move_cursor_exact(update_cursor, segment_start, self.line_index as u16);
        render_segment(update_cursor, &state.lines[self.line_index].segments[self.segment_index]);

        if diff_length {
            clear_rest_of_line();
            for segment in &state.lines[self.line_index].segments[self.segment_index+1..] {
                render_segment(update_cursor, segment);
            }
        }
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

        let segment_start = state.lines[self.line_index].get_segment_start(self.segment_index);
        move_cursor_exact(update_cursor, segment_start, self.line_index as u16);
        clear_rest_of_line();

        state.lines[self.line_index].segments.remove(self.segment_index);

        for segment in &state.lines[self.line_index].segments[self.segment_index..] {
            render_segment(update_cursor, segment);
        }
    }
}
