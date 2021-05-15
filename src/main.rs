use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub mod ash;

const DEFAULT_CONSOLE_COLOR: Color = Color::White;

fn main() {
	ash::start();
}

pub fn input(msg: &str) -> String {
	print!("{}", msg);
	let mut buf: String = String::new();

	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut buf).unwrap();

	buf.trim().to_string()
}

fn color_print(msg: &str, color: Color) {
	let mut stdout = StandardStream::stdout(ColorChoice::Always);
	stdout.set_color(ColorSpec::new().set_fg(Some(color))).expect("error: Could not set foreground color of text");
	writeln!(&mut stdout, "{}", msg).expect("error: Could not write colored text");
	stdout.set_color(ColorSpec::new().set_fg(Some(DEFAULT_CONSOLE_COLOR))).expect("error: Could not set foreground color of text");
}

pub fn error(msg: &str) { color_print(&format!("error: {}", msg), Color::Red); }
pub fn info(msg: &str) { color_print(&format!("info: {}", msg), Color::Yellow); }

