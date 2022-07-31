use crate::line::LineId;
use crate::segment::SegmentId;

/// A position in the terminal. May not have all information necessary to describe an exact position
/// and may be invalid depending on interface state.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Position {
    /// An absolute coordinate position in the terminal.
    Absolute(AbsolutePosition),
    /// A position relative to a line and segment.
    Relative(RelativePosition),
}

/// An absolute coordinate position in the terminal, irrespective of content or wrapping.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AbsolutePosition {
    column: u16,
    row: u16,
}

impl AbsolutePosition {
    /// Create a new absolute position.
    pub fn new(column: u16, row: u16) -> Self {
        Self { column, row }
    }

    /// Create a new absolute position from a (column, row) tuple.
    pub fn new_from_tuple(tuple: (u16, u16)) -> Self {
        Self::new(tuple.0, tuple.1)
    }

    /// This position's zero-indexed column.
    pub fn column(&self) -> u16 {
        self.column
    }

    /// This position's zero-indexed row relative to the interface's origin.
    pub fn row(&self) -> u16 {
        self.row
    }

    /// Create a new position from this one with the specified column.
    pub fn set_column(&self, column: u16) -> AbsolutePosition {
        AbsolutePosition {
            column,
            row: self.row,
        }
    }

    /// Create a new position from this one with the specified row.
    pub fn set_row(&self, row: u16) -> AbsolutePosition {
        AbsolutePosition {
            column: self.column,
            row,
        }
    }

    /// Create a new position from this one with the columns modified as specified.
    pub fn add_columns(&self, diff_columns: i16) -> AbsolutePosition {
        AbsolutePosition {
            column: (self.column as i16 + diff_columns) as u16,
            row: self.row,
        }
    }

    /// Create a new position from this one with the rows modified as specified.
    pub fn add_rows(&self, diff_rows: i16) -> AbsolutePosition {
        AbsolutePosition {
            column: self.column,
            row: (self.row as i16 + diff_rows) as u16,
        }
    }
}

impl Default for AbsolutePosition {
    /// Create a default position with value (0, 0).
    fn default() -> AbsolutePosition {
        AbsolutePosition::new(0, 0)
    }
}

/// A content-relative position in the terminal.
///
/// An absolute position may be derived with a layout by identifying the line and segment this
/// position refers to, the part layout whose indices overlap this position's `position`, and adding
/// the difference of those x offsets to the part layout's absolute position.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RelativePosition {
    line_id: LineId,
    segment_id: SegmentId,
    position: u16,
}

impl RelativePosition {
    /// Create a new relative position for the specified line, segment, and text position. The text
    /// position is a grapheme index.
    pub fn new(line_id: LineId, segment_id: SegmentId, position: u16) -> Self {
        Self {
            line_id,
            segment_id,
            position,
        }
    }

    /// The line this position is relative to.
    pub fn line_id(&self) -> LineId {
        self.line_id
    }

    /// The segment this position is relative to.
    pub fn segment_id(&self) -> SegmentId {
        self.segment_id
    }

    /// The grapheme index relative to the specified line and segment.
    pub fn position(&self) -> u16 {
        self.position
    }
}
