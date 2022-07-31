use std::fmt::Formatter;

/// Indicates whether an interface operation was successful or failed.
pub type Result<T> = std::result::Result<T, Error>;

/// Failure modes for interface operations.
#[derive(Debug)]
pub enum Error {
    /// The specified segment index was out of bounds.
    SegmentOutOfBounds,
    /// The specified line index was out of bounds.
    LineOutOfBounds,
    /// The specified segment ID is invalid.
    SegmentIdInvalid,
    /// The specified line ID is invalid.
    LineIdInvalid,
    /// The specified cursor position was invalid.
    CursorPositionInvalid,
    /// The segment's content included a newline.
    MidSegmentNewlineInvalid,
    /// A low-level IO error occurred while performing interface operations.
    IO(std::io::Error),
    /// A low-level formatting error occurred while performing interface operations.
    Format(core::fmt::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::SegmentOutOfBounds => write!(f, "Segment reference index is out-of-bounds."),
            Error::LineOutOfBounds => write!(f, "Line reference index is out-of-bounds."),
            Error::SegmentIdInvalid => write!(f, "Segment identifier is invalid."),
            Error::LineIdInvalid => write!(f, "Line identifier is invalid."),
            Error::CursorPositionInvalid => write!(f, "Specified cursor position is invalid."),
            Error::MidSegmentNewlineInvalid => write!(f, "Segment text includes a newline."),
            Error::IO(..) => write!(f, "Failure interacting with TTY device."),
            Error::Format(..) => write!(f, "Failure formatting TTY device output."),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::SegmentOutOfBounds => None,
            Error::LineOutOfBounds => None,
            Error::SegmentIdInvalid => None,
            Error::LineIdInvalid => None,
            Error::CursorPositionInvalid => None,
            Error::MidSegmentNewlineInvalid => None,
            Error::IO(ref err) => Some(err),
            Error::Format(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<core::fmt::Error> for Error {
    fn from(err: core::fmt::Error) -> Error {
        Error::Format(err)
    }
}
