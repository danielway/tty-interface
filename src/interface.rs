use crate::cursor::CursorPosition;
use crate::line::Line;
use crate::update::UpdateBatch;

pub(crate) struct InterfaceState {
    pub(crate) cursor: CursorPosition,
    pub(crate) lines: Vec<Line>,
}

pub struct TTYInterface {
    state: InterfaceState,
}

impl TTYInterface {
    pub fn new() -> TTYInterface {
        TTYInterface {
            state: InterfaceState {
                lines: Vec::new(),
                cursor: CursorPosition(0,0)
            }
        }
    }

    pub fn start_update(&self) -> UpdateBatch {
        UpdateBatch { steps: Vec::new() }
    }

    pub fn perform_update(&mut self, batch: UpdateBatch) {
        let mut update_cursor = self.state.cursor;
        for mut step in batch.steps {
            step.do_update(&mut self.state, &mut update_cursor);
        }
        // TODO: return from update_cursor to state.cursor
    }
}
