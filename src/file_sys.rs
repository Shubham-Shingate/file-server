#![allow(dead_code)]
// broad usage
use std::fs::{File, OpenOptions};
use std::io::*;
use std::path::Path;
use std::fs;
// used for hidden dir file op
use walkdir::DirEntry as WalkDirEntry;
use walkdir::WalkDir;
// used for folder root & command names
#[path = "constants.rs"] mod constants;
use constants as consts;

#[derive(Clone)]
pub struct Files; // fileIO system reference

pub enum ResponseType { // handles different return types
    File(File),
    String(String),
}

impl From<File> for ResponseType { // embed given file into response
    fn from(f: File) -> Self {
        ResponseType::File(f)
    }
}

impl From<String> for ResponseType { // embed given string into response
    fn from(s: String) -> Self {
        ResponseType::String(s)
    }
}

impl Files{
    pub fn new() -> Files { Files{} } // new fileIO system

    // handles basic input from a string and, if attached, a file
    pub fn call(s: &str, a: Option<String>) -> Result<ResponseType> {
        let root = consts::ROOT.to_string(); // root folder to add before paths
        let mut s = s.split_whitespace();
        match s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing function call"))? {
            consts::READ => Ok(Files::read_file(&(root + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing path parameter"))?))?.into()),
            consts::WRITE => Ok(Files::write_file_from_str(&(root + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing path parameter"))?), 
                &a.ok_or(Error::new(ErrorKind::InvalidInput, "Missing file parameter"))?)?.into()),
            consts::MOVE => Ok(Files::move_file(&(root.clone() + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing origin parameter"))?), 
                &(root + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing destination parameter"))?))?.into()),
            consts::COPY => Ok(Files::copy_file(&(root.clone() + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing origin parameter"))?), 
                &(root + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing destination parameter"))?))?.into()),
            consts::DELETE => Ok(Files::delete_file(&(root + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing path parameter"))?))?.into()),
            consts::MAKE_DIR => Ok(Files::make_directory(&(root + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing path parameter"))?))?.into()),
            consts::REMOVE_DIR => Ok(Files::remove_directory(&(root + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing path parameter"))?))?.into()),
            consts::SEARCH => Ok(Files::search(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing search term parameter"))?)?.into()),
            consts::PRINT_DIR => Ok(Files::handle_print_dir(&(root + s.next().unwrap_or("")))?.into()),
            consts::PRINT_HIDDEN => Ok(Files::handle_print_hidden()?.into()),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid function call")),
        }
    }

    // calls a specific function with parameters as a string
    // For input: 
        // func should be the function constant for the desired function call
        // params should be the rest of the call string
    pub fn call_fn(func: &str, params: &str) -> Result<ResponseType> {
        let root = consts::ROOT.to_string(); // root folder to add before paths
        let mut s = params.split_whitespace();
        match func {
            consts::WRITE => Ok(Files::write_file_from_str(&(root + s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Missing path parameter"))?),
                //params.split_once(' ').unwrap_or(("","")).1)?.into()),  // missing file writes empty file
                params.split_once(' ').ok_or(Error::new(ErrorKind::InvalidInput, "Missing file parameter"))?.1)?.into()), // missing file cancels write
            _ => Files::call(&(func.to_string() + " " + &params), None),
        }
    }

    pub fn find_dir(path: &str) -> bool { // find a directory in fileIO system
        fs::read_dir(Path::new(path)).is_ok()
    }
    pub fn find_file(path: &str) -> bool { // find a file in fileIO system
        OpenOptions::new().open(Path::new(&path)).is_ok()
    }
    pub fn read_file(path: &str) -> Result<File> { // read from file
        Ok(OpenOptions::new().read(true).open(Path::new(&path))?)
    }
    pub fn write_file(path: &str, mut ct: File) -> Result<File> { // write to file
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(Path::new(&path))?;
        copy(&mut ct, &mut file)?;
        Ok(OpenOptions::new().read(true).open(Path::new(&path))?)
    }
    pub fn write_file_from_str(path: &str, ct: &str) -> Result<File> { // write to file
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(Path::new(&path))?;
        file.write_all(ct.as_bytes())?;
        Ok(OpenOptions::new().read(true).open(Path::new(&path))?)
    }
    pub fn copy_file(old_path: &str, new_path: &str) -> Result<File> { // copy original to new location
        let file = OpenOptions::new().read(true).write(true).create(false).open(Path::new(old_path))?;
        Files::write_file(new_path, file)
    }
    pub fn move_file(old_path: &str, new_path: &str) -> Result<File> { // copy original to new location, then delete original
        Files::copy_file(old_path, new_path)?;
        Files::delete_file(old_path)?;
        OpenOptions::new().read(true).open(Path::new(new_path))
    }
    pub fn delete_file(path: &str) -> Result<String> { // remove file
        fs::remove_file(Path::new(path))?;
        Ok("File deleted successfully!".to_string())
    }
    pub fn make_directory(path: &str) -> Result<String> { // add directory
        fs::create_dir_all(Path::new(path))?;
        Ok("Directory successfully created!".to_string())
    }
    pub fn remove_directory(path: &str) -> Result<String> { // remove directory
        fs::remove_dir_all(Path::new(path))?;
        Ok("Directory successfully removed!".to_string())
    }
    pub fn search(term: &str) -> Result<String> { // returns a list
        let root = consts::ROOT; // root folder
        let p = fs::read_dir(root)?;
        let mut r = String::new();
        for i in p {
            if i.as_ref().unwrap().path().display().to_string().trim_start_matches(root).contains(term) { // return matching finds
                r += &format!("{} ", i?.path().display()).trim_start_matches(root);
            } 
            else if fs::metadata(i.as_ref().unwrap().path().display().to_string())?.is_dir() { // search recursively in unhidden directories
                r += &Files::subsearch(&i?.path().display().to_string(), term)?;
            }
        }
        Ok(r)
    }
    fn subsearch(start: &str, term: &str) -> Result<String> { // recursive call of the basic search fn
        let root = consts::ROOT; // root folder
        let p = fs::read_dir(start)?;
        let mut r = String::new();
        for i in p {
            if i.as_ref().unwrap().path().display().to_string().trim_start_matches(root).contains(term) { // return matching finds
                r += &format!("{} ", i?.path().display()).trim_start_matches(root);
            } 
            else if fs::metadata(i.as_ref().unwrap().path().display().to_string())?.is_dir() { // search recursively in unhidden directories
                Files::subsearch(&i?.path().display().to_string(), term)?;
            }
        }
        Ok(r)
    }
    pub fn handle_print_hidden() -> Result<String> { // walk current directory and print all hidden (.) directories and files
        let ignore = vec![".", ".git", ".workflows", ".gitignore"];
        Ok(WalkDir::new(".").into_iter()
            .filter_entry(|e| Files::is_hidden(e)) // filter hidden only
            .filter_map(|v| Some(v.ok()?.file_name().to_str()?.to_string() + " ")) // string conversion
            .filter(|x| !ignore.contains(&x.trim())) // filter ignored
            .collect())
    }
    fn is_hidden(entry: &WalkDirEntry) -> bool { // returns true if file or directory is hidden; false otherwise
        entry.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }
    pub fn handle_print_dir(dir_path: &str) -> Result<String> { // print a directory
        let root = consts::ROOT; // root folder
        if Files::find_dir(dir_path) { // check directory validity
            Ok(fs::read_dir(dir_path)? // add dir contents to result string & return it
                .filter_map(|x| Some(x.unwrap().path().display().to_string().trim_start_matches(root).to_string() + " "))
                .collect::<String>()
            )
        }
        else{
            Err(Error::new(ErrorKind::AddrNotAvailable, "Inalid directory")) // invalid directory message
        }
    }
}