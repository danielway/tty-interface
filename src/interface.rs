//! Provides simple TTY-based interface capabilities including partial re-renders of multi-line
//! displays. Includes a data structure and API to abstract low-level terminal manipulation.

use std::io;

use crate::cursor::CursorPosition;
use crate::line::Line;
use crate::update::UpdateBatch;
use crate::utility::move_cursor_to;
use crate::result::Result;

/// Contains interface and terminal state accumulated from previous update batches.
pub(crate) struct InterfaceState {
    /// The cursor's current intended position.
    pub(crate) cursor: CursorPosition,
    /// The interface's state contained within a list of lines.
    pub(crate) lines: Vec<Line>,
}

/// Provides terminal user interface capabilities.
pub struct TTYInterface<'a> {
    /// The writer to use for pushing interface changes.
    pub(crate) writer: &'a mut dyn io::Write,
    /// The interface's current state.
    pub(crate) state: InterfaceState,
}

impl TTYInterface<'_> {
    /// Create a new interface against the given writer.
    pub fn new(writer: &mut dyn io::Write) -> TTYInterface {
        TTYInterface {
            writer,
            state: InterfaceState {
                lines: Vec::new(),
                cursor: CursorPosition::init(0, 0)
            }
        }
    }

    /// Begins a new update batch for staging changes to the interface.
    pub fn start_update(&self) -> UpdateBatch {
        UpdateBatch { steps: Vec::new() }
    }

    /// Applies a given update batch to the interface by pushing changes to the terminal.
    pub fn perform_update(&mut self, batch: UpdateBatch) -> Result<()> {
        // Tracks cursor throughout update steps
        let mut update_cursor = self.state.cursor;

        // Apply update steps sequentially
        for mut step in batch.steps {
            step.do_update(self, &mut update_cursor)?;
        }

        // Return cursor from working position to state-specified position
        move_cursor_to(self.writer, &mut update_cursor, &self.state.cursor)?;

        self.writer.flush()?;

        Ok(())
    }

    /// Terminates this interface by restoring the cursor position for normal terminal use.
    pub fn end(&mut self) -> Result<()> {
        // Advance the cursor past interface content
        write!(self.writer, "{}", "\n".repeat(self.state.lines.len()))?;
        self.writer.flush()?;

        Ok(())
    }
}
