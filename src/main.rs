use std::fs::ReadDir;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;
use std::process::exit;

// used for hidden dir file op
use walkdir::DirEntry as WalkDirEntry;
use walkdir::WalkDir;
use colored::Colorize;

// Commands the client can use
const PRINT_DIR: &str = "printdir";        // lists contents of given directory
const PRINT_HIDDEN: &str = "ls -al";       // lists all hidden (.) files and directories
const QUIT: &str = "quit";                 // quits the file-client using exit()
const HELP: &str = "help";                 // lists all possible file operations/commands

/*
    TODO Commands:
    SEARCH          - "search"  ---- searches files' content and filenames that match the given search input
 */

 // returns true if file or directory is hidden; false otherwise
fn is_hidden(entry: &WalkDirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}


//fn handle_print_dir(directory_name: &str) -> Vec<std::path::PathBuf> {
fn handle_print_dir(directory_name: &str) {    
    let path = Path::new(directory_name);

    let mut entries= fs::read_dir(path).unwrap() 
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    entries.sort();
    for file in &entries { //Remove this later (no need to print at server side)
        //println!("{:?}", file); // TODO send print to file-client
        // prints hidden files in bold red text
        println!("{}",
            format!("{:?}", file.to_str()).bold().red()
        );
    }
    //return entries;
}

fn handle_print_hidden() {
    // walk current directory and print all hidden (.) directories and files
    WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| is_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display())); // TODO send print to file-client
}

mod file_sys;
use file_sys::Files;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            // collect user input from file-client
            let client_cmd_str = str::from_utf8(&data).unwrap();
            let client_cmd: Vec<&str> = client_cmd_str.split("#").collect();

            if client_cmd[0] == PRINT_DIR {
                //let entries = handle_print_dir(client_cmd[1]);  
                // input will be transferred from file-client to file-server via a String input  
                handle_print_dir(client_cmd[1]);  
                
            } else if client_cmd[0] == QUIT {
                // print "exiting server.." to file-client
                exit(0);
            } else if client_cmd[0] == PRINT_HIDDEN {
                handle_print_hidden(); 
            }

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let mut db = Files::new();
    let listener = TcpListener::bind("localhost:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}