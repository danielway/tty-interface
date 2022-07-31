/// How the cursor should be moved around the terminal.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CursorMode {
    /// Jogs the cursor with up/down/left/right movements, rather than using goto-commands.
    Relative,
    /// Positions the cursor by coordinates with goto-commands.
    Absolute,
}

impl Default for CursorMode {
    fn default() -> Self {
        CursorMode::Relative
    }
}

/// How the interface should be rendered to the terminal.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RenderMode {
    /// Appends the interface to the existing buffer content. Attempts to preserve the existing
    /// buffer content, but this may not handle terminal resizes well.
    Relative,
    /// Assumes full control of the terminal, clearing all existing content in the viewport.
    Full,
}

impl Default for RenderMode {
    fn default() -> Self {
        RenderMode::Full
    }
}
