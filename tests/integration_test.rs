use tty_interface::{self, Color, Interface, Position, Style, pos, test::VirtualDevice};

#[test]
fn basic_write() {
    let mut device = VirtualDevice::new();
    let mut interface = Interface::new_alternate(&mut device).unwrap();

    interface.set(pos!(0, 0), "Hello, world!");
    interface.apply().unwrap();

    assert_eq!("Hello, world!", &device.parser().screen().contents());
}

#[test]
fn multiple_writes() {
    let mut device = VirtualDevice::new();
    let mut interface = Interface::new_alternate(&mut device).unwrap();

    interface.set(pos!(0, 0), "Line 1");
    interface.apply().unwrap();

    interface.set(pos!(0, 1), "Line 2");
    interface.apply().unwrap();

    interface.set(pos!(7, 0), "with more");
    interface.apply().unwrap();

    assert_eq!(
        "Line 1 with more\nLine 2",
        &device.parser().screen().contents()
    );
}

#[test]
fn overlapping_writes() {
    let mut device = VirtualDevice::new();
    let mut interface = Interface::new_alternate(&mut device).unwrap();

    interface.set(pos!(0, 0), "ABCDEF");
    interface.apply().unwrap();

    interface.set(pos!(1, 0), "X");
    interface.apply().unwrap();

    interface.set(pos!(3, 0), "ZZ");
    interface.apply().unwrap();

    assert_eq!("AXCZZF", &device.parser().screen().contents());
}

#[test]
fn multiple_overlapping_formatted_writes() {
    let mut device = VirtualDevice::new();
    let mut interface = Interface::new_alternate(&mut device).unwrap();

    interface.set_styled(pos!(0, 0), "FIRST", Style::new().set_bold(true));
    interface.apply().unwrap();

    interface.set(pos!(2, 0), "SECOND");
    interface.apply().unwrap();

    interface.set_styled(
        pos!(4, 0),
        "THIRD",
        Style::new().set_italic(true).set_foreground(Color::Red),
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

    assert_eq!("FISETHIRD", &device.parser().screen().contents());

    for column in 0..expected_text.len() {
        let cell = device.parser().screen().cell(0, column as u16).unwrap();
        assert_eq!(expected_text[column], cell.contents());
        assert_eq!(expected_bold[column], cell.bold());
        assert_eq!(expected_italic[column], cell.italic());
        assert_eq!(expected_color[column], cell.fgcolor())
    }
}

#[test]
fn clearing_lines() {
    let mut device = VirtualDevice::new();
    let mut interface = Interface::new_alternate(&mut device).unwrap();

    interface.set(pos!(0, 0), "ABC");
    interface.set(pos!(0, 1), "DEF");
    interface.set(pos!(0, 2), "GHI");
    interface.apply().unwrap();

    interface.clear_line(1);
    interface.apply().unwrap();

    assert_eq!("ABC\n   \nGHI", &device.parser().screen().contents());
}

#[test]
fn clearing_rest_of_line() {
    let mut device = VirtualDevice::new();
    let mut interface = Interface::new_alternate(&mut device).unwrap();

    interface.set(pos!(0, 0), "ABC");
    interface.set(pos!(0, 1), "DEF");
    interface.set(pos!(0, 2), "GHI");
    interface.apply().unwrap();

    interface.clear_rest_of_line(pos!(1, 1));
    interface.apply().unwrap();

    assert_eq!("ABC\nD  \nGHI", &device.parser().screen().contents());
}

#[test]
fn clearing_rest_of_interface() {
    let mut device = VirtualDevice::new();
    let mut interface = Interface::new_alternate(&mut device).unwrap();

    interface.set(pos!(0, 0), "ABC");
    interface.set(pos!(0, 1), "DEF");
    interface.set(pos!(0, 2), "GHI");
    interface.apply().unwrap();

    interface.clear_rest_of_interface(pos!(1, 1));
    interface.apply().unwrap();

    assert_eq!("ABC\nD  \n   ", &device.parser().screen().contents());
}

#[test]
fn cursor_visible_after_exit_alternate() {
    let mut device = VirtualDevice::new();
    let interface = Interface::new_alternate(&mut device).unwrap();

    interface.exit().unwrap();

    let is_visible = !device.parser().screen().hide_cursor();
    assert!(is_visible);
}

#[test]
fn cursor_visible_after_exit_relative() {
    let mut device = VirtualDevice::new();
    let interface = Interface::new_relative(&mut device).unwrap();

    interface.exit().unwrap();

    let is_visible = !device.parser().screen().hide_cursor();
    assert!(is_visible);
}

#[test]
fn cursor_visible_after_exit_with_content() {
    let mut device = VirtualDevice::new();
    let mut interface = Interface::new_alternate(&mut device).unwrap();

    interface.set(pos!(0, 0), "Hello, world!");
    interface.apply().unwrap();

    interface.exit().unwrap();

    let is_visible = !device.parser().screen().hide_cursor();
    assert!(is_visible);
}
