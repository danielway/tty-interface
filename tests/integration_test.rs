use serial_test::serial;
use tty_interface::{self, pos, Color, Interface, Position, Style};
use vt100::Parser;

#[test]
#[serial]
fn basic_write() {
    let mut parser = Parser::default();

    let mut interface = Interface::for_writer(&mut parser).unwrap();
    interface.set(pos!(0, 0), "Hello, world!");
    interface.apply().unwrap();

    assert_eq!("Hello, world!", &parser.screen().contents());
}

#[test]
#[serial]
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
#[serial]
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

#[test]
#[serial]
fn multiple_overlapping_formatted_writes() {
    let mut parser = Parser::default();

    let mut interface = Interface::for_writer(&mut parser).unwrap();

    interface.set_styled(pos!(0, 0), "FIRST", Style::default().set_bold(true));
    interface.apply().unwrap();

    interface.set(pos!(2, 0), "SECOND");
    interface.apply().unwrap();

    interface.set_styled(
        pos!(4, 0),
        "THIRD",
        Style::default().set_italic(true).set_foreground(Color::Red),
    );
    interface.apply().unwrap();

    let expected_text = ["F", "I", "S", "E", "T", "H", "I", "R", "D"];
    let expected_bold = [true, true, false, false, false, false, false, false, false];
    let expected_italic = [false, false, false, false, true, true, true, true, true];
    let expected_color = [
        vt100::Color::Default,
        vt100::Color::Default,
        vt100::Color::Default,
        vt100::Color::Default,
        vt100::Color::Idx(9),
        vt100::Color::Idx(9),
        vt100::Color::Idx(9),
        vt100::Color::Idx(9),
        vt100::Color::Idx(9),
    ];

    assert_eq!("FISETHIRD", &parser.screen().contents());

    for column in 0..expected_text.len() {
        let cell = parser.screen().cell(0, column as u16).unwrap();
        assert_eq!(expected_text[column], cell.contents());
        assert_eq!(expected_bold[column], cell.bold());
        assert_eq!(expected_italic[column], cell.italic());
        assert_eq!(expected_color[column], cell.fgcolor())
    }
}
