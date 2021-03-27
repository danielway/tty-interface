use crate::cursor::CursorPosition;
use termion::raw::RawTerminal;
use std::io::Stdout;
use termion::cursor;

pub(crate) fn move_cursor(from: &CursorPosition, to: &CursorPosition) {
    if from.y < to.y {
        print!("{}", cursor::Down(to.y - from.y));
    } else if from.y > to.y {
        print!("{}", cursor::Up(from.y - to.y));
    }

    if from.x < to.x {
        print!("{}", cursor::Right(to.x - from.x));
    } else if from.x > to.x {
        print!("{}", cursor::Left(from.x - to.x));
    }
}