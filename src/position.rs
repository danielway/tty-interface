use std::fmt::Debug;

/// A coordinate position in the terminal. May be absolute or relative to some buffer's origin.
#[derive(Eq, PartialEq, Copy, Clone)]
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
    /// assert_eq!(2, origin.x());
    /// assert_eq!(4, origin.y());
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

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.y().cmp(&other.y()) {
            std::cmp::Ordering::Equal => self.x().cmp(&other.x()),
            ordering => ordering,
        }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position({}, {})", self.x(), self.y())
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Position;

    #[test]
    fn position_initialization() {
        let cases = [(0,0), (0,3), (8,2), (4,6)];

        for (x, y) in cases {
            let position = Position::new(x, y);
            
            assert_eq!(x, position.x());
            assert_eq!(y, position.y());
        }
    }

    #[test]
    fn position_comparison() {
        let assert_case = |first: (u16, u16), second: (u16, u16), expected: Ordering| {
            let first_position = Position::new(first.0, first.1);
            let second_position = Position::new(second.0, second.1);

            assert_eq!(
                expected, 
                first_position.cmp(&second_position), 
                "comparing {:?} and {:?}", first_position, second_position);
            
            assert_eq!(
                Some(expected), 
                first_position.partial_cmp(&second_position), 
                "comparing {:?} and {:?}", 
                first_position, 
                second_position,
            );
        };

        let positions = [(0,0), (0,1), (1,0), (1, 1)];

        let cases = [
            (positions[0], positions[0], Ordering::Equal),
            (positions[0], positions[1], Ordering::Less),
            (positions[0], positions[2], Ordering::Less),
            (positions[0], positions[3], Ordering::Less),
            (positions[1], positions[0], Ordering::Greater),
            (positions[1], positions[1], Ordering::Equal),
            (positions[1], positions[2], Ordering::Greater),
            (positions[1], positions[3], Ordering::Less),
            (positions[2], positions[0], Ordering::Greater),
            (positions[2], positions[1], Ordering::Less),
            (positions[2], positions[2], Ordering::Equal),
            (positions[2], positions[3], Ordering::Less),
            (positions[3], positions[0], Ordering::Greater),
            (positions[3], positions[1], Ordering::Greater),
            (positions[3], positions[2], Ordering::Greater),
            (positions[3], positions[3], Ordering::Equal),
        ];

        for case in cases {
            assert_case(case.0, case.1, case.2);
        }
    }
}