mod constants;
mod util;
mod file_ops;

use file_server::PgPersistance;
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
    file_ops::make_dir(constants::SERVER_FILE_STORAGE)?;
    let current_dir = constants::SERVER_FILE_STORAGE;

    //Establish a DB Connection
    let conn = PgPersistance::get_connection();
    
    //<<Sample usage of (ORM) to query database without SQL statements-------------------- (For developer reference)
    // let all_accounts = PgPersistance::find_all_acc(&conn);
    // let acct = PgPersistance::find_by_username(&conn, "KGF");
    // PgPersistance::save_new_acc(&conn, String::from("KGF"), String::from("KGF@3344"), String::from("kgf@gmail.com"));
    
    // let all_files = PgPersistance::find_all_files(&conn);
    // PgPersistance::save_new_file(&conn, String::from("D:/Home/Desktop"));
    //Sample usage of (ORM) to query database without SQL statements>>-------------------- (For developer reference)

    let mut codec = LinesCodec::new(stream)?;

    // Respond to initial handshake
    let mut msg: String = codec.read_message()?;
    codec.send_message(&msg)?;
    println!("Initial handshake was successful !!");

    let mut logged_in = false;
    let mut session_user_name = String::from("");

    while !logged_in {
       let choice = codec.read_message()?;

       match choice.as_str() {
           "1" => {
               //Read acc creation info
               let username = codec.read_message()?;
               let password = codec.read_message()?;
               let email = codec.read_message()?;
               //Create a new account in accounts table
               let acc_saved = PgPersistance::save_new_acc(&conn, username, password, email);
               let mut files = PgPersistance::find_all_files(&conn);
               files.iter_mut().for_each(|x| {
                   PgPersistance::save_new_acc_file_mapping(&conn, acc_saved.user_id, x.file_id, String::from("RW"));
               });
           },
           "2" => {
                //Read the login credentials
                let username = codec.read_message()?;
                let password = codec.read_message()?;
                //Check password with DB records.
                let acct = PgPersistance::find_by_username(&conn, username.as_str());
                if acct.is_none() {
                    println!("Invalid username, no account found for this username");
                    codec.send_message("Failure")?;
                    codec.send_message("Invalid username, no account found for this username")?;
                    continue;
                }
                let account = acct.unwrap();
                if password == account.password {
                    logged_in = true;
                    session_user_name = account.username;
                    println!("Login successfull, session began for user: {}", &session_user_name);
                    codec.send_message("Success")?;
                } else {
                    println!("Invalid password");
                    codec.send_message("Failure")?;
                    codec.send_message("Invalid Password")?;
                }
           },
           _ => {
                println!("Invalid Choice");
            }
       }
    }

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
        } else if cmd_vec[0] == constants::MAKE_DIR {
            file_ops::make_dir(&(String::from(current_dir)+"/"+cmd_vec[1]))?;
            codec.send_message("Success")?;
        } else if cmd_vec[0] == constants::PUT_FILE {
            let file_path = String::from(current_dir)+"/"+cmd_vec[1];
            let file_data = codec.read_file_socket()?;
            
            let file_entity = PgPersistance::find_by_filepath(&conn, &file_path);
            
            if file_entity.is_none() { //File Path does not exist in DB, create one and map it to all users
                file_ops::write_file(&file_path, &file_data)?;
                let fileentity = PgPersistance::save_new_file(&conn, file_path);
                let mut accounts = PgPersistance::find_all_acc(&conn);
                accounts.iter_mut().for_each(|acc| {
                    PgPersistance::save_new_acc_file_mapping(&conn, acc.user_id, fileentity.file_id, "RW".to_owned());
                });
                codec.send_message("Success")?;
            } else { //File exists, check for permissions
                if PgPersistance::is_authorized(&conn, &session_user_name, &file_path, "RW") {
                    file_ops::write_file(&file_path, &file_data)?;
                    codec.send_message("Success")?;
                } else {
                    codec.send_message("Failure: Unauthorized to write this file")?;
                }
            }
        } else if cmd_vec[0] == constants::GET_FILE {
            let file_path = String::from(current_dir)+"/"+cmd_vec[1];
            let file_data = file_ops::read_file(&file_path)?;
            codec.send_message(&file_data)?;
            codec.send_message("e*-of")?;
            
        }
    }

    Ok(())
}

fn main() {
    //let mut db = Files::new();
    
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