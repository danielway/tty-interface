extern crate tty_interface;

use tty_interface::interface::TTYInterface;
use termion::raw::IntoRawMode;
use std::io::stdout;
use tty_interface::line::Line;
use tty_interface::segment::Segment;

fn main() -> tty_interface::result::Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut tty = TTYInterface::new();

    let mut batch = tty.start_update();
    batch.set_line(0, Line {
        segments: vec![
            Segment { text: "He".to_string() },
            Segment { text: "lo".to_string() }
        ]
    });
    batch.set_line(1, Line {
        segments: vec![
            Segment { text: "world!".to_string() }
        ]
    });
    batch.set_segment(0, 1, Segment { text: "llo".to_string() });
    tty.perform_update(&mut stdout, batch)?;

    tty.end(&mut stdout)
}