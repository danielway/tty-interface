use std::io::{stdout, Stdout};

use messages::render_messages_gradually;
use tty_interface::{Interface, Result};

mod messages;

fn main() {
    execute().expect("execute alternate example");
}

fn execute() -> Result<()> {
    let mut device: Stdout = stdout();
    let mut interface = Interface::new_alternate(&mut device)?;

    let messages = [
        "This is an example of an interface which renders in an alternate screen.",
        "",
        "An alternate screen is a separate buffer, so this interface's contents",
        "have no impact on the buffer it was invoked from.",
        "",
        "When this interface exits, its contents will completely disappear and",
        "focus will be returned to the previous, unmodified buffer.",
    ];

    render_messages_gradually(&mut interface, &messages)?;
    interface.exit()?;

    Ok(())
}
