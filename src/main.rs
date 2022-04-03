use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;
use std::sync::Arc;

mod file_sys;
use file_sys::Files;

fn main() {
    let mut db = Arc::new(Files::new());
    let listener = TcpListener::bind("localhost:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        let db = db.clone();
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let lcl_db = Arc::clone(&db);
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream, lcl_db);
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

fn handle_client(mut stream: TcpStream, mut db: Arc<Files>) {
    let mut data = [0 as u8; 256]; // using 256 byte buffer
    while match stream.read(&mut data) {
        Ok(size) if size <= 256 => {
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
        // do things with data?
        if let Ok(s) = std::str::from_utf8(&data){ // will attempt to make a fn call, not sure how to also include a file
            if let Ok(s) = pregen_rqst(s.to_string(), None){ // not sure how to attach file
                let call = s.0.clone();
                if let Ok(ref r) = gen_rqst(s){
                    match Arc::<Files>::get_mut(&mut db).unwrap().file_rqst(r){
                        Ok(_f) => println!("{} succeeded!", call),
                        Err(e) => println!("{}", e),
                    };
                }
            }
        }
        data = [0 as u8; 256]; // clear buffer for next pass
    }
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

fn gen_rqst((user, cmd, path, path2, atch): (String, String, String, Option<String>, Option<File>)) -> Result<file_sys::FileRqst, &'static str>{
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
                Err("No file to write from")
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