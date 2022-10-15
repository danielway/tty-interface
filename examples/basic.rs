use std::{thread::sleep, time::Duration};

use tty_interface::{Interface, Result, Position, pos};

fn main() {
    execute().expect("execute basic example");
}

fn execute() -> Result<()> {
    let mut interface = Interface::for_stdout()?;

    interface.set(pos!(0, 0), "Hello, world!");
    interface.set(pos!(5, 2), "Let's count to 10:");
    interface.apply()?;

    for i in 1..=10 {
        interface.set(pos!(10, 3), &i.to_string());
        interface.apply()?;
        sleep(Duration::from_millis(250));
    }

    interface.exit()?;

    Ok(())
}