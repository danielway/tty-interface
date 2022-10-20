use crate::{Result, Vector};

/// An output device to be controlled for displaying an interface.
pub trait Device: std::io::Write {
    /// Retrieve the device's terminal viewport size.
    fn get_terminal_size(&mut self) -> Result<Vector>;

    /// Enable "raw mode" in the terminal.
    fn enable_raw_mode(&mut self) -> Result<()>;

    /// Restore the configuration before the terminal was placed in "raw mode".
    fn disable_raw_mode(&mut self) -> Result<()>;
}

impl Device for std::io::Stdout {
    fn get_terminal_size(&mut self) -> Result<Vector> {
        let (columns, lines) = crossterm::terminal::size()?;
        Ok(Vector::new(columns, lines))
    }

    fn enable_raw_mode(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(())
    }

    fn disable_raw_mode(&mut self) -> Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}
