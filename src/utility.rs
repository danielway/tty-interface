use crate::cursor::CursorPosition;
use termion::raw::RawTerminal;
use std::io::Stdout;
use termion::cursor;
use crate::segment::Segment;
use crate::line::Line;

pub(crate) fn move_cursor(from: CursorPosition, to: &CursorPosition) -> CursorPosition {
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

    // Return the new cursor position
    CursorPosition::init(to.x, to.y)
}

pub(crate) fn render_line(line: &Line, at: CursorPosition) -> CursorPosition {
    let mut cursor = at;
    for segment in line.segments {
        cursor = render_segment(&segment, cursor);
    }
    cursor
}

pub(crate) fn render_segment(segment: &Segment, at: CursorPosition) -> CursorPosition {
    // TODO: add color and style
    print!("{}", segment.text);

    // Return cursor advanced by segment length
    CursorPosition::init(at.x + segment.text.len(), at.y)
}
