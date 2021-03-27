use crate::update::UpdateStep;
use crate::interface::InterfaceState;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CursorPosition {
    pub x: u16,
    pub y: u16,
}

impl CursorPosition {
    pub(crate) fn init(x: u16, y: u16) -> CursorPosition {
        CursorPosition { x, y }
    }
}

pub(crate) struct UpdateCursorStep {
    pub(crate) new_cursor: CursorPosition,
}

impl UpdateStep for UpdateCursorStep {
    fn do_update(&mut self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {
        state.cursor = self.new_cursor;
    }
}
