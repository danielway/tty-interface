pub mod interface;
pub mod line;
pub mod segment;
pub mod cursor;

mod update;
mod utility;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
