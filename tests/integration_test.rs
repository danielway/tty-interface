use tty_interface::{self, Interface, Position, pos};
use vt100::Parser;

#[test]
fn basic_write() {
    let mut parser = Parser::default();
    
    let mut interface = Interface::for_writer(&mut parser).unwrap();
    interface.set(pos!(0, 0), "Hello, world!");
    interface.apply().unwrap();
    
    assert_eq!("Hello, world!", &parser.screen().contents());
}

#[test]
fn multiple_writes() {
    let mut parser = Parser::default();
    
    let mut interface = Interface::for_writer(&mut parser).unwrap();
    
    interface.set(pos!(0, 0), "Line 1");
    interface.apply().unwrap();
    
    interface.set(pos!(0, 1), "Line 2");
    interface.apply().unwrap();
    
    interface.set(pos!(7, 0), "with more");
    interface.apply().unwrap();
    
    assert_eq!("Line 1 with more\nLine 2", &parser.screen().contents());
}

#[test]
fn overlapping_writes() {
    let mut parser = Parser::default();
    
    let mut interface = Interface::for_writer(&mut parser).unwrap();
    
    interface.set(pos!(0, 0), "ABCDEF");
    interface.apply().unwrap();
    
    interface.set(pos!(1, 0), "X");
    interface.apply().unwrap();
    
    interface.set(pos!(3, 0), "ZZ");
    interface.apply().unwrap();
    
    assert_eq!("AXCZZF", &parser.screen().contents());
}