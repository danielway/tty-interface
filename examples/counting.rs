use std::{
    io::{Stdout, stdout},
    thread::sleep,
    time::Duration,
};

use tty_interface::{Color, Interface, Position, Result, Style, pos};

fn main() {
    execute().expect("execute counting example");
}

fn execute() -> Result<()> {
    let mut device: Stdout = stdout();
    let mut interface = Interface::new_relative(&mut device)?;

    interface.set(pos!(0, 0), "Hello, world!");
    interface.set_styled(
        pos!(5, 2),
        "Let's count 0-9:",
        Style::new().set_italic(true),
    );
    interface.apply()?;

    for i in 0..10 {
        interface.set_styled(
            pos!(10, 3 + i),
            &i.to_string(),
            Style::new().set_bold(true).set_foreground(Color::Red),
        );

        if i % 2 == 0 {
            interface.set_cursor(Some(pos!(10, 3 + i)));
        } else {
            interface.set_cursor(None);
        }

        interface.apply()?;
        sleep(Duration::from_millis(250));
    }

    for i in 0..10 {
        let j = 9 - i;

        if i % 2 == 0 {
            interface.clear_line(3 + j);
        }

        interface.set_cursor(Some(pos!(10, 3 + j)));
        interface.apply()?;

        sleep(Duration::from_millis(100));
    }

    interface.exit()?;

    Ok(())
}
