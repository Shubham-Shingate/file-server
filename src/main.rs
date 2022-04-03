use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;
use std::sync::Arc;
use tempfile::tempfile;
use std::time::Duration;

mod file_sys;
use file_sys::Files;

// initialize & run server
fn main() {
    let mut db = Arc::new(Files::new()); // init database
    let listener = TcpListener::bind("localhost:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        let db = db.clone();
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let lcl_db = Arc::clone(&db); // create new database reference for thread
                thread::spawn(move|| {
                    handle_client(stream, lcl_db); // connection succeeded
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
fn handle_client(mut stream: TcpStream, mut db: Arc<Files>) {
    const BUF_SIZE: usize = 1024;
    let mut cmd_buf = [0 as u8; BUF_SIZE]; // buffer for command
    let mut fle_buf = [0 as u8; BUF_SIZE]; // buffer for attached file
    while match stream.read(&mut cmd_buf) {
        Ok(size) if size <= BUF_SIZE => {
            // echo everything!
            stream.write(&cmd_buf[0..size]).unwrap();
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
        stream.set_read_timeout(Some(Duration::from_secs(5)));
        if let Ok(..) = stream.read(&mut fle_buf){ // check for attached file data
            if let Ok(mut file) = tempfile(){ // make temp file to hold data
                if let Ok(..) = file.write_all(&fle_buf){ // write file data to temp file
                    if let Ok(s) = std::str::from_utf8(&cmd_buf){ // will attempt to make a fn call, not sure how to also include a file
                        if let Ok(s) = pregen_rqst(s.to_string(), Some(file)){ // not sure how to attach file
                            let call = s.0.clone(); // logs fn called
                            if let Ok(ref r) = gen_rqst(s){
                                match Arc::<Files>::get_mut(&mut db).unwrap().file_rqst(r){
                                    Ok(mut f) => {
                                        println!("{} succeeded!", call);
                                        fle_buf = [0 as u8; BUF_SIZE]; // clear file buffer for return
                                        if let Ok(size) = f.read(&mut fle_buf){
                                            stream.write(&fle_buf[0..size]).unwrap(); // return file data to stream
                                        }
                                    },
                                    Err(e) => println!("{}", e),
                                };
                            }
                        }
                    }
                }
            }
            fle_buf = [0 as u8; BUF_SIZE]; // clear fiile buffer for next pass
        }
        else{
            if let Ok(s) = std::str::from_utf8(&cmd_buf){ // will attempt to make a fn call, not sure how to also include a file
                if let Ok(s) = pregen_rqst(s.to_string(), None){ // not sure how to attach file
                    let call = s.0.clone(); // logs fn called
                    if let Ok(ref r) = gen_rqst(s){
                        match Arc::<Files>::get_mut(&mut db).unwrap().file_rqst(r){
                            Ok(mut f) => {
                                println!("{} succeeded!", call);
                                fle_buf = [0 as u8; BUF_SIZE]; // clear file buffer for return
                                if let Ok(size) = f.read(&mut fle_buf){
                                    stream.write(&fle_buf[0..size]).unwrap(); // return file data to stream
                                }
                            },
                            Err(e) => println!("{}", e),
                        };
                    }
                }
            }
        }
        stream.set_read_timeout(None);
        cmd_buf = [0 as u8; BUF_SIZE]; // clear command buffer for next pass
    }
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