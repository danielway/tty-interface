use crate::cursor::{CursorPosition, UpdateCursorStep};
use crate::line::{Line, DeleteLineStep, SetLineStep};
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
        self.steps.push(
            Box::new(UpdateCursorStep { new_cursor })
        );
    }

    fn set_line(&mut self, line_index: usize, line: Line) {
        self.steps.push(
            Box::new(SetLineStep { line_index, line: Some(line) })
        );
    }

    fn delete_line(&mut self, line_index: usize) {
        self.steps.push(
            Box::new(DeleteLineStep { line_index })
        );
    }

    fn set_segment(&self, line_index: usize, segment_index: usize, segment: Segment) {}

    fn delete_segment(&self, line_index: usize, segment_index: usize) {}
}
