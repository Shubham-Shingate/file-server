// broad usage
use std::fs::{File, OpenOptions};
use std::io::*;
use std::path::Path;
use std::fs;
// used for print dir & other ops that may need multi-line messages
use tempfile::tempfile;
// used for hidden dir file op
use walkdir::DirEntry as WalkDirEntry;
use walkdir::WalkDir;

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

    pub fn call(&mut self, s: &str, a: Option<File>) -> Result<ResponseType> { // handles basic input from a string and, if attached, a file
        let mut s = s.split_whitespace();
        match s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad function call"))? {
            "read" => Ok(self.read_file(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "write" => Ok(self.write_file(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?, a.ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "move" => Ok(self.move_file(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?, s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "copy" => Ok(self.copy_file(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?, s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "del" => Ok(self.delete_file(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "mkdir" => Ok(self.make_directory(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "rmdir" => Ok(self.remove_directory(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "search" => Ok(self.search(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "printdir" => Ok(self.handle_print_dir(s.next().ok_or(Error::new(ErrorKind::InvalidInput, "Bad parameter"))?)?.into()),
            "printhidden" => Ok(self.handle_print_hidden()?.into()),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Bad function call")),
        }
    }

    pub fn find_dir(&self, path: &str) -> bool { // find a directory in fileIO system
        fs::read_dir(Path::new(path)).is_ok()
    }
    pub fn find_file(&self, path: &str) -> bool { // find a file in fileIO system
        OpenOptions::new().open(Path::new(&path)).is_ok()
    }
    pub fn read_file(&self, path: &str) -> Result<File> { // read from file
        Ok(OpenOptions::new().read(true).open(Path::new(&path))?)
    }
    pub fn write_file(&self, path: &str, mut ct: File) -> Result<File> { // write to file
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(Path::new(&path))?;
        copy(&mut ct, &mut file)?;
        Ok(OpenOptions::new().read(true).open(Path::new(&path))?)
    }
    pub fn copy_file(&self, old_path: &str, new_path: &str) -> Result<File> { // copy original to new location
        let file = OpenOptions::new().read(true).write(true).create(false).open(Path::new(old_path))?;
        self.write_file(new_path, file)
    }
    pub fn move_file(&self, old_path: &str, new_path: &str) -> Result<File> {// copy original to new location, then delete original
        self.copy_file(old_path, new_path)?;
        self.delete_file(old_path)?;
        OpenOptions::new().read(true).open(Path::new(old_path))
    }
    pub fn delete_file(&self, path: &str) -> Result<String> { // remove file
        fs::remove_file(Path::new(path))?;
        Ok("File deleted successfully!".to_string())
    }
    pub fn make_directory(&self, path: &str) -> Result<String> { // add directory
        fs::create_dir_all(Path::new(path))?;
        Ok("Directory successfully created!".to_string())
    }
    pub fn remove_directory(&self, path: &str) -> Result<String> { // remove directory
        fs::remove_dir_all(Path::new(path))?;
        Ok("Directory successfully removed!".to_string())
    }
    pub fn search(&self, term: &str) -> Result<String> { // returns a list
        let p = fs::read_dir("fileIO system")?;
        let mut r = String::new();
        for i in p {
            if format!("{} ", i.as_ref().unwrap().path().display()).contains(term) { // return matching finds
                r += &format!("{} ", i?.path().display());
            } 
            else if !format!("{} ", i.as_ref().unwrap().path().display()).contains(".") { // search recursively in unhidden directories
                r += &self.subsearch(&format!("{}", i?.path().display()), term)?;
            }
        }
        Ok(r)
    }
    fn subsearch(&self, start: &str, term: &str) -> Result<String> { // recursive call of the basic search fn
        let p = fs::read_dir(start)?;
        let mut r = String::new();
        for i in p {
            if format!("{} ", i.as_ref().unwrap().path().display()).contains(term) { // return matching finds
                r += &format!("{} ", i?.path().display());
            } 
            else if !format!("{} ", i.as_ref().unwrap().path().display()).contains(".") { // search recursively in unhidden directories
                self.subsearch(&format!("{}", i?.path().display()), term)?;
            }
        }
        Ok(r)
    }
    pub fn handle_print_hidden(&self) -> Result<String> { // walk current directory and print all hidden (.) directories and files
        Ok(WalkDir::new(".").into_iter()
            .filter_entry(|e| self.is_hidden(e))
            .filter_map(|v| Some(v.ok()?.file_name().to_str()?.to_string() + " ")) // lost the ignore list b/c this doesn't like "mod constants"
            .collect())                                                            // for some reason
    }
    fn is_hidden(&self, entry: &WalkDirEntry) -> bool { // returns true if file or directory is hidden; false otherwise
        entry.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }
    pub fn handle_print_dir(&self, dir_path: &str) -> Result<File> { // print a directory
        if self.find_dir(dir_path) { // check directory validity
            let mut file = tempfile()?;
            let mut s: String = "dir specified: ".to_string() + dir_path + "\n"; // initialize result string
            s += &fs::read_dir(dir_path)?.filter_map(|x| Some(x.unwrap().path().display().to_string() + "\n")).collect::<String>(); // add dir contents to result string
            file.write_all(&s.as_bytes())?; // write result string to file for multi-line capability
            Ok(file) // return file
        }
        else{
            Err(Error::new(ErrorKind::AddrNotAvailable, "Bad directory")) // invalid directory
        }
    }
}