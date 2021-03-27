use crate::update::UpdateStep;
use crate::interface::InterfaceState;
use termion::raw::RawTerminal;
use std::io::Stdout;

#[derive(Copy, Clone)]
pub struct CursorPosition(pub u16, pub u16);

pub(crate) struct UpdateCursorStep {
    pub(crate) new_cursor: CursorPosition,
}

impl UpdateStep for UpdateCursorStep {
    fn do_update(&mut self, stdout: &mut RawTerminal<Stdout>, state: &mut InterfaceState,
                 update_cursor: &mut CursorPosition) {
        state.cursor = self.new_cursor;
    }
}
