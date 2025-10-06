use crate::{pos, Device, Position, Result, Vector};

/// A virtual testing device based on the vte/vt100 parser. Ideally, this would be hidden from
/// production builds and only available to functional, documentation, and unit tests, but that does
/// not seem to be possible currently.
pub struct VirtualDevice(vt100::Parser);

impl Default for VirtualDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualDevice {
    /// Create a new device based around a virtual terminal.
    pub fn new() -> Self {
        Self(vt100::Parser::default())
    }

    /// Access this device's underlying parser.
    pub fn parser(&mut self) -> &mut vt100::Parser {
        &mut self.0
    }
}

impl Device for VirtualDevice {
    fn get_terminal_size(&mut self) -> Result<Vector> {
        let (lines, columns) = self.0.screen().size();
        Ok(Vector::new(columns, lines))
    }

    fn enable_raw_mode(&mut self) -> Result<()> {
        Ok(())
    }

    fn disable_raw_mode(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<Position> {
        Ok(pos!(0, 0))
    }
}

impl std::io::Write for VirtualDevice {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
