//Tester
pub const HELLO: &str = "Hello";

//The Commands
pub const PRINT_DIR: &str = "printdir";         // lists contents of given directory
pub const QUIT: &str = "quit";                  // quits the file-client using exit()
pub const PRINT_HIDDEN: &str = "printhidden";   // lists all hidden (.) files and directories
pub const HELP: &str = "help";                  // lists all possible file operations/commands
pub const READ: &str = "read";                  // reads a file from provided path
pub const WRITE: &str = "write";                // writes a file from a provided local to a provided server path
pub const MOVE: &str = "move";                  // moves a file from a provided path to a provided new path
pub const COPY: &str = "copy";                  // copys a file from a provided path to a provided new path
pub const DELETE: &str = "del";                 // deletes a file from a provided path
pub const MAKE_DIR: &str = "mkdir";             // makes directories to a provided path
pub const REMOVE_DIR: &str = "rmdir";           // removes a directory & all contents from a provided path
pub const SEARCH: &str = "search";              // searches for files or directories containing a specified term

//Ignore Lists
pub const HIDDEN_IGNORE: &'static [&'static str] = &[".", ".git", ".workflows", ".gitignore"]; // ignore list for 
