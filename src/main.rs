use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod ash;
mod consts;

const DEFAULT_CONSOLE_COLOR: Color = Color::White;
const CONSOLE_CURSOR_COLOR: Color = Color::Green;

const ERROR_MESSAGES: bool = true;
const DEBUG_MESSAGES: bool = true;
const SUCCESS_MESSAGES: bool = true;

fn main() {
	ash::start("me", "mypc");
}

pub fn input(msg: &str) -> String {
	print!("{}", msg);
	let mut buf: String = String::new();

	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut buf).unwrap();

	buf.trim().to_string()
}

fn _color_print(msg: &str, color: Color, newline: bool) {
	let mut stdout = StandardStream::stdout(ColorChoice::Always);
	stdout.set_color(ColorSpec::new().set_fg(Some(color))).expect("error: Could not set foreground color of text");
	if newline {
		writeln!(&mut stdout, "{}", msg).expect("error: Could not write colored text");
	} else {
		write!(&mut stdout, "{}", msg).expect("error: Could not write colored text");
	}
	stdout.set_color(ColorSpec::new().set_fg(Some(DEFAULT_CONSOLE_COLOR))).expect("error: Could not set foreground color of text");
}

fn color_println(msg: &str, color: Color) { _color_print(msg, color, true); }
fn color_print(msg: &str, color: Color) { _color_print(msg, color, false); }

pub fn error(msg: &str) { if ERROR_MESSAGES { color_println(&format!("error: {}", msg), Color::Red); } }
pub fn debug(msg: &str) { if DEBUG_MESSAGES { color_println(&format!("debug: {}", msg), Color::Yellow); } }
pub fn success(msg: &str) { if SUCCESS_MESSAGES { color_println(&format!("success: {}", msg), Color::Green) } }

pub fn cursor(msg: &str) -> String {
	color_print(msg, CONSOLE_CURSOR_COLOR);
	input("")
}