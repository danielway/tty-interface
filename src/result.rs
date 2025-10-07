/// An interface operation's result containing either a successful value or error.
pub type Result<T> = std::result::Result<T, Error>;

/// A failed interface operation's error information.
#[derive(Debug)]
pub enum Error {
    /// A low-level terminal interaction error.
    Terminal(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Terminal(err)
    }
}
