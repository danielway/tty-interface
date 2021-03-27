use crate::update::UpdateStep;
use crate::interface::InterfaceState;

pub struct CursorPosition(pub u16, pub u16);

struct CursorUpdate {
    new_cursor: CursorPosition,
}

impl UpdateStep for CursorUpdate {
    fn do_update(&self, state: &mut InterfaceState, update_cursor: &mut CursorPosition) {}
}
