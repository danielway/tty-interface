use std::{thread::sleep, time::Duration};

use tty_interface::{pos, Interface, Position, Result, Style, Color};

fn main() {
    execute().expect("execute basic example");
}

fn execute() -> Result<()> {
    let mut interface = Interface::for_stdout()?;

    interface.set(pos!(0, 0), "Hello, world!");
    interface.set_styled(pos!(5, 2), "Let's count to 10:", Style::default().set_italic(true));
    interface.apply()?;

    for i in 1..=10 {
        interface.set_styled(pos!(10, 3), &i.to_string(), Style::default().set_bold(true).set_foreground(Color::Red));
        interface.apply()?;
        sleep(Duration::from_millis(250));
    }

    interface.exit()?;

    Ok(())
}
