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

    for i in 1..=10 {
        let mut batch = tty.start_update();

        for j in 0..i {
            batch.set_line(j, Line {
                segments: vec![
                    Segment { text: format!("Line {}", i - j) }
                ]
            });
        }

        tty.perform_update(batch)?;

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    for i in 0..10 {
        let mut batch = tty.start_update();

        batch.set_line(i, Line {
            segments: vec![
                Segment { text: format!("Line {}", i + 1) }
            ]
        });

        tty.perform_update(batch)?;

        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    for i in 0..10 {
        let mut batch = tty.start_update();
        batch.delete_line(9 - i);
        tty.perform_update(batch)?;

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    tty.end()?;

    Ok(())
}