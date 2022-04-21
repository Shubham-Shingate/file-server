mod constants;
mod file_sys;
mod util;

use file_server::PgPersistance;
use file_sys::Files;
use util::LinesCodec;

use std::fs::ReadDir;
use std::fs::{self};
use std::io;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use walkdir::DirEntry as WalkDirEntry;
use walkdir::WalkDir;


fn handle_print_dir(dir_path: &str) -> ReadDir {
    //let path = Path::new(directory_name)
    let paths = fs::read_dir(dir_path).unwrap();    
    return paths;
}

fn is_hidden(entry: &WalkDirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
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

fn handle_client(stream: TcpStream) -> io::Result<()> {
    //Establish a DB Connection
    let conn = PgPersistance::get_connection();
    let all_accounts = PgPersistance::find_all(&conn);
    
    PgPersistance::save_new_acc(&conn, 1, String::from("ShubhamS"), String::from("Shubham@3344"), String::from("shubhamshingte2234@gmail.com"));

    
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
        } else if cmd_vec[0] == constants::PRINT_HIDDEN {
            let vec = handle_print_hidden();
            let mut result_str = String::from("");

            for e in vec { 
                //result_str = result_str + &format!("{}", path.unwrap().path().display()) + "  ";
                if e.file_name() != "." && e.file_name() != ".git" && e.file_name() != ".workflows" 
                    && e.file_name() != ".gitignore" {
                    result_str = result_str + &format!("{:?}", e.file_name()) + " ";
                } 
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