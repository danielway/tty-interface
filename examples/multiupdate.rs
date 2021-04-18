//! Demonstrates performing a variety of updates on an interface including inserts, updates,
//! deletions, and the interactions between these changes. The changes are animated and include
//! formatting encoded by Termion.

extern crate tty_interface;

use termion::{color, style};

use tty_interface::interface::TTYInterface;
use tty_interface::line::Line;
use tty_interface::segment::{Segment, SegmentFormatting};

fn main() -> tty_interface::result::Result<()> {
    let mut stdout = std::io::stdout();
    let mut tty = TTYInterface::new(&mut stdout);

    // Display "Line 1", "Line 2", ..., "Line 10" in reverse order (inserting at the top and
    //      pushing existing lines down) with separate updates to animate the changes
    for i in 1..=10 {
        let mut batch = tty.start_update();

        for j in 0..i {
            batch.set_line(j, Line::new(vec![
                Segment::new(format!("Line {}", i - j))
            ]));
        }

        tty.perform_update(batch)?;

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // Configure a bold-red and italic-blue format for later use
    let bold_red = SegmentFormatting::new(
        format!("{}{}", color::Fg(color::Red), style::Bold),
        format!("{}{}", color::Fg(color::Reset), style::NoBold)
    );
    let italic_blue = SegmentFormatting::new(
        format!("{}{}", color::Fg(color::Blue), style::Italic),
        format!("{}{}", color::Fg(color::Reset), style::NoItalic)
    );

    // Display "Line 1", "Line 2", ..., "Line 10" in reverse order (updating from the bottom to the
    //      top) and with alternating formatting (bold-red, italic-blue)
    for i in 0..10 {
        let mut batch = tty.start_update();

        batch.set_line(i, Line::new(vec![
            Segment::new_formatted(
                format!("Line {}", i + 1),
                if i % 2 == 0 { bold_red.clone() } else { italic_blue.clone() }
            )
        ]));

        tty.perform_update(batch)?;

        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    // Delete each line from bottom to top
    for i in 0..10 {
        let mut batch = tty.start_update();

        batch.delete_line(9 - i);

        tty.perform_update(batch)?;

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    tty.end()?;

    Ok(())
}