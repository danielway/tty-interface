use std::{io::{Stdout, stdout, Write}, mem::swap};

use crossterm::{terminal, queue, cursor, style};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Position, Vector, Result};

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
    /// use tty_interface::Interface;
    /// 
    /// let interface = Interface::new().expect("terminal size should be available");
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
    /// use tty_interface::{Interface, Position};
    /// 
    /// let mut interface = Interface::new().expect("terminal size should be available");
    /// interface.set(Position::new(1, 1), "Hello, world!");
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
    /// use tty_interface::{Interface, Position};
    /// 
    /// let mut interface = Interface::new().expect("terminal size should be available");
    /// interface.set(Position::new(1, 1), "Hello, world!");
    /// interface.apply().expect("updates should be valid");
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