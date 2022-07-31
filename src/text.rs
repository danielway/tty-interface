use crate::format::{Color, Style};
use crate::layout::PartLayout;

/// Parameters for rendering text to the terminal including optional metadata like styling.
#[derive(Debug, PartialEq, Clone)]
pub struct TextParameters<'a> {
    text: &'a str,
    color: Option<Color>,
    styles: Vec<Style>,
}

impl TextParameters<'_> {
    /// Create text parameters with optional coloring or styling.
    pub(crate) fn new(text: &str, color: Option<Color>, styles: Vec<Style>) -> TextParameters {
        TextParameters {
            text,
            color,
            styles,
        }
    }

    /// The text to render.
    pub(crate) fn text(&self) -> &str {
        self.text
    }

    /// The color to style text with.
    pub(crate) fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }

    /// The styles to apply to text.
    pub(crate) fn styles(&self) -> &Vec<Style> {
        &self.styles
    }

    /// Create new parameters with updated render text.
    pub(crate) fn set_text<'a>(&self, new_text: &'a str) -> TextParameters<'a> {
        TextParameters::new(new_text, self.color, self.styles.clone())
    }

    /// Determines whether this and another set of parameters have identical color and styles.
    pub(crate) fn has_same_styles(&self, other: &TextParameters) -> bool {
        self.color == other.color && self.styles == other.styles
    }
}

/// Rendered text parameters with their final layout.
#[derive(Debug)]
pub(crate) struct TextParametersWithLayout<'a> {
    parameters: TextParameters<'a>,
    layout: Vec<PartLayout>,
}

impl TextParametersWithLayout<'_> {
    /// Create for the specified parameters and their layout.
    pub(crate) fn new(
        parameters: TextParameters,
        layout: Vec<PartLayout>,
    ) -> TextParametersWithLayout {
        TextParametersWithLayout { parameters, layout }
    }

    /// The parameters used to render.
    pub(crate) fn parameters(&self) -> &TextParameters {
        &self.parameters
    }

    /// The parameters' rendered layout in the terminal.
    pub(crate) fn layout(&self) -> &Vec<PartLayout> {
        &self.layout
    }
}
