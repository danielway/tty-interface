//! # tty-interface
//! 
//! Provides simple TTY-based user interface capabilities including partial re-renders of multi-line displays.
//! 

use std::{io::{Stdout, stdout, Write}, mem::swap};

use crossterm::{terminal, queue, cursor, style};
use unicode_segmentation::UnicodeSegmentation;

/// A TTY-based user-interface providing optimized update rendering.
pub struct Interface {
    stdout: Stdout,
    size: Vector,
    current_cells: Vec<Vec<String>>,
    alternate_cells: Option<Vec<Vec<String>>>,
}

impl Interface {
    /// Create a new interface for stdout.
    /// 
    /// # Examples
    /// ```
    /// let interface = tty_interface::Interface::new().unwrap();
    /// ```
    pub fn new() -> Result<Interface> {
        let (columns, lines) = terminal::size()?;
        
        let mut cells = Vec::new();
        for _ in 0..lines {
            cells.push(vec![String::new(); columns.into()]);
        }

        let interface = Interface {
            stdout: stdout(),
            size: Vector::new(columns, lines),
            current_cells: cells,
            alternate_cells: None,
        };

        Ok(interface)
    }

    /// Update the interface's text at the specified position. Changes are staged until applied.
    /// 
    /// # Examples
    /// ```
    /// let mut interface = tty_interface::Interface::new().unwrap();
    /// interface.set(tty_interface::Position::new(1, 1), "Hello, world!");
    /// ```
    pub fn set(&mut self, position: Position, text: &str) {
        let alternate_cells = self.alternate_cells.get_or_insert_with(|| self.current_cells.clone());

        let mut line: usize = position.y().into();
        let mut column: usize = position.x().into();

        for grapheme in text.graphemes(true) {
            if column > self.size.x().into() {
                column = 0;
                line += 1;
            }

            alternate_cells[line][column] = grapheme.to_string();

            column += 1;
        }
    }

    /// Applies staged changes to the terminal.
    /// 
    /// # Examples
    /// ```
    /// let mut interface = tty_interface::Interface::new().unwrap();
    /// interface.set(tty_interface::Position::new(1, 1), "Hello, world!");
    /// interface.apply().unwrap();
    /// ```
    pub fn apply(&mut self) -> Result<()> {
        if self.alternate_cells.is_none() {
            return Ok(())
        }

        let mut alternate = self.alternate_cells.take().unwrap();
        swap(&mut self.current_cells, &mut alternate);

        queue!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        for line in 0..self.size.y().into() {
            for column in 0..self.size.x().into() {
                let text = &self.current_cells[line][column];
                queue!(self.stdout, cursor::MoveTo(column as u16, line as u16))?;
                queue!(self.stdout, style::Print(text))?;
            }
        }

        self.stdout.flush()?;
        
        Ok(())
    }
}

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

/// A directional vector with no positional information.
pub struct Vector {
    x: u16,
    y: u16,
}

impl Vector {
    /// Create a new, immutable vector.
    /// 
    /// # Examples
    /// ```
    /// let size = tty_interface::Vector::new(7, 4);
    /// assert_eq!(size.x(), 7);
    /// assert_eq!(size.y(), 4);
    /// ```
    pub fn new(x: u16, y: u16) -> Vector {
        Vector { x, y }
    }

    /// This vector's column value.
    pub fn x(&self) -> u16 {
        self.x
    }
    
    /// This vector's line value.
    pub fn y(&self) -> u16 {
        self.y
    }
}

/// An interface operation's result containing either a successful value or error.
pub type Result<T> = std::result::Result<T, Error>;

/// A failed interface operation's error information.
#[derive(Debug)]
pub enum Error {
    /// A low-level terminal interaction error.
    Terminal(crossterm::ErrorKind),
}

impl From<crossterm::ErrorKind> for Error {
    fn from(err: crossterm::ErrorKind) -> Self {
        Error::Terminal(err)
    }
}
