use std::io::{stdout, Stdout};

use common::render_messages_gradually;
use tty_interface::{Interface, Result};

mod common;

fn main() {
    execute().expect("execute relative example");
}

fn execute() -> Result<()> {
    let mut device: Stdout = stdout();
    let mut interface = Interface::new_relative(&mut device)?;

    let messages = [
        "This is an example of an interface which renders relatively.",
        "",
        "A relative interface is rendered from the bottom of the buffer,",
        "without disturbing the buffer's previous contents.",
        "",
        "When this interface exits, its contents will remain present",
        "in the buffer's history.",
    ];

    render_messages_gradually(&mut interface, &messages)?;
    interface.exit()?;

    Ok(())
}
