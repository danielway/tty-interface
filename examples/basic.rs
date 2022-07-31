use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use tty_interface::config::Configuration;
use tty_interface::format::{Color, Style};
use tty_interface::line::LineId;
use tty_interface::mode::{CursorMode, RenderMode};
use tty_interface::position::{Position, RelativePosition};
use tty_interface::segment::SegmentId;
use tty_interface::{Interface, Result};

fn main() {
    run().expect("Run basic example");
}

fn run() -> Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;

    let config = Configuration::new(CursorMode::Relative, RenderMode::Relative);
    let mut interface = Interface::new_with_configuration(&mut stdout, config)?;

    let (first_line_id, hello_segment_id) = first_render(&mut interface)?;
    sleep(Duration::from_secs(1));

    let (second_line_id, help_segment_id) =
        second_render(&mut interface, &first_line_id, &hello_segment_id)?;
    sleep(Duration::from_secs(2));

    third_render(
        &mut interface,
        &first_line_id,
        &hello_segment_id,
        &second_line_id,
        &help_segment_id,
    )?;
    sleep(Duration::from_secs(2));

    interface.advance_to_end()?;

    Ok(())
}

fn first_render(interface: &mut Interface) -> Result<(LineId, SegmentId)> {
    let first_line = interface.add_line();
    let first_line_id = first_line.identifier();

    let hello_segment = first_line.add_segment();
    let hello_segment_id = hello_segment.identifier();

    hello_segment.set_text("Hello, wrld!");

    interface.apply_changes()?;
    Ok((first_line_id, hello_segment_id))
}

fn second_render(
    interface: &mut Interface,
    first_line_id: &LineId,
    hello_segment_id: &SegmentId,
) -> Result<(LineId, SegmentId)> {
    let first_line = interface.get_line_mut(first_line_id)?;
    let hello_segment = first_line.get_segment_mut(hello_segment_id)?;
    hello_segment.set_color(Color::Red);
    hello_segment.set_styles(vec![Style::Italic]);

    let second_line = interface.add_line();
    let second_line_id = second_line.identifier();

    let help_segment = second_line.add_segment();
    let help_segment_id = help_segment.identifier();

    help_segment.set_text("Oops, typo! ^ Let's fix it.");
    help_segment.set_styles(vec![Style::Underline]);

    interface.set_cursor(Position::Relative(RelativePosition::new(
        *first_line_id,
        *hello_segment_id,
        8,
    )));

    interface.apply_changes()?;
    Ok((second_line_id, help_segment_id))
}

fn third_render(
    interface: &mut Interface,
    first_line_id: &LineId,
    hello_segment_id: &SegmentId,
    second_line_id: &LineId,
    help_segment_id: &SegmentId,
) -> Result<()> {
    let first_line = interface.get_line_mut(first_line_id)?;
    let hello_segment = first_line.get_segment_mut(hello_segment_id)?;
    hello_segment.set_text("Hello, world!");
    hello_segment.reset_color();
    hello_segment.set_styles(vec![Style::Bold]);

    let second_line = interface.get_line_mut(second_line_id)?;
    let help_segment = second_line.get_segment_mut(help_segment_id)?;
    help_segment.set_text("Fixed! That's better.");
    help_segment.reset_color();
    help_segment.reset_styles();

    interface.set_cursor(Position::Relative(RelativePosition::new(
        *first_line_id,
        *hello_segment_id,
        13,
    )));

    interface.apply_changes()?;

    Ok(())
}
