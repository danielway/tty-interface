use crate::result::Result;
use std::env;
use std::fmt::{Debug, Formatter};
use termion::cursor::DetectCursorPos;
use termion::terminal_size;

/// Provides low-level output capabilities using the stdout device.
pub struct Device<'a>(&'a mut dyn std::io::Write);

impl Device<'_> {
    /// Creates a new device for the specified stdout writer.
    pub(crate) fn new<'a>(writer: &'a mut dyn std::io::Write) -> Device<'a> {
        Device(writer)
    }

    /// The output device's viewport dimensions as (column, row).
    pub(crate) fn size(&self) -> Result<(u16, u16)> {
        Ok(terminal_size()?)
    }

    /// Writes the specified text to the terminal. If applicable, renders from the cursor position.
    pub(crate) fn write(&mut self, text: &str) -> Result<()> {
        Ok(self.0.write_all(text.as_ref())?)
    }

    /// Clear the entire terminal's contents.
    pub(crate) fn clear(&mut self) -> Result<()> {
        Ok(write!(self.0, "{}", termion::clear::All)?)
    }

    /// The cursor's current zero-indexed position (column, row).
    pub(crate) fn position(&mut self) -> Result<(u16, u16)> {
        let position = self.0.cursor_pos()?;
        Ok((position.0 - 1, position.1 - 1))
    }

    /// Sets whether the cursor position is visible or not.
    pub(crate) fn set_visible(&mut self, visible: bool) -> Result<()> {
        match visible {
            true => Ok(write!(self.0, "{}", termion::cursor::Show)?),
            false => Ok(write!(self.0, "{}", termion::cursor::Hide)?),
        }
    }

    /// Move the cursor to zero-indexed (column, row).
    pub(crate) fn goto(&mut self, column: u16, row: u16) -> Result<()> {
        Ok(write!(
            self.0,
            "{}",
            termion::cursor::Goto(column + 1, row + 1)
        )?)
    }

    /// Move the cursor up a specified number of rows.
    pub(crate) fn move_up(&mut self, rows: u16) -> Result<()> {
        Ok(write!(self.0, "{}", termion::cursor::Up(rows))?)
    }

    /// Move the cursor down a specified number of rows.
    pub(crate) fn move_down(&mut self, rows: u16) -> Result<()> {
        Ok(write!(self.0, "{}", "\n".repeat(rows as usize))?)
    }

    /// Move the cursor left a specified number of columns.
    pub(crate) fn move_left(&mut self, columns: u16) -> Result<()> {
        Ok(write!(self.0, "{}", termion::cursor::Left(columns))?)
    }

    /// Move the cursor right a specified number of columns.
    pub(crate) fn move_right(&mut self, columns: u16) -> Result<()> {
        Ok(write!(self.0, "{}", termion::cursor::Right(columns))?)
    }

    /// Save the cursor's position to be restored later.
    pub(crate) fn save_position(&mut self) -> Result<()> {
        if is_terminal_app() {
            Ok(write!(self.0, "\\u001B[7")?)
        } else {
            Ok(write!(self.0, "{}", termion::cursor::Save)?)
        }
    }

    /// Restore the cursor's previously-saved position.
    pub(crate) fn restore_position(&mut self) -> Result<()> {
        if is_terminal_app() {
            Ok(write!(self.0, "\\u001B[8")?)
        } else {
            Ok(write!(self.0, "{}", termion::cursor::Restore)?)
        }
    }

    /// Clears the current line's contents.
    pub(crate) fn clear_line(&mut self) -> Result<()> {
        Ok(write!(self.0, "{}", termion::clear::CurrentLine)?)
    }

    /// Clears contents from the cursor position until the next line.
    pub(crate) fn clear_until_newline(&mut self) -> Result<()> {
        Ok(write!(self.0, "{}", termion::clear::UntilNewline)?)
    }

    /// If the device supports buffering, flushes all pending commands.
    pub(crate) fn flush(&mut self) -> Result<()> {
        Ok(self.0.flush()?)
    }
}

impl Debug for Device<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Stdout device with writer")
    }
}

/// Whether the current terminal is the Mac OS Terminal.app.
pub(crate) fn is_terminal_app() -> bool {
    env::var("TERM_PROGRAM").unwrap_or_default() == "Apple_Terminal"
}
