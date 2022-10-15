use std::{io::{Stdout, stdout, Write}, mem::swap};

use crossterm::{terminal, cursor, style, QueueableCommand};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Position, Vector, Result, State};

/// A TTY-based user-interface providing optimized update rendering.
pub struct Interface<'a> {
    writer: Option<&'a mut dyn Write>,
    stdout: Option<Stdout>,
    size: Vector,
    current: State,
    alternate: Option<State>,
}

impl Interface<'_> {
    /// Create a new interface for stdout.
    /// 
    /// # Examples
    /// ```
    /// use tty_interface::Interface;
    /// 
    /// let interface = Interface::for_stdout().expect("terminal size should be available");
    /// ```
    pub fn for_stdout<'a>() -> Result<Interface<'a>> {
        Self::new(None, Some(stdout()))
    }

    /// Create a new interface for the specified writer.
    /// 
    /// # Examples
    /// ```
    /// use tty_interface::Interface;
    /// use std::io::{Write, stdout};
    /// 
    /// let writer: &mut dyn Write = &mut stdout();
    /// let interface = Interface::for_writer(writer).expect("terminal size should be available");
    /// ```
    pub fn for_writer(writer: &mut dyn Write) -> Result<Interface> {
        Self::new(Some(writer), None)
    }

    /// Create a new interface with the specified writer or stdout device. Initializes the terminal.
    fn new(writer: Option<&mut dyn Write>, stdout: Option<Stdout>) -> Result<Interface> {
        let (columns, lines) = terminal::size()?;

        let mut interface = Interface {
            writer,
            stdout,
            size: Vector::new(columns, lines),
            current: State::new(),
            alternate: None,
        };

        terminal::enable_raw_mode()?;
        
        let writer = interface.writer();
        writer.queue(terminal::Clear(terminal::ClearType::All))?;
        writer.queue(cursor::MoveTo(0, 0))?;

        Ok(interface)
    }

    /// When finished using this interface, uninitialize its terminal configuration.
    pub fn exit(self) -> Result<()> {
        terminal::disable_raw_mode()?;
        println!();
        Ok(())
    }

    /// Update the interface's text at the specified position. Changes are staged until applied.
    /// 
    /// # Examples
    /// ```
    /// use tty_interface::{Interface, Position};
    /// 
    /// let mut interface = Interface::for_stdout().expect("terminal size should be available");
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

            alternate.set(Position::new(column, line), grapheme);

            column += 1;
        }
    }

    /// Applies staged changes to the terminal.
    /// 
    /// # Examples
    /// ```
    /// use tty_interface::{Interface, Position};
    /// 
    /// let mut interface = Interface::for_stdout().expect("terminal size should be available");
    /// interface.set(Position::new(1, 1), "Hello, world!");
    /// interface.apply().expect("updates should be valid");
    /// ```
    pub fn apply(&mut self) -> Result<()> {
        if self.alternate.is_none() {
            return Ok(())
        }

        let mut alternate = self.alternate.take().unwrap();
        swap(&mut self.current, &mut alternate);

        let dirty_cells: Vec<(Position, String)> = self.current.dirty_iter().collect();
        
        let writer = self.writer();
        for (position, text) in dirty_cells {
            writer.queue(cursor::MoveTo(position.x(), position.y()))?;
            writer.queue(style::Print(text))?;
        }

        writer.flush()?;

        self.current.clear_dirty();
        
        Ok(())
    }

    /// Get the appropriate output writer for this interface.
    fn writer(&mut self) -> &mut dyn Write {
        if let Some(writer) = self.writer.as_mut() {
            writer
        } else if let Some(stdout) = self.stdout.as_mut() {
            stdout
        } else {
            panic!("interface has no output writer")
        }
    }
}