use std::{
    io::{stdout, Stdout},
    thread::sleep,
    time::Duration,
};

use tty_interface::{pos, Color, Interface, Position, Result, Style};

fn main() {
    execute().expect("execute basic example");
}

fn execute() -> Result<()> {
    let mut device: Stdout = stdout();
    let mut interface = Interface::new(&mut device)?;

    interface.set(pos!(0, 0), "Hello, world!");
    interface.set_styled(
        pos!(5, 2),
        "Let's count 0-9:",
        Style::default().set_italic(true),
    );
    interface.apply()?;

    for i in 0..10 {
        interface.set_styled(
            pos!(10, 3 + i),
            &i.to_string(),
            Style::default().set_bold(true).set_foreground(Color::Red),
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
        interface.set_cursor(Some(pos!(10, 3 + j)));
        interface.clear_line(3 + j);
        interface.apply()?;
        sleep(Duration::from_millis(100));
    }

    interface.exit()?;

    Ok(())
}
