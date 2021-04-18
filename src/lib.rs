//! Provides simple TTY-based interface capabilities including partial re-renders of multi-line displays.

pub mod interface;
pub mod line;
pub mod segment;
pub mod cursor;
pub mod result;
pub mod update;

mod utility;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
