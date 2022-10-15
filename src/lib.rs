//! # tty-interface
//! 
//! Provides simple TTY-based user interface capabilities including partial re-renders of multi-line displays.
//! 

use std::io::{Stdout, stdout};

pub struct Interface {
    stdout: Stdout,
}

impl Interface {
    /// Create a new interface for stdout.
    /// 
    /// # Examples
    /// ```
    /// let interface = tty_interface::Interface::new();
    /// ```
    pub fn new() -> Interface {
        Interface {
            stdout: stdout(),
        }
    }
}

pub struct Buffer;

pub struct Cell;

/// A coordinate position in the terminal. May be absolute or relative to some buffer's origin.
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    /// Create a new, immutable position.
    /// 
    /// # Examples
    /// ```
    /// let origin = tty_interface::Position::new(2, 4);
    /// assert_eq!(origin.x(), 2);
    /// assert_eq!(origin.y(), 4);
    /// ```
    pub fn new(x: u16, y: u16) -> Position {
        Position { x, y }
    }

    /// This position's column value.
    pub fn x(&self) -> u16 {
        self.x
    }

    /// This position's line value.
    pub fn y(&self) -> u16 {
        self.y
    }
}
