use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;

mod file_sys;
use file_sys::Files;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
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

fn pregen_rqst(s: String, a: Option<File>) -> (){

}

fn gen_rqst(user: String, cmd: String, path: String, atch: Option<File>) -> Result<file_sys::FileRqst, &'static str>{
    let cmd = &cmd[..];
    match cmd{
        "read" => Ok(file_sys::FileRqst::new(
            user,
            path,
            file_sys::Request::Read,
        )),
        "write" => {
            if let Some(x) = atch{
                Ok(file_sys::FileRqst::new(
                    user,
                    path,
                    file_sys::Request::Write(x),
                ))
            }
            else{
                Err("No file to write to")
            }
        }   
        "del" => Ok(file_sys::FileRqst::new(
            user,
            path,
            file_sys::Request::Del,
        )),
        "copy" => {
            if let Some(x) = atch{
                Ok(file_sys::FileRqst::new(
                    user,
                    path,
                    file_sys::Request::Write(x),
                ))
            }
            else{
                Err("No file to copy")
            }
        }   
        _ => Err("Invalid Command")
    }
}