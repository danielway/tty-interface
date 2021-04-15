use crate::cursor::CursorPosition;
use crate::line::Line;
use crate::update::UpdateBatch;
use termion::raw::RawTerminal;
use std::io::{Write, StdoutLock};
use crate::utility::move_cursor_to;
use crate::result::Result;

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
                cursor: CursorPosition::init(0, 0)
            }
        }
    }

    pub fn start_update(&self) -> UpdateBatch {
        UpdateBatch { steps: Vec::new() }
    }

    pub fn perform_update(&mut self, stdout: &mut RawTerminal<StdoutLock>, batch: UpdateBatch) -> Result<()> {
        // Tracks cursor throughout update steps
        let mut update_cursor = self.state.cursor;

        // Apply update steps sequentially
        for mut step in batch.steps {
            step.do_update(stdout, &mut self.state, &mut update_cursor)?;
        }

        // Return cursor from working position to state-specified position
        move_cursor_to(stdout, &mut update_cursor, &self.state.cursor)?;

        stdout.flush()?;

        Ok(())
    }

    pub fn end(&self, stdout: &mut RawTerminal<StdoutLock>) -> Result<()> {
        // Advance the cursor past interface content
        write!(stdout, "{}", "\n".repeat(self.state.lines.len()))?;
        stdout.flush()?;

        Ok(())
    }
}
