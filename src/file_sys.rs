use std::fs::{File, OpenOptions};
use std::io::*;
use std::path::Path;
use std::fs;

#[derive(Clone)]
pub struct Files; // db reference

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
    pub fn new() -> Files { Files{} } // new db
    pub fn call(&mut self, s: &str, a: Option<File>) -> Result<ResponseType> {
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
            _ => Err(Error::new(ErrorKind::InvalidInput, "Bad function call")),
        }
    }
    pub fn find_dir(&self, path: &str) -> bool { // find a directory in db
        fs::read_dir(Path::new(path)).is_ok()
    }
    pub fn find_file(&self, path: &str) -> bool { // find a file in db
        OpenOptions::new().open(Path::new(&path)).is_ok()
    }
    pub fn read_file(&self, path: &str) -> Result<File> { // read from file
        Ok(OpenOptions::new().read(true).open(Path::new(&path))?)
    }
    pub fn write_file(&self, path: &str, mut ct: File) -> Result<File> { // write to file
        if self.find_file(path) { //look for existing file
                let mut file = OpenOptions::new().read(true).write(true).create(true).open(Path::new(&path))?;
                copy(&mut ct, &mut file)?;
                Ok(OpenOptions::new().read(true).open(Path::new(&path))?)
        }
        else { // write new file
            let mut file = OpenOptions::new().read(true).write(true).create(true).open(Path::new(&path))?;
            copy(&mut ct, &mut file)?;
            OpenOptions::new().read(true).open(Path::new(&path))
        }
    }
    pub fn copy_file(&self, old_path: &str, new_path: &str) -> Result<File> { // copy original to new location
        let mut file = OpenOptions::new().read(true).write(true).create(false).open(Path::new(old_path))?;
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
    pub fn make_directory(&self, path: &str) -> Result<String> {
        fs::create_dir_all(Path::new(path))?; // add directory
        Ok("Directory successfully created!".to_string())
    }
    pub fn remove_directory(&self, path: &str) -> Result<String> {
        fs::remove_dir_all(Path::new(path))?; // remove directory
        Ok("Directory successfully removed!".to_string())
    }
    pub fn search(&self, term: &str) -> Result<String> {
        let p = fs::read_dir("db")?;
        let mut r = String::new();
        for i in p {
            if format!("{} ", i.as_ref().unwrap().path().display()).contains(term) {
                r += &format!("{} ", i?.path().display());
            } 
            else if !format!("{} ", i.as_ref().unwrap().path().display()).contains(".") {
                r += &self.subsearch(&format!("{}", i?.path().display()), term)?;
            }
        }
        Ok(r)
    }
    fn subsearch(&self, start: &str, term: &str) -> Result<String> {
        let p = fs::read_dir(start)?;
        let mut r = String::new();
        for i in p {
            if format!("{} ", i.as_ref().unwrap().path().display()).contains(term) {
                r += &format!("{} ", i?.path().display());
            } 
            else if !format!("{} ", i.as_ref().unwrap().path().display()).contains(".") {
                self.subsearch(&format!("{}", i?.path().display()), term)?;
            }
        }
        Ok(r)
    }
}