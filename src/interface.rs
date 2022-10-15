use std::{io::{Stdout, stdout, Write}, mem::swap};

use crossterm::{terminal, queue, cursor, style};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Position, Vector, Result, State};

/// A TTY-based user-interface providing optimized update rendering.
pub struct Interface {
    stdout: Stdout,
    size: Vector,
    current: State,
    alternate: Option<State>,
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

        let mut interface = Interface {
            stdout: stdout(),
            size: Vector::new(columns, lines),
            current: State::new(),
            alternate: None,
        };

        terminal::enable_raw_mode()?;
        queue!(interface.stdout, terminal::Clear(terminal::ClearType::All))?;
        queue!(interface.stdout, cursor::MoveTo(0, 0))?;

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
        let alternate = self.alternate.get_or_insert_with(|| self.current.clone());

        let mut line = position.y().into();
        let mut column = position.x().into();

        for grapheme in text.graphemes(true) {
            if column > self.size.x().into() {
                column = 0;
                line += 1;
            }

            alternate.set(Position::new(column, line), grapheme.to_string());

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
        if self.alternate.is_none() {
            return Ok(())
        }

        let mut alternate = self.alternate.take().unwrap();
        swap(&mut self.current, &mut alternate);

        for (position, text) in self.current.dirty_iter() {
            queue!(self.stdout, cursor::MoveTo(position.x(), position.y()))?;
            queue!(self.stdout, style::Print(text))?;
        }

        self.stdout.flush()?;

        self.current.clear_dirty();
        
        Ok(())
    }
}