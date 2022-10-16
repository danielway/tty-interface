/// An interface operation's result containing either a successful value or error.
pub type Result<T> = std::result::Result<T, Error>;

/// A failed interface operation's error information.
#[derive(Debug)]
pub enum Error {
    /// A low-level terminal interaction error.
    Terminal(crossterm::ErrorKind),
}

impl From<crossterm::ErrorKind> for Error {
    fn from(err: crossterm::ErrorKind) -> Self {
        Error::Terminal(err)
    }
}
