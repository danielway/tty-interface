use termion::{color, style};

/// May append contents to a string.
pub(crate) trait StringAppender {
    /// Appends contents to the specified string.
    fn append_to_string(&self, string: &mut String);
}

/// A color to be applied to text foreground or background in the interface.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl StringAppender for Color {
    fn append_to_string(&self, string: &mut String) {
        match *self {
            Color::Black => string.push_str(&color::Fg(color::Black).to_string()),
            Color::Red => string.push_str(&color::Fg(color::Red).to_string()),
            Color::Green => string.push_str(&color::Fg(color::Green).to_string()),
            Color::Yellow => string.push_str(&color::Fg(color::Yellow).to_string()),
            Color::Blue => string.push_str(&color::Fg(color::Blue).to_string()),
            Color::Magenta => string.push_str(&color::Fg(color::Magenta).to_string()),
            Color::Cyan => string.push_str(&color::Fg(color::Cyan).to_string()),
            Color::White => string.push_str(&color::Fg(color::White).to_string()),
        }
    }
}

/// Styling to be applied to text in the interface.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Style {
    Bold,
    Italic,
    Underline,
}

impl ToString for Style {
    fn to_string(&self) -> String {
        match *self {
            Style::Bold => style::Bold.to_string(),
            Style::Italic => style::Italic.to_string(),
            Style::Underline => style::Underline.to_string(),
        }
    }
}

impl StringAppender for Vec<Style> {
    fn append_to_string(&self, string: &mut String) {
        for style in self {
            string.push_str(&style.to_string());
        }
    }
}
