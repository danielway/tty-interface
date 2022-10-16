/// Colors to be used for foreground and background text formatting.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Grey,
    Reset,
}

/// Text formatting styles.
///
/// # Examples
/// ```
/// use tty_interface::{Color, Style};
///
/// let style = Style::default().set_foreground(Color::Red).set_bold(true);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Style {
    foreground_color: Option<Color>,
    background_color: Option<Color>,
    is_bold: bool,
    is_italic: bool,
    is_underline: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground_color: None,
            background_color: None,
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }
}

impl Style {
    /// Create a new style with the specified foreground color.
    pub fn set_foreground(&self, color: Color) -> Style {
        Style {
            foreground_color: Some(color),
            ..*self
        }
    }

    /// This style's foreground color, if specified.
    pub fn foreground(&self) -> Option<Color> {
        self.foreground_color
    }

    /// Create a new style with the specified background color.
    pub fn set_background(&self, color: Color) -> Style {
        Style {
            background_color: Some(color),
            ..*self
        }
    }

    /// This style's background color, if specified.
    pub fn background(&self) -> Option<Color> {
        self.background_color
    }

    /// Create a new style with the specified bold value.
    pub fn set_bold(&self, is_bold: bool) -> Style {
        Style { is_bold, ..*self }
    }

    /// Whether this style is bolded.
    pub fn is_bold(&self) -> bool {
        self.is_bold
    }

    /// Create a new style with the specified italic value.
    pub fn set_italic(&self, is_italic: bool) -> Style {
        Style { is_italic, ..*self }
    }

    /// Whether this style is italicized.
    pub fn is_italic(&self) -> bool {
        self.is_italic
    }

    /// Create a new style with the specified underline value.
    pub fn set_underline(&self, is_underline: bool) -> Style {
        Style {
            is_underline,
            ..*self
        }
    }

    /// Whether this style is underlined.
    pub fn is_underlined(&self) -> bool {
        self.is_underline
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Style};

    #[test]
    fn style_foreground() {
        let mut style = Style::default();
        assert_eq!(None, style.foreground());

        style = style.set_foreground(Color::Blue);
        assert_eq!(Some(Color::Blue), style.foreground());

        style = style.set_foreground(Color::Red);
        assert_eq!(Some(Color::Red), style.foreground());
    }

    #[test]
    fn style_background() {
        let mut style = Style::default();
        assert_eq!(None, style.background());

        style = style.set_background(Color::Yellow);
        assert_eq!(Some(Color::Yellow), style.background());

        style = style.set_background(Color::Magenta);
        assert_eq!(Some(Color::Magenta), style.background());
    }

    #[test]
    fn style_bold() {
        let mut style = Style::default();
        assert_eq!(false, style.is_bold());

        style = style.set_bold(true);
        assert_eq!(true, style.is_bold());
    }

    #[test]
    fn style_italic() {
        let mut style = Style::default();
        assert_eq!(false, style.is_italic());

        style = style.set_italic(true);
        assert_eq!(true, style.is_italic());
    }

    #[test]
    fn style_underline() {
        let mut style = Style::default();
        assert_eq!(false, style.is_underlined());

        style = style.set_underline(true);
        assert_eq!(true, style.is_underlined());
    }
}
