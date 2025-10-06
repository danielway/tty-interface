use std::io::{Stdout, stdout};

use tty_interface::{Interface, Position, Result, pos};

fn main() {
    execute().expect("execute basic example");
}

fn execute() -> Result<()> {
    let mut device: Stdout = stdout();
    let mut interface = Interface::new_relative(&mut device)?;

    interface.set(pos!(0, 0), "Hello,");
    interface.set(pos!(7, 0), "world!");
    interface.apply()?;

    interface.exit()?;

    Ok(())
}
