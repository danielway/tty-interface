//! Contains interface update batching/staging structures and API.

use crate::cursor::{CursorPosition, UpdateCursorStep};
use crate::line::{Line, DeleteLineStep, SetLineStep};
use crate::segment::{Segment, SetSegmentStep, DeleteSegmentStep};
use crate::interface::TTYInterface;
use crate::result::Result;

/// Represents a particular step/change to be applied as part of an update batch.
pub(crate) trait UpdateStep {
    fn do_update(&mut self, interface: &mut TTYInterface, update_cursor: &mut CursorPosition) -> Result<()>;
}

/// A batch of staged changes to be applied by the user.
pub struct UpdateBatch {
    pub(crate) steps: Vec<Box<dyn UpdateStep>>,
}

impl UpdateBatch {
    /// Sets the cursor position, relative to the interface.
    pub fn set_cursor(&mut self, new_cursor: CursorPosition) {
        self.steps.push(
            Box::new(UpdateCursorStep { new_cursor })
        );
    }

    /// Sets or inserts a line in the interface.
    pub fn set_line(&mut self, line_index: usize, line: Line) {
        self.steps.push(
            Box::new(SetLineStep { line_index, line: Some(line) })
        );
    }

    /// Deletes a line from the interface.
    pub fn delete_line(&mut self, line_index: usize) {
        self.steps.push(
            Box::new(DeleteLineStep { line_index })
        );
    }

    /// Sets or inserts a segment into the interface.
    pub fn set_segment(&mut self, line_index: usize, segment_index: usize, segment: Segment) {
        self.steps.push(
            Box::new(SetSegmentStep { line_index, segment_index, segment: Some(segment) })
        );
    }

    /// Deletes a segment from the specified line of the interface.
    pub fn delete_segment(&mut self, line_index: usize, segment_index: usize) {
        self.steps.push(
            Box::new(DeleteSegmentStep { line_index, segment_index })
        );
    }
}
