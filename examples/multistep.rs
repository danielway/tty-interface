//! Demonstrates an interface update with multiple steps, including a step which updates a segment
//! already updated previously within the same batch. Omits any direct dependency on Termion.

extern crate tty_interface;

use tty_interface::interface::TTYInterface;
use tty_interface::line::Line;
use tty_interface::segment::Segment;

fn main() -> tty_interface::result::Result<()> {
    let mut stdout = std::io::stdout();
    let mut tty = TTYInterface::new(&mut stdout);

    let mut batch = tty.start_update();

    // Line 0: "Helo"
    batch.set_line(0, Line::new(vec![
        Segment::new("He".to_string()),
        Segment::new("lo".to_string()),
    ]));

    // Line 1: "world!"
    batch.set_line(1, Line::new(
        vec![ Segment::new("world!".to_string()) ]
    ));

    // Back to Line 0: "Hello" (updated the "lo" to "llo")
    batch.set_segment(0, 1, Segment::new("llo".to_string()));

    tty.perform_update(batch)?;

    tty.end()?;

    Ok(())
}