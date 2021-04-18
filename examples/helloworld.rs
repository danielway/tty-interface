//! Demonstrates the simplest "Hello, world!" use-case for TTY Interface. This is a single-batch,
//! single-step, single-line, and single-segment interface with no direct dependency on Termion.

extern crate tty_interface;

use tty_interface::interface::TTYInterface;
use tty_interface::line::Line;
use tty_interface::segment::Segment;

fn main() -> tty_interface::result::Result<()> {
    // Initialize TTY Interface with stdout
    let mut stdout = std::io::stdout();
    let mut tty = TTYInterface::new(&mut stdout);

    // Start a batch which contains interface changes staged for display
    let mut batch = tty.start_update();

    // Add/stage setting a line of the interface to "Hello, world!"
    batch.set_line(0, Line::new(vec![
        Segment::new("Hello, world!".to_string())
    ]));

    // Apply the update to the interface, thereby pushing the changes to stdout
    tty.perform_update(batch)?;

    // End the session with TTY Interface which resets the terminal
    tty.end()?;

    Ok(())
}