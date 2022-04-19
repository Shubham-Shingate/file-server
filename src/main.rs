mod file_sys;
mod lib;
mod constants;

use file_sys::{Files, ResponseType};
use lib::LinesCodec;
use std::io;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::sync::Arc;
use std::fs::{self, ReadDir};
//use std::path::Path;
use std::str;
use std::error;
use std::boxed::Box;

// used for hidden dir file op
use walkdir::DirEntry as WalkDirEntry;
use walkdir::WalkDir;
//use colored::Colorize;

 // returns true if file or directory is hidden; false otherwise
fn is_hidden(entry: &WalkDirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}


fn handle_print_dir(dir_path: &str) -> ReadDir {
    // TODO check that directory exists //

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

fn main() {
    let mut db = Arc::new(Files::new()); // init database
    if let Some(_) = Arc::<Files>::get_mut(&mut db){
        println!("Database initialization successful");
    }
    else{
        println!("Database initialization failed");
    }
    let listener = TcpListener::bind("localhost:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        /*let mut cmd = String::new(); // add way to shutdown server
        io::stdin().read_line(&mut cmd).unwrap();
        cmd = cmd.trim().to_owned();
        if &cmd == constants::QUIT {
            break;
        }*/
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
                println!("Failed connection: {}", e); // connection failed
            }
        }
    }
    drop(listener); // close the socket server
}

// handle individual clients
fn handle_client(stream: TcpStream, mut db: Arc<Files>) -> Result<(), Box<dyn error::Error>> {
    let other = &stream.peer_addr()?; // store other for later reference
    let mut codec = LinesCodec::new(stream)?;
    let msg: String = codec.read_message()?; // Respond to initial handshake
    codec.send_message(&msg)?;
    println!("Initial handshake with {} was successful !!", other);
    if let Some(_) = Arc::<Files>::get_mut(&mut db){ // Check database connection
        println!("Database connection for {} successful", other);
        codec.send_message("Connected to database successfully")?;
    }
    else{
        println!("Database connection for {} failed", other);
        codec.send_message("Could not connect to database")?;
    }
    loop {
        match codec.read_message() { // command
            Ok(cmd) if &cmd == constants::QUIT => break, // end conncetion
            Ok(cmd) => { // run command from file_sys
                codec.set_timeout(5);
                match Arc::<Files>::make_mut(&mut db).call(&cmd, codec.read_file().ok()) {
                    Ok(ResponseType::File(mut f)) => codec.send_file(&mut f)?,
                    Ok(ResponseType::String(s)) => codec.send_message(&s)?,
                    Err(_) => {
                        let cmd_vec: Vec<&str> = cmd.split(" ").collect();
                        match cmd_vec[0] {
                            constants::PRINT_DIR => {
                                let dir_path = cmd_vec[1];
                                println!("dir specified: {}", dir_path);
                                let paths = handle_print_dir(&dir_path);
                                let mut result_str = String::new();
                                for path in paths { 
                                    result_str = result_str + &format!("{}", path.unwrap().path().display()) + "  ";
                                    codec.send_message(&result_str)?;
                                }
                            },
                            constants::PRINT_HIDDEN => {
                                let vec = handle_print_hidden();
                                let mut result_str = String::new();
                    
                                for e in vec {
                                    //result_str = result_str + &format!("{}", path.unwrap().path().display()) + "  ";
                                    if e.file_name() != "." && e.file_name() != ".git" && e.file_name() != ".workflows" 
                                        && e.file_name() != ".gitignore" {
                                        result_str = result_str + &format!("{:?}", e.file_name()) + " ";
                                    } 
                                }
                                codec.send_message(&result_str)?;
                            },
                            /*constants::SEARCH => {
                                codec.send_message(&db.search(&cmd_vec[1]));
                            }*/
                            _ => codec.send_message("Invalid Command")?,
                        }
                    },
                }
                codec.set_timeout(0);
            },
            Err(e) => {
                return Err(Box::new(e)) // report error
            }
        }
    }
    println!("disconnecting from {}", other);
    Ok(())
}