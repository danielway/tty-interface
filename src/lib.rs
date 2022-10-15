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

mod result;
pub use result::{Result, Error};

mod state;
pub(crate) use state::State;
