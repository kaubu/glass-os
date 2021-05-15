use shell_words;

use crate::{error, info, input};

pub fn start() {
	loop {
		let command = input("$ ");
		let command = match shell_words::split(&command) {
			Err(e) => {
				error(&format!("Could not split command arguments. Details: {}", e));
				continue;
			},
		    Ok(a) => a,
		};

		info(&format!("Command args = {:#?}", command));
	}
}