use std::{
    io::{stdout, Stdout, Write},
    mem::swap,
};

use crossterm::{
    cursor,
    style::{self, Attribute, ContentStyle, StyledContent},
    terminal, QueueableCommand,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{pos, Cell, Color, Position, Result, State, Style, Vector};

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
    /// use tty_interface::{Interface, Position, pos};
    ///
    /// let mut interface = Interface::for_stdout().expect("terminal size should be available");
    /// interface.set(pos!(1, 1), "Hello, world!");
    /// ```
    pub fn set(&mut self, position: Position, text: &str) {
        self.stage_text(position, text, None)
    }

    /// Update the interface's text at the specified position. Changes are staged until applied.
    ///
    /// # Examples
    /// ```
    /// use tty_interface::{Interface, Position, pos, Style};
    ///
    /// let mut interface = Interface::for_stdout().expect("terminal size should be available");
    /// interface.set_styled(pos!(1, 1), "Hello, world!", Style::default().set_bold(true));
    /// ```
    pub fn set_styled(&mut self, position: Position, text: &str, style: Style) {
        self.stage_text(position, text, Some(style))
    }

    /// Stages the specified text and optional style at a position in the terminal.
    fn stage_text(&mut self, position: Position, text: &str, style: Option<Style>) {
        let alternate = self.alternate.get_or_insert_with(|| self.current.clone());

        let mut line = position.y().into();
        let mut column = position.x().into();

        for grapheme in text.graphemes(true) {
            if column > self.size.x().into() {
                column = 0;
                line += 1;
            }

            let cell_position = pos!(column, line);
            match style {
                Some(style) => alternate.set_styled_text(cell_position, grapheme, style),
                None => alternate.set_text(cell_position, grapheme),
            }

            column += 1;
        }
    }

    /// Applies staged changes to the terminal.
    ///
    /// # Examples
    /// ```
    /// use tty_interface::{Interface, Position, pos};
    ///
    /// let mut interface = Interface::for_stdout().expect("terminal size should be available");
    /// interface.set(pos!(1, 1), "Hello, world!");
    /// interface.apply().expect("updates should be valid");
    /// ```
    pub fn apply(&mut self) -> Result<()> {
        if self.alternate.is_none() {
            return Ok(());
        }

        let mut alternate = self.alternate.take().unwrap();
        swap(&mut self.current, &mut alternate);

        let dirty_cells: Vec<(Position, Cell)> = self.current.dirty_iter().collect();

        let writer = self.writer();
        for (position, cell) in dirty_cells {
            writer.queue(cursor::MoveTo(position.x(), position.y()))?;

            let mut content_style = ContentStyle::default();
            if let Some(style) = cell.style() {
                content_style = get_content_style(*style);
            }

            let styled_content = StyledContent::new(content_style, cell.grapheme());
            writer.queue(style::PrintStyledContent(styled_content))?;
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

/// Converts a style from its internal representation to crossterm's.
fn get_content_style(style: Style) -> ContentStyle {
    let mut content_style = ContentStyle::default();

    if let Some(color) = style.foreground() {
        match color {
            Color::Black => content_style.foreground_color = Some(style::Color::Black),
            Color::Red => content_style.foreground_color = Some(style::Color::Red),
            Color::Green => content_style.foreground_color = Some(style::Color::Green),
            Color::Yellow => content_style.foreground_color = Some(style::Color::Yellow),
            Color::Blue => content_style.foreground_color = Some(style::Color::Blue),
            Color::Magenta => content_style.foreground_color = Some(style::Color::Magenta),
            Color::Cyan => content_style.foreground_color = Some(style::Color::Cyan),
            Color::White => content_style.foreground_color = Some(style::Color::White),
            Color::Grey => content_style.foreground_color = Some(style::Color::Grey),
            Color::Reset => content_style.foreground_color = Some(style::Color::Reset),
        };
    }

    if style.is_bold() {
        content_style.attributes.set(Attribute::Bold);
    }

    if style.is_italic() {
        content_style.attributes.set(Attribute::Italic);
    }

    if style.is_underlined() {
        content_style.attributes.set(Attribute::Underlined);
    }

    content_style
}
