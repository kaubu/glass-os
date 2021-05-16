use phf::phf_map;

pub const HELP_MESSAGE: &str = "[GlassOS Help Menu]
help [command ...]\tDisplay information about builtin commands
echo [args]\t\tDisplay a line of text
cd [dir]\t\tChange the working directory
ls [dir ...]\t\tList directory contents
mkdir [dir]\t\tMake directories
rmdir [dir]\t\tRemove empty directories
rm [file]\t\tRemove files
clear [args ...]\tClears the screen
quit\t\t\tShuts down GlassOS";

pub const COMMANDS_HELP: phf::Map<&'static str, &'static str> = phf_map! {
	"help" => "Usage: help [command ...]
Lists all commands if no command is provided.
If a command is provided, it will print detailed usage on that specific command.

Examples:
help
help quit",
	"echo" => "Usage: echo [args ...]
Prints all arguments to the screen.

Example: echo Hello World!",
	"quit" => "Usage: quit
Shutsdown GlassOS.
It's important to note that you can not exit using Ctrl-C, as it doesn't restore the main terminal,
and instead you must use this command.

Examples:
quit",
	"cd" => "Usage: cd [dir]
Changes directory to the first argument given.
Use .. to go up a directory.

Examples:
cd Videos/
cd ../Documents",
	"ls" => "Usage: ls [dir ...]
Lists all files and directories in current directory, or in the relative directory in argument.
The relative directory is optional, and ls can be used on it's own.

Examples:
ls
ls src",
	"mkdir" => "Usage: mkdir [dir]
Creates directories recursively.
Does not create a directory if it already exists.

Examples:
mkdir test
mkdir code/src",
	"rmdir" => "Usage: rmdir [dir]
Removes empty directories.
Can not remove directories containing files, to do that use 'rmall'.

Examples:
rmdir test
rmdir code/src",
	"rm" => "Usage: rm [file]
Removes files.

Examples:
rm passwords.txt
rm src/colors/Color.h",
	"clear" => "Usage: clear [args]
Clears the screen.

Args:
\t-r, --reset\tAfter clearing the screen, moves the cursor up to the top

Examples:
clear
clear --reset",
};

pub const DEFAULT_DIR: &str = "./";