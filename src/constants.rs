//Tester
pub const HELLO: &str = "Hello";

//The Commands
pub const PRINT_DIR: &str = "printdir";    // lists contents of given directory
pub const QUIT: &str = "quit";             // quits the file-client using exit()
pub const PRINT_HIDDEN: &str = "printhidden";       // lists all hidden (.) files and directories
pub const HELP: &str = "help";                 // lists all possible file operations/commands
pub const SEARCH: &str = "search";
//TODO Commands:
    // SEARCH          - "search"  ---- searches files' content and filenames that match the given search input

//Ignore List
pub const HIDDEN_IGNORE: &'static [&'static str] = &[".", ".git", ".workflows", ".gitignore"];
