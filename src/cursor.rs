use crate::update::UpdateStep;
use crate::interface::InterfaceState;

#[derive(Copy, Clone)]
pub struct CursorPosition(pub u16, pub u16);

pub(crate) struct UpdateCursorStep {
    pub(crate) new_cursor: CursorPosition,
}

impl UpdateStep for UpdateCursorStep {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {
        state.cursor = self.new_cursor;
    }
}
