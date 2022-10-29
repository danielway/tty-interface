use std::mem::swap;

use crossterm::{
    cursor,
    style::{self, Attribute, ContentStyle, StyledContent},
    terminal, QueueableCommand,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{pos, Cell, Color, Device, Position, Result, State, Style, Vector};

/// A TTY-based user-interface providing optimized update rendering.
pub struct Interface<'a> {
    device: &'a mut dyn Device,
    size: Vector,
    current: State,
    alternate: Option<State>,
    staged_cursor: Option<Position>,
}

impl Interface<'_> {
    /// Create a new interface for the specified device.
    ///
    /// # Examples
    /// ```
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::Interface;
    ///
    /// let interface = Interface::new(&mut device)?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn new<'a>(device: &'a mut dyn Device) -> Result<Interface<'a>> {
        let size = device.get_terminal_size()?;

        let mut interface = Interface {
            device,
            size,
            current: State::new(),
            alternate: None,
            staged_cursor: None,
        };

        let device = &mut interface.device;
        device.enable_raw_mode()?;
        device.queue(terminal::EnterAlternateScreen)?;
        device.queue(terminal::Clear(terminal::ClearType::All))?;
        device.queue(cursor::Hide)?;
        device.queue(cursor::MoveTo(0, 0))?;
        device.flush()?;

        Ok(interface)
    }

    /// When finished using this interface, uninitialize its terminal configuration.
    ///
    /// # Examples
    /// ```
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::Interface;
    ///
    /// let interface = Interface::new(&mut device)?;
    /// interface.exit()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn exit(self) -> Result<()> {
        self.device.disable_raw_mode()?;
        self.device.queue(terminal::LeaveAlternateScreen)?;
        self.device.flush()?;

        println!();
        Ok(())
    }

    /// Update the interface's text at the specified position. Changes are staged until applied.
    ///
    /// # Examples
    /// ```
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::{Interface, Position, pos};
    ///
    /// let mut interface = Interface::new(&mut device)?;
    /// interface.set(pos!(1, 1), "Hello, world!");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn set(&mut self, position: Position, text: &str) {
        self.stage_text(position, text, None)
    }

    /// Update the interface's text at the specified position. Changes are staged until applied.
    ///
    /// # Examples
    /// ```
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::{Interface, Style, Position, pos};
    ///
    /// let mut interface = Interface::new(&mut device)?;
    /// interface.set_styled(pos!(1, 1), "Hello, world!", Style::new().set_bold(true));
    /// # Ok::<(), Error>(())
    /// ```
    pub fn set_styled(&mut self, position: Position, text: &str, style: Style) {
        self.stage_text(position, text, Some(style))
    }

    /// Clear all text on the specified line. Changes are staged until applied.
    ///
    /// # Examples
    /// ```
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::{Interface, Style, Position, pos};
    ///
    /// let mut interface = Interface::new(&mut device)?;
    ///
    /// // Write "Hello," and "world!" on two different lines
    /// interface.set(pos!(0, 0), "Hello,");
    /// interface.set(pos!(0, 1), "world!");
    /// interface.apply()?;
    ///
    /// // Clear the second line, "world!"
    /// interface.clear_line(1);
    /// interface.apply()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn clear_line(&mut self, line: u16) {
        let alternate = self.alternate.get_or_insert_with(|| self.current.clone());
        alternate.clear_line(line);
    }

    /// Clear the remainder of the line from the specified position. Changes are staged until
    /// applied.
    ///
    /// # Examples
    /// ```
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::{Interface, Style, Position, pos};
    ///
    /// let mut interface = Interface::new(&mut device)?;
    ///
    /// // Write "Hello, world!" to the first line
    /// interface.set(pos!(0, 0), "Hello, world!");
    /// interface.apply()?;
    ///
    /// // Clear everything after "Hello"
    /// interface.clear_rest_of_line(pos!(5, 0));
    /// interface.apply()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn clear_rest_of_line(&mut self, from: Position) {
        let alternate = self.alternate.get_or_insert_with(|| self.current.clone());
        alternate.clear_rest_of_line(from);
    }

    /// Clear the remainder of the interface from the specified position. Changes are staged until
    /// applied.
    ///
    /// # Examples
    /// ```
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::{Interface, Style, Position, pos};
    ///
    /// let mut interface = Interface::new(&mut device)?;
    ///
    /// // Write two lines of content
    /// interface.set(pos!(0, 0), "Hello, world!");
    /// interface.set(pos!(0, 1), "Another line");
    /// interface.apply()?;
    ///
    /// // Clear everything after "Hello", including the second line
    /// interface.clear_rest_of_interface(pos!(5, 0));
    /// interface.apply()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn clear_rest_of_interface(&mut self, from: Position) {
        let alternate = self.alternate.get_or_insert_with(|| self.current.clone());
        alternate.clear_rest_of_interface(from);
    }

    /// Update the interface's cursor to the specified position, or hide it if unspecified.
    ///
    /// # Examples
    /// ```
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::{Interface, Position, pos};
    ///
    /// let mut interface = Interface::new(&mut device)?;
    /// interface.set_cursor(Some(pos!(1, 2)));
    /// # Ok::<(), Error>(())
    /// ```
    pub fn set_cursor(&mut self, position: Option<Position>) {
        self.staged_cursor = position;
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
    /// # use tty_interface::{Error, test::VirtualDevice};
    /// # let mut device = VirtualDevice::new();
    /// use tty_interface::{Interface, Position, pos};
    ///
    /// let mut interface = Interface::new(&mut device)?;
    /// interface.set(pos!(1, 1), "Hello, world!");
    /// interface.apply()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn apply(&mut self) -> Result<()> {
        if self.alternate.is_none() {
            return Ok(());
        }

        let mut alternate = self.alternate.take().unwrap();
        swap(&mut self.current, &mut alternate);

        let dirty_cells: Vec<(Position, Option<Cell>)> = self.current.dirty_iter().collect();

        self.device.queue(cursor::Hide)?;

        for (position, cell) in dirty_cells {
            let move_cursor = cursor::MoveTo(position.x(), position.y());
            self.device.queue(move_cursor)?;

            match cell {
                Some(cell) => {
                    let mut content_style = ContentStyle::default();
                    if let Some(style) = cell.style() {
                        content_style = get_content_style(*style);
                    }

                    let styled_content = StyledContent::new(content_style, cell.grapheme());
                    let print_styled_content = style::PrintStyledContent(styled_content);
                    self.device.queue(print_styled_content)?;
                }
                None => {
                    let clear_content = style::Print(' ');
                    self.device.queue(clear_content)?;
                }
            }
        }

        if let Some(position) = self.staged_cursor {
            self.device
                .queue(cursor::MoveTo(position.x(), position.y()))?;
            self.device.queue(cursor::Show)?;
        }

        self.device.flush()?;

        self.current.clear_dirty();

        Ok(())
    }
}

/// Converts a style from its internal representation to crossterm's.
fn get_content_style(style: Style) -> ContentStyle {
    let mut content_style = ContentStyle::default();

    if let Some(color) = style.foreground() {
        match color {
            Color::Black => content_style.foreground_color = Some(style::Color::Black),
            Color::DarkGrey => content_style.foreground_color = Some(style::Color::DarkGrey),
            Color::Red => content_style.foreground_color = Some(style::Color::Red),
            Color::DarkRed => content_style.foreground_color = Some(style::Color::DarkRed),
            Color::Green => content_style.foreground_color = Some(style::Color::Green),
            Color::DarkGreen => content_style.foreground_color = Some(style::Color::DarkGreen),
            Color::Yellow => content_style.foreground_color = Some(style::Color::Yellow),
            Color::DarkYellow => content_style.foreground_color = Some(style::Color::DarkYellow),
            Color::Blue => content_style.foreground_color = Some(style::Color::Blue),
            Color::DarkBlue => content_style.foreground_color = Some(style::Color::DarkBlue),
            Color::Magenta => content_style.foreground_color = Some(style::Color::Magenta),
            Color::DarkMagenta => content_style.foreground_color = Some(style::Color::DarkMagenta),
            Color::Cyan => content_style.foreground_color = Some(style::Color::Cyan),
            Color::DarkCyan => content_style.foreground_color = Some(style::Color::DarkCyan),
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
