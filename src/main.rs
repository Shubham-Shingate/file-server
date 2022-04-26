mod file_sys;
mod lib;
mod constants;

use file_sys::{Files, ResponseType};
use lib::LinesCodec;
use std::thread;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("localhost:3333").unwrap();
    for stream in listener.incoming() { // accept connections and process them, spawning a new thread for each one
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    match handle_client(stream){
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
fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let other = &stream.peer_addr()?; // store other for later reference
    let mut codec = LinesCodec::new(stream)?;
    let msg: String = codec.read_message()?; // Respond to initial handshake
    codec.send_message(&msg)?;
    println!("Initial handshake with {} was successful !!", other);
    loop { // basic command response loop
        match &codec.read_message()?[..] { // command
            constants::QUIT => break, // end conncetion
            cmd => { // run command from file_sys
                let cmd_name = cmd.split_whitespace().next().unwrap_or("missing command");
                println!("Attempting to run command '{}' for {}...", cmd_name, other);
                codec.set_timeout(1)?; // check for file attachment
                let attachment = codec.read_file_as_str();
                if &cmd == &constants::WRITE && attachment.is_err() {
                    println!("Attachment error: {:?}", attachment);
                }
                match Files::call(&cmd, attachment.ok()) { // make fn call
                    Ok(ResponseType::File(mut f)) => {
                        println!("Successfully ran command '{}' for {}", cmd_name, other);
                        codec.send_message("Ok")?;
                        codec.send_file_as_str(&mut f)?; // send file response
                    }
                    Ok(ResponseType::String(s)) => {
                        println!("Successfully ran command '{}' for {}", cmd_name, other);
                        codec.send_message("Ok")?;
                        codec.send_message(&s)?; // send message response
                    }
                    Err(e) => {
                        println!("Error running command '{}' for {}: {}", cmd_name, other, e);
                        codec.send_message(&format!("Error running command '{}': {}", cmd_name, e))?; // send error info
                    }
                }
                codec.set_timeout(0)?; // reset timeout to await further input
            },
        }
    }
    println!("disconnecting from {}", other); // server-side disconnect notice w/ client info
    Ok(())
}