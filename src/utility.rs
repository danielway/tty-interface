//! Contains low-level functions for navigating and interacting-with the TTY terminal.

use std::io;
use termion::cursor;

use crate::cursor::CursorPosition;
use crate::segment::Segment;
use crate::line::Line;
use crate::result::{Result, TTYError};

/// Moves the cursor to `to` and updates `cursor` with the new position.
pub(crate) fn move_cursor_to(writer: &mut dyn io::Write, cursor: &mut CursorPosition,
                             to: &CursorPosition) -> Result<()> {
    move_cursor_exact(writer, cursor, to.x, to.y)?;

    Ok(())
}

/// Moves the cursor to the specified coordinates and updates `cursor` accordingly.
pub(crate) fn move_cursor_exact(writer: &mut dyn io::Write, cursor: &mut CursorPosition,
                                x: u16, y: u16) -> Result<()> {
    let diff_x: i16 = (x as i16) - (cursor.x as i16);
    let diff_y: i16 = (y as i16) - (cursor.y as i16);

    move_cursor_by(writer, cursor, diff_x, diff_y)?;

    Ok(())
}

/// Moves the cursor by a specified diff and updates `cursor`.
pub(crate) fn move_cursor_by(writer: &mut dyn io::Write, cursor: &mut CursorPosition,
                             diff_x: i16, diff_y: i16) -> Result<()> {
    if (diff_x < 0 && diff_x > cursor.x as i16) || (diff_y < 0 && diff_y > cursor.y as i16) {
        return Err(TTYError::CursorOutOfBounds);
    }

    if diff_x > 0 {
        write!(writer, "{}", cursor::Right(diff_x as u16))?;
        cursor.x += diff_x as u16;
    } else if diff_x < 0 {
        write!(writer, "{}", cursor::Left(diff_x.abs() as u16))?;
        cursor.x -= diff_x.abs() as u16;
    }

    if diff_y > 0 {
        write!(writer, "{}", "\n".repeat(diff_y as usize))?;
        cursor.y += diff_y as u16;
    } else if diff_y < 0 {
        write!(writer, "{}", cursor::Up(diff_y.abs() as u16))?;
        cursor.y -= diff_y.abs() as u16;
    }

    Ok(())
}

/// Clears the contents of the cursor's current line. Does not move the cursor.
pub(crate) fn clear_line(writer: &mut dyn io::Write) -> Result<()> {
    write!(writer, "{}", termion::clear::CurrentLine)?;
    Ok(())
}

/// Clears from the current cursor position to the end of the line.
pub(crate) fn clear_rest_of_line(writer: &mut dyn io::Write) -> Result<()> {
    write!(writer, "{}", termion::clear::UntilNewline)?;
    Ok(())
}

/// Clears the terminal line, renders the `line`, and updates the `cursor` position according to the
/// `line`'s total length. Will move the `cursor` to `x=0` if `>0`.
pub(crate) fn render_line(writer: &mut dyn io::Write, cursor: &mut CursorPosition,
                          line: &Line) -> Result<()> {
    if cursor.x != 0 {
        move_cursor_exact(writer, cursor, 0, cursor.y)?;
    }

    clear_line(writer)?;

    for segment in &line.segments {
        render_segment(writer, cursor, &segment)?;
    }

    Ok(())
}

/// Renders the `segment` at `cursor` and advances `cursor` with `segment.text`'s length.
pub(crate) fn render_segment(writer: &mut dyn io::Write, cursor: &mut CursorPosition,
                             segment: &Segment) -> Result<()> {
    cursor.x += segment.text.len() as u16;

    match &segment.format {
        Some(format) => write!(writer, "{}{}{}", format.pre, segment.text, format.post)?,
        None => write!(writer, "{}", segment.text)?,
    }

    Ok(())
}
