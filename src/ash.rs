use std::path::PathBuf;

use shell_words;

use crate::{consts::{self, COMMANDS_HELP, HELP_MESSAGE}, cursor, debug, error, input};

fn echo(argv: &Vec<String>) {
	let mut args = argv.clone();
	args.remove(0);
	println!("{}", args.join(" "));
}

pub fn start(username: &str, pc_name: &str) {
	let mut current_dir = PathBuf::from("./");

	loop {
		let commands = cursor(&format!("[{}@{} {}]$ ", username, pc_name, current_dir.display()));
		let commands = match shell_words::split(&commands) {
			Err(e) => {
				error(&format!("Could not split command arguments. Details: {}", e));
				continue;
			},
		    Ok(a) => a,
		};

		debug(&format!("commands args = {:#?}", commands));
		debug(&format!("len = {}", commands.len()));

		let command_len = commands.len();
		if command_len <= 0 { continue; } // If no command was entered
		let command = &commands[0];

		if command == "help" {
			if command_len >= 2 {
				let help_command = &commands[1];
				let help_command = help_command.as_str();
	
				if COMMANDS_HELP.contains_key(help_command) {
					println!("{}", COMMANDS_HELP[help_command]);
				}
			} else {
				println!("{}", HELP_MESSAGE);
			}
		} else if command == "echo" {
			echo(&commands);
		} else if command == "quit" {
			break;
		}
	}
}