use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;

mod file_sys;
use file_sys::Files;

fn handle_client(mut stream: TcpStream, db: Files) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) if size <= 50 => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Ok(..) => {
            println!("Data exceeded buffer size!");
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        // do things with data
        
    }
}

fn main() {
    let mut db = Files::new();
    let listener = TcpListener::bind("localhost:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        let db = db.clone();
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream, db)
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

fn pregen_rqst(s: String, a: Option<File>) -> Result<(String, String, String, Option<String>, Option<File>), &'static str>{
    let mut s = s.split_whitespace();
    if let Some(u) = s.next(){
        if let Some(c) = s.next(){
            if let Some(p) = s.next(){
                if let Some(p2) = s.next(){
                    return Ok((u.to_string(), c.to_string(), p.to_string(), Some(p2.to_string()), a))
                }
                else{
                    return Ok((u.to_string(), c.to_string(), p.to_string(), None, a))
                }
            }
        }
    }
    Err("Invalid argument count")
}

fn gen_rqst(user: String, cmd: String, path: String, path2: Option<String>, atch: Option<File>) -> Result<file_sys::FileRqst, &'static str>{
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
            if let Some(x) = path2{
                Ok(file_sys::FileRqst::new(
                    user,
                    path,
                    file_sys::Request::Copy(x),
                ))
            }
            else{
                Err("No file to copy")
            }
        },
        "move" => {
            if let Some(x) = path2{
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
        "mkdir" => Ok(file_sys::FileRqst::new(
            user,
            path,
            file_sys::Request::MakeDir,
        )),
        "rmdir" => Ok(file_sys::FileRqst::new(
            user,
            path,
            file_sys::Request::DelDir,
        )),
        _ => Err("Invalid Command"),
    }
}