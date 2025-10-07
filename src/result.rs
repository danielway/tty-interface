use thiserror::Error as ThisError;

/// An interface operation's result containing either a successful value or error.
pub type Result<T> = std::result::Result<T, Error>;

/// A failed interface operation's error information.
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("terminal interaction error")]
    Terminal(#[from] std::io::Error),
}
