//! # tty-interface
//!
//! Provides simple TTY-based user interface capabilities including partial re-renders of multi-line displays.
//!

mod position;
pub use position::Position;

mod vector;
pub use vector::Vector;

mod interface;
pub use interface::Interface;

mod device;
pub use device::Device;

mod result;
pub use result::{Error, Result};

mod style;
pub use style::{Color, Style};

mod state;
pub(crate) use state::{Cell, State};

pub mod test;
