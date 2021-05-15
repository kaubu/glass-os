use std::path::PathBuf;
use std::fs;
use shell_words;
use termcolor::StandardStream;
use crate::{consts::{COMMANDS_HELP, DEFAULT_DIR, HELP_MESSAGE}, cursor, error, success};

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

fn cd(current_dir: &mut PathBuf, commands: Vec<String>, screen: &mut StandardStream) -> bool {
	if commands.len() < 2 { return true; }
	
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
					error("Can not go above root directory", screen);
					return true;
				}
				
				current_dir.pop(); // Gets rid of the dir above
			// If it's not a directory, a file, or nothing
			} else if current_dir.is_file() {
				current_dir.pop();
				error(&format!("'{}' is a file, not a directory", dir), screen);
			} else if !current_dir.is_dir() && dir != "" {
				error(&format!("The directory '{}' does not exist", dir), screen);
				current_dir.pop();
				return false;
			}
		}
	}

	true
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

pub fn start(username: &str, pc_name: &str, screen: &mut StandardStream) {
	let mut current_dir = PathBuf::from(DEFAULT_DIR);

	

	loop {
		let commands = cursor(&format!("[{}@{} /{}]$ ", username, pc_name, fwd(&current_dir)), screen);
		let commands = match shell_words::split(&commands) {
			Err(e) => {
				error(&format!("Could not split command arguments. Details: {}", e), screen);
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
				cd(&mut current_dir, commands, screen);
			}
		} else if command == "ls" {
			if commands_len == 1 {
				ls(&current_dir);
			} else if commands_len >= 2 {
				let mut temp_dir = current_dir.clone();
				if cd(&mut temp_dir, commands, screen) {
					ls(&temp_dir);
				}
			}
		} else if command == "mkdir" {
			if commands_len >= 2 {
				let dir_name = &commands[1];
				let mut temp_dir = current_dir.clone();

				temp_dir.push(dir_name);

				if !temp_dir.is_dir() {
					match fs::create_dir_all(&temp_dir) {
					    Ok(_) => success(&format!("Successfully created directory '{}'", dir_name), screen),
					    Err(e) => error(&format!("Failed to create directory. Details: {}", e), screen),
					}
				} else {
					error(&format!("The directory '{}' already exists", dir_name), screen);
				}
			}
		} else if command == "rmdir" {
			if commands_len >= 2 {
				let dir_name = &commands[1];
				let mut temp_dir = current_dir.clone();

				temp_dir.push(dir_name);

				if temp_dir.is_dir() {
					let is_empty = temp_dir.read_dir().unwrap().next().is_none();

					if !is_empty {
						error(&format!("The directory '{}' is not empty", dir_name), screen);
						continue;
					}

					match fs::remove_dir(&temp_dir) {
					    Ok(_) => success(&format!("Successfully removed empty directory '{}'", dir_name), screen),
					    Err(e) => error(&format!("Failed to remove directory. Details: {}", e), screen),
					}
				} else {
					error(&format!("The directory '{}' does not exist", dir_name), screen);
				}
			}
		}
	}
}