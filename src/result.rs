use std::{fmt, error};
use std::fmt::Formatter;
use std::error::Error;

/// A specialized result type encompassing errors from the tty-interface crate and `std::io`.
pub type Result<T> = std::result::Result<T, TTYError>;

/// A specialized error type encompassing tty-interface and `std::io::Error` errors.
#[derive(Debug)]
pub enum TTYError {
    LineOutOfBounds,
    SegmentOutOfBounds,
    CursorOutOfBounds,
    IO(std::io::Error),
}

impl fmt::Display for TTYError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            TTYError::LineOutOfBounds => write!(f, "line update references out-of-bounds index"),
            TTYError::SegmentOutOfBounds => write!(f, "segment update references out-of-bounds index"),
            TTYError::CursorOutOfBounds => write!(f, "cursor update is invalid/out-of-bounds"),
            TTYError::IO(..) => write!(f, "failure while attempting to render updates"),
        }
    }
}

impl Error for TTYError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            TTYError::LineOutOfBounds => None,
            TTYError::SegmentOutOfBounds => None,
            TTYError::CursorOutOfBounds => None,
            TTYError::IO(ref e) => Some(e),
        }
    }
}

impl From<std::io::Error> for TTYError {
    fn from(err: std::io::Error) -> TTYError {
        TTYError::IO(err)
    }
}
