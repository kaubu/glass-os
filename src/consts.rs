use phf::phf_map;

pub const HELP_MESSAGE: &str = "[GlassOS Help Menu]
help\t\t\tThis command. Lists all available commands
help [COMMAND]\t\tGets detailed usage on a specific command
echo [ARGS]\t\tDisplays arguments to the screen
quit\t\t\tShuts down GlassOS";

// TODO: Add [HELP: {command}] before printing this out
pub const COMMANDS_HELP: phf::Map<&'static str, &'static str> = phf_map! {
	"help" => "Usage: help [command ...]
Lists all commands if no command is provided.
If a command is provided, it will print detailed usage on that specific command.",
	"echo" => "Usage: echo [args ...]
Prints all arguments to the screen.",
	"quit" => "Usage: quit
Shutsdown GlassOS.",
};