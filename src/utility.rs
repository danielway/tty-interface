use crate::cursor::CursorPosition;
use termion::raw::RawTerminal;
use std::io::Stdout;

pub(crate) fn move_cursor(stdout: &mut RawTerminal<Stdout>, from: &CursorPosition, to: &CursorPosition) {
    // TODO: implement
}