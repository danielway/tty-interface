extern crate tty_interface;

use termion::{color, style};
use termion::raw::IntoRawMode;
use std::io::stdout;

use tty_interface::interface::TTYInterface;
use tty_interface::line::Line;
use tty_interface::segment::{Segment, SegmentFormatting};

fn main() -> tty_interface::result::Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut tty = TTYInterface::new(&mut stdout);

    for i in 1..=10 {
        let mut batch = tty.start_update();

        for j in 0..i {
            batch.set_line(j, Line {
                segments: vec![
                    Segment::new(format!("Line {}", i - j))
                ]
            });
        }

        tty.perform_update(batch)?;

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    let bold_red = SegmentFormatting::new(
        format!("{}{}", color::Fg(color::Red), style::Bold),
        format!("{}{}", color::Fg(color::Reset), style::NoBold)
    );

    let italic_blue = SegmentFormatting::new(
        format!("{}{}", color::Fg(color::Blue), style::Italic),
        format!("{}{}", color::Fg(color::Reset), style::NoItalic)
    );

    for i in 0..10 {
        let mut batch = tty.start_update();

        batch.set_line(i, Line {
            segments: vec![
                Segment::new_formatted(
                    format!("Line {}", i + 1),
                    if i % 2 == 0 { bold_red.clone() } else { italic_blue.clone() }
                )
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