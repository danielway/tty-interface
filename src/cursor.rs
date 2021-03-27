use crate::update::UpdateStep;
use crate::interface::InterfaceState;

#[derive(Copy, Clone)]
pub struct CursorPosition(pub u16, pub u16);

pub(crate) struct CursorUpdate {
    pub(crate) new_cursor: CursorPosition,
}

impl UpdateStep for CursorUpdate {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {
        state.cursor = self.new_cursor;
    }
}
