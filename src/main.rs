mod file_sys;
mod lib;
mod constants;

use file_sys::{Files, FileError};
use lib::LinesCodec;
use std::io;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::sync::Arc;
use std::fs;
use std::path::Path;
use std::str;
use std::error;

// used for hidden dir file op
use walkdir::DirEntry as WalkDirEntry;
use walkdir::WalkDir;
use colored::Colorize;

 // returns true if file or directory is hidden; false otherwise
fn is_hidden(entry: &WalkDirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

//fn handle_print_dir(directory_name: &str) -> Vec<std::path::PathBuf> {
fn handle_print_dir(directory_name: &str) {   // printing to test connection, will change
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

fn main() {
    let db = Arc::new(Files::new()); // init database
    let listener = TcpListener::bind("localhost:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let lcl_db = Arc::clone(&db); // create new database reference for thread
                thread::spawn(move|| {
                    match handle_client(stream, lcl_db){
                        Ok(()) => (),
                        Err(e) => println!("Error in Connection: {}", e),
                    }
                });
            }
            Err(e) => {
                println!("Error: {}", e); // connection failed
            }
        }
    }
    drop(listener); // close the socket server
}

// handle individual clients
fn handle_client(stream: TcpStream, mut db: Arc<Files>) -> Result<(), Box<dyn error::Error>> {
    println!("Connection to {} Successful!", &stream.peer_addr()?);
    let mut codec = LinesCodec::new(stream)?;
    let quit = String::from("quit");
    loop {
        match codec.read_message(){ // command
            Ok(cmd) if cmd == quit => break, // end conncetion
            Ok(cmd) => { // run command from file_sys
                codec.set_timeout(5);
                match Arc::<Files>::get_mut(&mut db).unwrap().file_request(&gen_request(pregen_request(cmd.to_string(), codec.read_file().ok())?)?)?{
                    Some(file) => codec.send_file(&file)?,
                    None => codec.send_message("success!")?,
                }
                codec.set_timeout(0);
            },
            Err(e) => {
                codec.kill(); // shutdown connection
                return Err(Box::new(e)) // report error
            }
        }
    }
    codec.kill(); // shutdown connection
    Ok(())
}

// convert single string to elems for file request
fn pregen_request(s: String, a: Option<File>) -> Result<(String, String, String, Option<String>, Option<File>), FileError>{
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
    Err(FileError::BadCommand)
}

// generate file request to call from db
fn gen_request((user, cmd, path, path2, attachment): (String, String, String, Option<String>, Option<File>)) -> Result<file_sys::FileRequest, FileError>{
    let cmd = &cmd[..]; // convert command string to string literal for easier matching
    match cmd{
        "read" => Ok(file_sys::FileRequest::new( // read file
            user,
            path,
            file_sys::Request::Read,
        )),
        "write" => { // write to file
            if let Some(file) = attachment{
                Ok(file_sys::FileRequest::new(
                    user,
                    path,
                    file_sys::Request::Write(file),
                ))
            }
            else{
                Err(FileError::MissingFile)
            }
        }
        "del" => Ok(file_sys::FileRequest::new( // delete file
            user,
            path,
            file_sys::Request::Del,
        )),
        "copy" => { // copy file to new location
            if let Some(new_path) = path2{ // copy to path2 from path
                Ok(file_sys::FileRequest::new(
                    user,
                    path,
                    file_sys::Request::Copy(new_path),
                ))
            }
            else{
                Err(FileError::MissingTarget)
            }
        },
        "move" => { // move file to new location
            if let Some(new_path) = path2{ // move to path2 from path
                Ok(file_sys::FileRequest::new(
                    user,
                    path,
                    file_sys::Request::Move(new_path),
                ))
            }
            else{
                Err(FileError::MissingTarget)
            }
        },
        "mkdir" => Ok(file_sys::FileRequest::new( // make directory
            user,
            path,
            file_sys::Request::MakeDir,
        )),
        "rmdir" => Ok(file_sys::FileRequest::new( // remove directory
            user,
            path,
            file_sys::Request::DelDir,
        )),
        _ => Err(FileError::BadCommand), // default when command has no equivalent
    }
}