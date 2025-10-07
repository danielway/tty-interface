use std::io::{Stdout, stdout};

use rand::rngs::ThreadRng;
use tty_interface::{Color, Interface, Position, Result, Style, pos};

fn main() {
    execute().expect("execute basic example");
}

fn execute() -> Result<()> {
    let mut device: Stdout = stdout();
    let mut interface = Interface::new_relative(&mut device)?;

    interface.set(pos!(0, 0), "Here's the alphabet formatted randomly:");

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rand = rand::rng();

    for line in 1..=10 {
        for (col, ch) in alphabet.chars().enumerate() {
            interface.set_styled(
                pos!(col as u16, line),
                &ch.to_string(),
                get_random_style(&mut rand),
            );
        }
    }

    interface.apply()?;

    interface.exit()?;

    Ok(())
}

fn get_random_style(rand: &mut ThreadRng) -> Style {
    let mut style = Style::new();

    style = style.set_bold(rand::Rng::random_bool(rand, 0.5));
    style = style.set_italic(rand::Rng::random_bool(rand, 0.5));
    style = style.set_underline(rand::Rng::random_bool(rand, 0.5));

    style = style.set_foreground(get_random_color(rand));
    style = style.set_background(get_random_color(rand));

    style
}

fn get_random_color(rand: &mut ThreadRng) -> Color {
    match rand::Rng::random_range(rand, 0..17) {
        0 => Color::Black,
        1 => Color::DarkGrey,
        2 => Color::Red,
        3 => Color::DarkRed,
        4 => Color::Green,
        5 => Color::DarkGreen,
        6 => Color::Yellow,
        7 => Color::DarkYellow,
        8 => Color::Blue,
        9 => Color::DarkBlue,
        10 => Color::Magenta,
        11 => Color::DarkMagenta,
        12 => Color::Cyan,
        13 => Color::DarkCyan,
        14 => Color::White,
        15 => Color::Grey,
        _ => Color::Reset,
    }
}
