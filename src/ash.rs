use std::path::PathBuf;
use shell_words;
use crate::{consts::{COMMANDS_HELP, DEFAULT_DIR, HELP_MESSAGE}, cursor, error};

fn echo(argv: &Vec<String>) {
	let mut args = argv.clone();
	args.remove(0);
	println!("{}", args.join(" "));
}

fn fwd(cd: &PathBuf) -> String {
	match cd.strip_prefix(".") {
		Ok(c) => c.display().to_string(),
		Err(_e) => {
			// error(&format!("Failed to strip prefix. Details: {}", _e)); // Happens when trying to cd .. multiple times
			String::from("")
		},
	}
}

fn pwd(cd: &PathBuf) {
	println!("/{}", fwd(cd));
}

fn cd(current_dir: &mut PathBuf, commands: Vec<String>) {
	if commands.len() < 2 { return; }
	
	let default_dir = PathBuf::from(DEFAULT_DIR);
	
	if commands[1] == "/" {
		*current_dir = default_dir;
	} else if commands[1] != "." { // Ignore "cd ."
		let mut commands = commands.clone();
		commands.remove(0);
		
		for dir in commands[0].split("/").collect::<Vec<&str>>() {
			current_dir.push(dir);
			
			if dir == ".." {
				current_dir.pop(); // Gets rid of ..
				if current_dir.as_path() == default_dir.as_path() {
					// Gets rid of .., but does not get rid of directory above, because it's at the root
					error("Can not go above root directory");
					return;
				}
				
				current_dir.pop(); // Gets rid of the dir above
			// If it's not a directory, a file, or nothing
			} else if current_dir.is_file() {
				current_dir.pop();
				error(&format!("'{}' is a file, not a directory", dir));
			} else if !current_dir.is_dir() && dir != "" {
				error(&format!("The directory '{}' does not exit", dir));
				current_dir.pop();
				return;
			}
		}
	}
}

fn ls(current_dir: &PathBuf) {
	if current_dir.is_dir() {
		for entry in current_dir.read_dir().expect("error: Reading directory failed") {
			if let Ok(entry) = entry {
				let entry_name = entry.path();
				let entry_name = entry_name.file_name().unwrap().to_string_lossy();
				if entry.path().is_dir() {
					println!("{}/", entry_name);
				} else {
					println!("{}", entry_name);
				}
			}
		}
	}
}

pub fn start(username: &str, pc_name: &str) {
	let mut current_dir = PathBuf::from(DEFAULT_DIR);

	loop {
		let commands = cursor(&format!("[{}@{} /{}]$ ", username, pc_name, fwd(&current_dir)));
		let commands = match shell_words::split(&commands) {
			Err(e) => {
				error(&format!("Could not split command arguments. Details: {}", e));
				continue;
			},
		    Ok(a) => a,
		};

		let commands_len = commands.len();
		if commands_len <= 0 { continue; } // If no command was entered
		let command = &commands[0];

		if command == "help" {
			if commands_len >= 2 {
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
		} else if command == "pwd" {
			pwd(&current_dir);
		} else if command == "cd" {
			if commands_len >= 2 {
				cd(&mut current_dir, commands);
			}
		} else if command == "ls" {
			if commands_len == 1 {
				ls(&current_dir);
			} else if commands_len >= 2 {
				let mut temp_dir = current_dir.clone();
				cd(&mut temp_dir, commands);
				ls(&temp_dir);
			}
		}
	}
}