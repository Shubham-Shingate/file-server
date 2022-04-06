mod constants;
mod file_sys;
mod lib;

use file_sys::Files;
use lib::LinesCodec;
use std::fs::ReadDir;
use std::fs::{self, DirEntry};
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::Path;
use std::str;
use std::thread;

fn handle_print_dir(dir_path: &str) -> ReadDir {
    //let path = Path::new(directory_name)
    let paths = fs::read_dir(dir_path).unwrap();    
    return paths;
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut codec = LinesCodec::new(stream)?;

    // Respond to initial handshake
    let mut msg: String = codec.read_message()?;
    codec.send_message(&msg)?;
    println!("Initial handshake was successful !!");

    loop {
        msg = codec.read_message()?;
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
        } else {
            
        }
    }

    Ok(())
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
