//! Management of terminal cursor position within the interface.

use crate::update::UpdateStep;
use crate::interface::TTYInterface;
use crate::result::Result;

/// A cursor position relative to where this interface was initialized (e.g. 0,0 describes the
/// origin point for this interface rather than the absolute start of the terminal window).
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CursorPosition {
    /// Horizontal position, interface-relative, 0-indexed.
    pub x: u16,
    /// Vertical position, interface-relative, 0-indexed.
    pub y: u16,
}

impl CursorPosition {
    /// Create a new cursor position, interface-relative, 0-indexed.
    pub(crate) fn init(x: u16, y: u16) -> CursorPosition {
        CursorPosition { x, y }
    }
}

/// Describes a cursor location movement. Note that this indicates a desired final cursor location,
/// rather than an incidental cursor movement as part of applying an update.
pub(crate) struct UpdateCursorStep {
    pub(crate) new_cursor: CursorPosition,
}

impl UpdateStep for UpdateCursorStep {
    fn do_update(&mut self, interface: &mut TTYInterface, _update_cursor: &mut CursorPosition) -> Result<()> {
        interface.state.cursor = self.new_cursor;

        Ok(())
    }
}
