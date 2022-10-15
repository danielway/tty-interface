/// A directional vector with no positional information.
pub struct Vector {
    x: u16,
    y: u16,
}

impl Vector {
    /// Create a new, immutable vector.
    /// 
    /// # Examples
    /// ```
    /// let size = tty_interface::Vector::new(7, 4);
    /// assert_eq!(size.x(), 7);
    /// assert_eq!(size.y(), 4);
    /// ```
    pub fn new(x: u16, y: u16) -> Vector {
        Vector { x, y }
    }

    /// This vector's column value.
    pub fn x(&self) -> u16 {
        self.x
    }
    
    /// This vector's line value.
    pub fn y(&self) -> u16 {
        self.y
    }
}