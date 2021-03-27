use crate::cursor::CursorPosition;
use crate::line::Line;
use crate::update::UpdateBatch;

pub(crate) struct InterfaceState {
    cursor: CursorPosition,
    lines: Vec<Line>,
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

    pub fn perform_update(&self, batch: UpdateBatch) {}
}
