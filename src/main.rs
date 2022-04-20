mod file_sys;
mod lib;
mod constants;

use file_sys::{Files, ResponseType};
use lib::LinesCodec;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::error;
use std::boxed::Box;

fn main() {
    let files = Arc::new(Files::new()); // init fileIO
    let listener = TcpListener::bind("localhost:3333").unwrap();
    for stream in listener.incoming() { // accept connections and process them, spawning a new thread for each one
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let lcl_db = Arc::clone(&files); // create new fileIO reference for thread
                thread::spawn(move|| {
                    match handle_client(stream, lcl_db){
                        Ok(()) => (), // manual disconnect, no problems
                        Err(e) => println!("Error in Connection: {}", e), // error notice
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
fn handle_client(stream: TcpStream, mut files: Arc<Files>) -> Result<(), Box<dyn error::Error>> {
    let other = &stream.peer_addr()?; // store other for later reference
    let mut codec = LinesCodec::new(stream)?;
    let msg: String = codec.read_message()?; // Respond to initial handshake
    codec.send_message(&msg)?;
    println!("Initial handshake with {} was successful !!", other);
    loop { // basic command response loop
        match codec.read_message() { // command
            Ok(cmd) if &cmd == constants::QUIT => break, // end conncetion
            Ok(cmd) => { // run command from file_sys
                codec.set_timeout(5); // check for file attachment
                match Arc::<Files>::make_mut(&mut files).call(&cmd, codec.read_file().ok()) { // make fn call
                    Ok(ResponseType::File(mut f)) => codec.send_file(&mut f)?, // send file response
                    Ok(ResponseType::String(s)) => codec.send_message(&s)?, // send message response
                    Err(e) => codec.send_message(&format!("{}", e))?, // send error info
                }
                codec.set_timeout(0); // reset timeout to await further input
            },
            Err(e) => {
                return Err(Box::new(e)) // report error
            }
        }
    }
    println!("disconnecting from {}", other); // server-side disconnect notice w/ client info
    Ok(())
}