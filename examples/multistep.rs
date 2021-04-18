extern crate tty_interface;

use tty_interface::interface::TTYInterface;
use termion::raw::IntoRawMode;
use std::io::stdout;
use tty_interface::line::Line;
use tty_interface::segment::Segment;

fn main() -> tty_interface::result::Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut tty = TTYInterface::new(&mut stdout);

    let mut batch = tty.start_update();
    batch.set_line(0, Line {
        segments: vec![
            Segment::new("He".to_string()),
            Segment::new("lo".to_string()),
        ]
    });
    batch.set_line(1, Line {
        segments: vec![
            Segment::new("world!".to_string())
        ]
    });
    batch.set_segment(0, 1, Segment::new("llo".to_string()));
    tty.perform_update(batch)?;

    tty.end()?;

    Ok(())
}