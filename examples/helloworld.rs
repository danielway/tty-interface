extern crate tty_interface;

use tty_interface::interface::TTYInterface;
use tty_interface::line::Line;
use tty_interface::segment::Segment;

fn main() -> tty_interface::result::Result<()> {
    let mut stdout = std::io::stdout();
    let mut tty = TTYInterface::new(&mut stdout);

    let mut batch = tty.start_update();
    batch.set_line(0, Line {
        segments: vec![
            Segment::new("Hello, world!".to_string())
        ]
    });
    tty.perform_update(batch)?;

    tty.end()
}