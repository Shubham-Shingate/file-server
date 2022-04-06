mod file_sys;
mod lib;
mod constants;

use file_sys::Files;
use lib::LinesCodec;
use std::fs::ReadDir;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;
use std::sync::Arc;
//use tempfile::tempfile;
use std::time::Duration;
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

fn handle_print_dir(dir_path: &str) -> ReadDir {
    /// TODO check that directory exists ///

    let paths = fs::read_dir(dir_path).unwrap();    
    return paths;
    /*
    let path = Path::new(directory_name);
    let mut entries = fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();
    for file in &entries {
        //Remove this later (no need to print at server side)
        println!("{:?}", file);
    }
    return entries;*/
}

fn handle_print_hidden() -> Vec<walkdir::DirEntry> {
    // walk current directory and print all hidden (.) directories and files
    let paths = WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| is_hidden(e))
        .filter_map(|v| v.ok());
        //.for_each(|x| println!("{}", x.path().display())) // TODO send print to file-client
    let mut vec: Vec<walkdir::DirEntry> = Vec::new();
    for e in paths {
        vec.push(e);        
    }
    return vec;   
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
     let mut codec = LinesCodec::new(stream)?;

    // Respond to initial handshake
    let mut msg: String = codec.read_message()?;
    codec.send_message(&msg)?;
    println!("Initial handshake was successful !!");

    loop {
        msg = codec.read_message()?;
        // TODO check that         
        let cmd_vec: Vec<&str> = msg.split(" ").collect();

        if cmd_vec[0] == constants::QUIT {
            println!("exiting the server...");
            break;
        } else if cmd_vec[0] == constants::PRINT_DIR {
            let dir_path = cmd_vec[1];
            println!("dir specified: {}", dir_path);
            let paths = handle_print_dir(&dir_path);
            let mut result_str = String::from("");
            for path in paths { 
                result_str = result_str + &format!("{}", path.unwrap().path().display()) + "  ";
            }
            codec.send_message(&result_str)?;
        } 
        else if cmd_vec[0] == constants::PRINT_HIDDEN {
            let vec = handle_print_hidden();
            let mut result_str = String::from("");

            for e in vec { 
                //result_str = result_str + &format!("{}", path.unwrap().path().display()) + "  ";
                if e.file_name() != "." && e.file_name() != ".git" && e.file_name() != ".workflows" 
                    && e.file_name() != ".gitignore" {
                    result_str = result_str + &format!("{:?}", e.file_name()) + " ";
                } 
            }
            codec.send_message(&result_str)?;
        }
    }

    Ok(())
}

// kept old main, this will need to be fixed with @Matthew's IO code
fn main() {
    let mut db = Files::new();
    let listener = TcpListener::bind("localhost:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
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

// convert single string to elems for file request
fn pregen_rqst(s: String, a: Option<File>) -> Result<(String, String, String, Option<String>, Option<File>), &'static str>{
    let mut s = s.split_whitespace();
    if let Some(u) = s.next(){ // user
        if let Some(c) = s.next(){ // command
            if let Some(p) = s.next(){ // path
                match s.next(){ // path2
                    Some(p2) => return Ok((u.to_string(), c.to_string(), p.to_string(), Some(p2.to_string()), a)),
                    None => return Ok((u.to_string(), c.to_string(), p.to_string(), None, a)),
                }
            }
        }
    }
    Err("Invalid argument count")
}

// generate file request to call from db
fn gen_rqst((user, cmd, path, path2, atch): (String, String, String, Option<String>, Option<File>)) -> Result<file_sys::FileRqst, &'static str>{
    let cmd = &cmd[..]; // convert command string to string literal for easier matching
    match cmd{
        "read" => Ok(file_sys::FileRqst::new( // read file
            user,
            path,
            file_sys::Request::Read,
        )),
        "write" => { // write to file
            if let Some(x) = atch{
                Ok(file_sys::FileRqst::new(
                    user,
                    path,
                    file_sys::Request::Write(x),
                ))
            }
            else{
                Err("No file to write from")
            }
        }
        "del" => Ok(file_sys::FileRqst::new( // delete file
            user,
            path,
            file_sys::Request::Del,
        )),
        "copy" => { // copy file to new location
            if let Some(x) = path2{ // copy to path2 from path
                Ok(file_sys::FileRqst::new(
                    user,
                    path,
                    file_sys::Request::Copy(x),
                ))
            }
            else{
                Err("No destination provided")
            }
        },
        "move" => { // move file to new location
            if let Some(x) = path2{ // move to path2 from path
                Ok(file_sys::FileRqst::new(
                    user,
                    path,
                    file_sys::Request::Move(x),
                ))
            }
            else{
                Err("No file to move")
            }
        },
        "mkdir" => Ok(file_sys::FileRqst::new( // make directory
            user,
            path,
            file_sys::Request::MakeDir,
        )),
        "rmdir" => Ok(file_sys::FileRqst::new( // remove directory
            user,
            path,
            file_sys::Request::DelDir,
        )),
        _ => Err("Invalid Command"), // default when command has no equivalent
    }
}