use crate::mode::{CursorMode, RenderMode};

/// Options for an interface's behavior.
pub struct Configuration {
    cursor_mode: CursorMode,
    render_mode: RenderMode,
}

impl Configuration {
    /// Create a new, immutable configuration.
    pub fn new(cursor_mode: CursorMode, render_mode: RenderMode) -> Self {
        Self {
            cursor_mode,
            render_mode,
        }
    }

    /// This configuration's cursor movement mode.
    pub fn cursor_mode(&self) -> CursorMode {
        self.cursor_mode
    }

    /// This configuration's terminal rendering mode.
    pub fn render_mode(&self) -> RenderMode {
        self.render_mode
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cursor_mode: CursorMode::default(),
            render_mode: RenderMode::default(),
        }
    }
}
