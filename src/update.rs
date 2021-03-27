use crate::cursor::{CursorPosition, UpdateCursorStep};
use crate::line::{Line, DeleteLineStep, SetLineStep};
use crate::segment::{Segment, SetSegmentStep, DeleteSegmentStep};
use crate::interface::InterfaceState;
use termion::raw::RawTerminal;
use std::io::Stdout;

pub(crate) trait UpdateStep {
    fn do_update(&mut self, stdout: &mut RawTerminal<Stdout>, state: &mut InterfaceState,
                 update_cursor: &mut CursorPosition);
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

    fn set_segment(&mut self, line_index: usize, segment_index: usize, segment: Segment) {
        self.steps.push(
            Box::new(SetSegmentStep { line_index, segment_index, segment: Some(segment) })
        );
    }

    fn delete_segment(&mut self, line_index: usize, segment_index: usize) {
        self.steps.push(
            Box::new(DeleteSegmentStep { line_index, segment_index })
        );
    }
}
