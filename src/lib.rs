//! Provides simple TTY-based interface capabilities through an accessible API. Stages changes to
//! the interface until applied as a batch while minimizing writes to the output device.

extern crate core;

pub mod config;
pub mod device;
pub mod format;
pub mod layout;
pub mod line;
pub mod mode;
pub mod position;
pub mod segment;

mod interface;
mod result;
mod terminal;
mod text;

pub use crate::interface::Interface;
pub use crate::result::{Error, Result};
