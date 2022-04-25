//The file storage directory
pub const SERVER_FILE_STORAGE: &str = "ServerFileStorage";

//The Commands
pub const PRINT_DIR: &str = "printdir";
pub const QUIT: &str = "quit";
pub const PRINT_HIDDEN: &str = "printhidden";
pub const MAKE_DIR: &str = "mkdir";  









//Matthew Constants
//Root Folder
pub const ROOT: &str = "file_root/";

//The Commands
pub const HELP: &str = "help";                  // lists all possible file operations/commands
pub const READ: &str = "read";                  // reads a file from provided path
pub const WRITE: &str = "write";                // writes a file from a provided local to a provided server path
pub const MOVE: &str = "move";                  // moves a file from a provided path to a provided new path
pub const COPY: &str = "copy";                  // copys a file from a provided path to a provided new path
pub const DELETE: &str = "del";                 // deletes a file from a provided path
           // makes directories to a provided path
pub const REMOVE_DIR: &str = "rmdir";           // removes a directory & all contents from a provided path
pub const SEARCH: &str = "search";              // searches for files or directories containing a specified term

//Ignore Lists
pub const HIDDEN_IGNORE: &'static [&'static str] = &[".", ".git", ".workflows", ".gitignore"]; // ignore list for 