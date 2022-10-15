/// A coordinate position in the terminal. May be absolute or relative to some buffer's origin.
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    /// Create a new, immutable position.
    /// 
    /// # Examples
    /// ```
    /// let origin = tty_interface::Position::new(2, 4);
    /// assert_eq!(origin.x(), 2);
    /// assert_eq!(origin.y(), 4);
    /// ```
    pub fn new(x: u16, y: u16) -> Position {
        Position { x, y }
    }

    /// This position's column value.
    pub fn x(&self) -> u16 {
        self.x
    }

    /// This position's line value.
    pub fn y(&self) -> u16 {
        self.y
    }
}