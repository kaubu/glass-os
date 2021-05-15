use phf::phf_map;

pub const HELP_MESSAGE: &str = "[GlassOS Help Menu]
help [command ...]\tDisplay information about builtin commands
echo [args ...]\t\tDisplay a line of text
cd [dir]\t\tChange the working directory
ls [dir ...]\t\tList directory contents
mkdir [dir]\t\tMake directories
rmdir [dir]\t\tRemove empty directories
quit\t\t\tShuts down GlassOS";

pub const COMMANDS_HELP: phf::Map<&'static str, &'static str> = phf_map! {
	"help" => "Usage: help [command ...]
Lists all commands if no command is provided.
If a command is provided, it will print detailed usage on that specific command.

Example: help
Example: help quit",
	"echo" => "Usage: echo [args ...]
Prints all arguments to the screen.

Example: echo Hello World!",
	"quit" => "Usage: quit
Shutsdown GlassOS.

Example: quit",
	"cd" => "Usage: cd [dir]
Changes directory to the first argument given.
Use .. to go up a directory.

Example: cd Videos/
Example: cd ../Documents",
	"ls" => "Usage: ls [dir ...]
Lists all files and directories in current directory, or in the relative directory in argument.
The relative directory is optional, and ls can be used on it's own.

Example: ls
Example: ls src",
	"mkdir" => "Usage: mkdir [dir]
Creates directories recursively.
Does not create a directory if it already exists.

Example: mkdir test
Example: mkdir code/src",
	"rmdir" => "Usage: rmdir [dir]
Removes empty directories.
Can not remove directories containing files, to do that use 'rmall'.

Example: rmdir test
Example: rmdir code/src",
};

pub const DEFAULT_DIR: &str = "./";