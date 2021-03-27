use crate::cursor::CursorPosition;
use termion::raw::RawTerminal;
use std::io::{Stdout, Cursor};
use termion::cursor;
use crate::segment::Segment;
use crate::line::Line;

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

pub(crate) fn clear_line() {
    print!("{}", termion::clear::CurrentLine);
}

pub(crate) fn render_line(cursor: &mut CursorPosition, line: &Line) {
    for segment in line.segments {
        render_segment(cursor, &segment);
    }
}

pub(crate) fn render_segment(cursor: &mut CursorPosition, segment: &Segment) {
    // TODO: add color and style
    print!("{}", segment.text);
    cursor.x += segment.text.len();
}
