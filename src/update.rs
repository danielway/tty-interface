use crate::cursor::{CursorPosition, UpdateCursorStep};
use crate::line::Line;
use crate::segment::Segment;
use crate::interface::InterfaceState;

pub(crate) trait UpdateStep {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition);
}

pub struct UpdateBatch {
    pub(crate) steps: Vec<Box<dyn UpdateStep>>,
}

impl UpdateBatch {
    fn set_cursor(&mut self, new_cursor: CursorPosition) {
        self.steps.push(Box::new(UpdateCursorStep { new_cursor }));
    }

    fn set_line(&self, line_index: usize, line: Line) {}

    fn delete_line(&self, line_index: usize) {}

    fn set_segment(&self, line_index: usize, segment_index: usize, segment: Segment) {}

    fn delete_segment(&self, line_index: usize, segment_index: usize) {}
}
