use std::io::{stdout, Stdout};

use tty_interface::{pos, Interface, Position, Result};

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
