use crate::cursor::CursorPosition;
use termion::raw::RawTerminal;
use std::io::{Stdout, Cursor};
use termion::cursor;
use crate::segment::Segment;
use crate::line::Line;

/// Moves the cursor to `to` and updates `cursor` with the new position.
pub(crate) fn move_cursor_to(cursor: &mut CursorPosition, to: &CursorPosition) {
    move_cursor_exact(cursor, to.x, to.y);
}

/// Moves the cursor to the specified coordinates and updates `cursor` accordingly.
pub(crate) fn move_cursor_exact(cursor: &mut CursorPosition, x: u16, y: u16) {
    move_cursor_by(cursor, (cursor.x - x) as i16, (cursor.y - y) as i16);
}

/// Moves the cursor by a specified diff and updates `cursor`.
pub(crate) fn move_cursor_by(cursor: &mut CursorPosition, diff_x: i16, diff_y: i16) {
    if diff_x < 0 && diff_x as u16 > cursor.x {
        // TODO: throw error, invalid (negative) new cursor position
    }
    if diff_y < 0 && diff_y as u16 > cursor.y {
        // TODO: throw error, invalid (negative) new cursor position
    }

    if diff_x > 0 {
        print!("{}", cursor::Right(diff_x as u16));
        cursor.x += diff_x as u16;
    } else if diff_x < 0 {
        print!("{}", cursor::Left(diff_x.abs() as u16));
        cursor.x -= diff_x.abs() as u16;
    }

    if diff_y > 0 {
        print!("{}", cursor::Down(diff_y as u16));
        cursor.y += diff_y as u16;
    } else if diff_y < 0 {
        print!("{}", cursor::Up(diff_y.abs() as u16));
        cursor.y -= diff_y.abs() as u16;
    }
}

/// Clears the contents of the cursor's current line. Does not move the cursor.
pub(crate) fn clear_line() {
    print!("{}", termion::clear::CurrentLine);
}

/// Clears from the current cursor position to the end of the line.
pub(crate) fn clear_rest_of_line() {
    print!("{}", termion::clear::UntilNewline)
}

/// Clears the terminal line, renders the `line`, and updates the `cursor` position according to the
/// `line`'s total length. Will move the `cursor` to `x=0` if `>0`.
pub(crate) fn render_line(cursor: &mut CursorPosition, line: &Line) {
    if cursor.x != 0 {
        move_cursor_exact(cursor, 0, cursor.y);
    }

    clear_line();
    for segment in &line.segments {
        render_segment(cursor, &segment);
    }
}

/// Renders the `segment` at `cursor` and advances `cursor` with `segment.text`'s length.
pub(crate) fn render_segment(cursor: &mut CursorPosition, segment: &Segment) {
    // TODO: add color and style
    print!("{}", segment.text);
    cursor.x += segment.text.len() as u16;
}
