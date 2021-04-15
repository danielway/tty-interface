use crate::update::UpdateStep;
use crate::interface::InterfaceState;
use termion::raw::RawTerminal;
use std::io::StdoutLock;
use crate::result::Result;

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
    fn do_update(&mut self, _stdout: &mut RawTerminal<StdoutLock>, state: &mut InterfaceState,
                 _update_cursor: &mut CursorPosition) -> Result<()> {
        state.cursor = self.new_cursor;

        Ok(())
    }
}
