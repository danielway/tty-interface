use crate::cursor::CursorPosition;
use termion::raw::RawTerminal;
use std::io::{Stdout, Cursor};
use termion::cursor;
use crate::segment::Segment;
use crate::line::Line;

/// Moves the cursor to `to` and updates `cursor` with the new position.
pub(crate) fn move_cursor(cursor: &mut CursorPosition, to: &CursorPosition) {
    // Move cursor vertically
    if from.y < to.y {
        print!("{}", cursor::Down(to.y - from.y));
    } else if from.y > to.y {
        print!("{}", cursor::Up(from.y - to.y));
    }

    // Move cursor horizontally
    if from.x < to.x {
        print!("{}", cursor::Right(to.x - from.x));
    } else if from.x > to.x {
        print!("{}", cursor::Left(from.x - to.x));
    }

    cursor.y = to.y;
    cursor.x = to.x;
}

/// Clears the contents of the cursor's current line. Does not move the cursor.
pub(crate) fn clear_line() {
    print!("{}", termion::clear::CurrentLine);
}

/// Clears the terminal line, renders the `line`, and updates the `cursor` position according to the
/// `line`'s total length. Will move the `cursor` to `x=0` if `>0`.
pub(crate) fn render_line(cursor: &mut CursorPosition, line: &Line) {
    if cursor.x != 0 {
        move_cursor(cursor, &CursorPosition::init(0, cursor.y));
    }

    clear_line();
    for segment in line.segments {
        render_segment(cursor, &segment);
    }
}

/// Renders the `segment` at `cursor` and advances `cursor` with `segment.text`'s length.
pub(crate) fn render_segment(cursor: &mut CursorPosition, segment: &Segment) {
    // TODO: add color and style
    print!("{}", segment.text);
    cursor.x += segment.text.len();
}
