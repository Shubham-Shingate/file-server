use std::collections::HashMap;
use std::fmt;
use std::error;
use std::fs::File;
use std::io::*;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub enum FileError {
    PermissionDenied,
    MissingFile,
    MissingTarget,
    BadCommand,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            FileError::PermissionDenied => write!(f, "You do not have permission to access the file"),
            FileError::MissingFile => write!(f, "File Not Found"),
            FileError::MissingTarget => write!(f, "No Destination Provided"),
            FileError::BadCommand => write!(f, "Invalid File Request"),
        }
    }
}

impl error::Error for FileError {}

#[derive(PartialEq, Clone)]
enum Permission{ // user permissions
    Owner,
    Read,
    Write,
}

#[derive(Clone)]
struct FileInfo{ // info to access file through std::io
    filepath: String,
    permissions: HashMap<String, Permission>
}

pub enum Request{ // various request types
    MakeDir,
    DelDir,
    Read,
    Write(File/*file to write from*/),
    Move(String/*new path*/),
    Copy(String/*new path*/),
    Del,

}

pub struct FileRequest{ // required info to make a file request
    user: String,
    filepath: String,
    request_type: Request,
}

#[derive(Clone)]
pub struct Files{ // collection of known files
    files: Vec<FileInfo>,
}

impl FileRequest{
    pub fn new(user: String, filepath: String, request_type: Request) -> FileRequest{ // make new file request
        FileRequest{
            user,
            filepath,
            request_type,
        }
    }
}

impl FileInfo{
    fn has_permission(&self, u: &String, p: &Permission) -> bool{ // check if user has permissions
        match self.permissions.get(u){
            Some(Permission::Owner) => true,
            Some(Permission::Read) if p == &Permission::Read => true,
            Some(Permission::Write) if p != &Permission::Owner => true,
            _ => false,
        }
    }
    fn new(filepath: String, o: String) -> FileInfo{ // make new fileinfo
        let mut permissions = HashMap::new();
        permissions.insert(o, Permission::Owner);
        FileInfo{
            filepath,
            permissions
        }
    }
}

impl Files{
    pub fn new() -> Files{ // new db
        Files{
            files: Vec::new()
        }
    }
    fn find(&self, s: &String) -> std::result::Result<&FileInfo, FileError>{ // find a fileinfo in db
        for i in &self.files{
            if s == &i.filepath{
                return Ok(i)
            }
        }
        Err(FileError::MissingFile)
    }
    pub fn file_request(&mut self, request: &FileRequest) -> std::result::Result<Option<File>, Box<dyn error::Error>>{ // do file request
        match &request.request_type {
            Request::Read => {
                if self.find(&request.filepath)?.has_permission(&request.user, &Permission::Read){ // check permission
                        Ok(Some(File::open(Path::new(&request.filepath))?))
                    }
                    else{
                        Err(Box::new(FileError::PermissionDenied))
                    }
            },
            Request::Write(a) => {
                let mut a = &*a.clone();
                if let Ok(ref x) = self.find(&request.filepath){ //look for existing file
                    if x.has_permission(&request.user, &Permission::Write){ // check permission
                        let mut file = File::create(Path::new(&request.filepath))?;
                        copy(&mut a, &mut file)?;
                        Ok(Some(file))
                    }
                    else{
                        Err(Box::new(FileError::PermissionDenied))
                    }
                }
                else {
                    let mut file = File::create(Path::new(&request.filepath))?; // create new file in location
                    copy(&mut a, &mut file)?;
                    self.files.push(FileInfo::new(request.filepath.clone(), request.user.clone()));
                    Ok(Some(file))
                }
            },
            Request::Copy(new_path) => {
                    if self.find(&request.filepath)?.has_permission(&request.user, &Permission::Read){ // check permission on old file
                        let ofile = BufReader::new(File::open(Path::new(&request.filepath))?); // open old file & create buffer to write from
                        if let Ok(x) = self.find(new_path){
                            if x.has_permission(&request.user, &Permission::Write){ // check permission on new file
                                let mut nfile = File::create(Path::new(&new_path))?; // overwrite new file
                                nfile.write_all(&ofile.buffer())?;
                                return Ok(Some(nfile))
                            }
                            else{
                                Err(Box::new(FileError::PermissionDenied))
                            }
                        }
                        else {
                            let mut nfile = File::create(Path::new(&request.filepath))?; // create new file in location
                            nfile.write_all(&ofile.buffer())?; // write to file from buffer
                            self.files.push(FileInfo::new(request.filepath.clone(), request.user.clone()));
                            Ok(Some(nfile))
                        }
                    }
                    else{
                        Err(Box::new(FileError::PermissionDenied))
                    }
            },
            Request::Move(new_path) => {
                let request = FileRequest{ // prep to copy original to new location
                    request_type: Request::Copy(new_path.to_string()),
                    user: request.user.clone(),
                    filepath: request.filepath.clone(),
                };
                self.file_request(&request)?; // copy original to new location
                let request = FileRequest{
                    request_type: Request::Del,
                    user: request.user.clone(),
                    filepath: request.filepath.clone(),
                };
                self.file_request(&request)?; // delete orignal on successful copy
                let request = FileRequest{ // prep to return file
                    request_type: Request::Read,
                    user: request.user.clone(),
                    filepath: request.filepath.clone(),
                };
                self.file_request(&request) // return file
            }
            Request::Del => {
                if self.find(&request.filepath)?.has_permission(&request.user, &Permission::Write){ // check permission
                    fs::remove_file(request.filepath.clone())?; // remove file
                    self.files.swap_remove(self.files.iter().position(|x| x.filepath == request.filepath).unwrap()); // remove fileinfo
                    Ok(None)
                }
                else{
                    Err(Box::new(FileError::PermissionDenied))
                }
                
            },
            Request::MakeDir => {
                fs::create_dir_all(request.filepath.clone())?; // add directory
                Ok(None)
            },
            Request::DelDir => {
                fs::remove_dir_all(request.filepath.clone())?; // remove directory
                Ok(None)
            },
        }
    }
}