use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ctrlc;
use termion::screen::AlternateScreen;

mod ash;
mod consts;

const DEFAULT_CONSOLE_COLOR: Color = Color::White;
const CONSOLE_CURSOR_COLOR: Color = Color::Green;

const ERROR_MESSAGES: bool = true;
const DEBUG_MESSAGES: bool = true;
const SUCCESS_MESSAGES: bool = true;

fn main() {
	let mut screen = AlternateScreen::from(
		StandardStream::stdout(ColorChoice::Always));

	// Ctrl+C doesn't mix well with alternative screens
	// because it doesn't restore the previous screen
	// so it advises the user to use the quit command
	// to exit properly, and restore the main screen
	ctrlc::set_handler(move || {
		println!("\nPlease type the command 'quit' to shutdown GlassOS.
Press <Enter> to continue.");
	}).expect("error: Could not set Ctrl-C handler");

	ash::start("me", "mypc", &mut screen);
}

pub fn input(msg: &str) -> String {
	print!("{}", msg);
	let mut buf: String = String::new();

	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut buf).unwrap();

	buf.trim().to_string()
}

fn _color_print(msg: &str, color: Color, newline: bool, screen: &mut StandardStream) {
	screen.set_color(ColorSpec::new().set_fg(Some(color))).expect("error: Could not set foreground color of text");
	if newline {
		writeln!(screen, "{}", msg).expect("error: Could not write colored text");
	} else {
		write!(screen, "{}", msg).expect("error: Could not write colored text");
	}
	screen.set_color(ColorSpec::new().set_fg(Some(DEFAULT_CONSOLE_COLOR))).expect("error: Could not set foreground color of text");
}

fn color_println(msg: &str, color: Color, screen: &mut StandardStream) { _color_print(msg, color, true, screen); }
fn color_print(msg: &str, color: Color, screen: &mut StandardStream) { _color_print(msg, color, false, screen); }

pub fn error(msg: &str, screen: &mut StandardStream) {
	if ERROR_MESSAGES { color_println(&format!("error: {}", msg), Color::Red, screen); }
}

pub fn debug(msg: &str, screen: &mut StandardStream) {
	if DEBUG_MESSAGES { color_println(&format!("debug: {}", msg), Color::Yellow, screen); }
}

pub fn success(msg: &str, screen: &mut StandardStream) {
	if SUCCESS_MESSAGES { color_println(&format!("success: {}", msg), Color::Green, screen) }
}

pub fn cursor(msg: &str, screen: &mut StandardStream) -> String {
	color_print(msg, CONSOLE_CURSOR_COLOR, screen);
	input("")
}