use std::{thread::sleep, time::Duration};

use tty_interface::{pos, Interface, Position, Result};

pub fn render_messages_gradually(interface: &mut Interface, messages: &[&str]) -> Result<()> {
    for line in 1..=messages.len() {
        render_messages(interface, &messages[0..line])?;
        sleep(Duration::from_millis(500));
    }

    sleep(Duration::from_secs(2));

    Ok(())
}

fn render_messages(interface: &mut Interface, messages: &[&str]) -> Result<()> {
    let message_lengths = messages.iter().map(|line| line.len());
    let longest_message = message_lengths.max().expect("should have longest message");

    let horizontal_line = format!("+{}+", "-".repeat(longest_message));

    interface.set(pos!(0, 0), &horizontal_line);
    for (index, message) in messages.iter().enumerate() {
        let padding = longest_message - message.len();
        let line = format!("|{}{}|", message, " ".repeat(padding));
        interface.set(pos!(0, index as u16 + 1), &line);
    }
    interface.set(pos!(0, messages.len() as u16 + 1), &horizontal_line);

    interface.apply()?;

    Ok(())
}
