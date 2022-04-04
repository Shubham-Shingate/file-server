mod file_sys;
mod lib;
mod constants;

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

fn handle_print_dir(directory_name: &str) -> Vec<std::path::PathBuf> {
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
    return entries;
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut codec = LinesCodec::new(stream)?;

    // Respond to initial handshake
    let msg: String = codec.read_message()?;
    codec.send_message(&msg)?;
    println!("Initial handshake was successful !!");

    loop {
        let cleint_cmd_str = str::from_utf8(&data).unwrap();
        let client_cmd: Vec<&str> = cleint_cmd_str.split("#").collect();

        if client_cmd[0] == constants::PRINT_DIR {
            let entries = handle_print_dir(client_cmd[1]);
            //CONTINUE HERE SHUBHAM
        } else if client_cmd[0] == constants::QUIT {
            break;
        }
    }
    stream.shutdown(Shutdown::Both).unwrap();
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
