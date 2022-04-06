use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::*;
use std::path::Path;
use std::fs;

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
    Del,
    Copy(String/*new path*/),
    Move(String/*new path*/),
}

pub struct FileRqst{ // required info to make a file request
    user: String,
    filepath: String,
    rqst_tp: Request,
}

#[derive(Clone)]
pub struct Files{ // collection of known files
    files: Vec<FileInfo>,
}

impl FileRqst{
    pub fn new(user: String, filepath: String, rqst_tp: Request) -> FileRqst{ // make new file request
        FileRqst{
            user,
            filepath,
            rqst_tp,
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
    fn find(&self, s: &String) -> Option<&FileInfo>{ // find a fileinfo in db
        for i in &self.files{
            if s == &i.filepath{
                return Some(i)
            }
        }
        None
    }
    pub fn file_rqst(&mut self, rqst: &FileRqst) -> std::result::Result<File, &str>{ // do file request
        match &rqst.rqst_tp {
            Request::Read => {
                if let Some(ref x) = self.find(&rqst.filepath){ // look for file
                    if x.has_permission(&rqst.user, &Permission::Read){ // check permission
                        match File::open(Path::new(&rqst.filepath)){
                            Ok(x) => Ok(x),
                            Err(..) => Err("File not found"),
                        }
                    }
                    else{
                        Err("You do not have permission to access this file")
                    }
                }
                else{
                    Err("File not in system")
                }
            },
            Request::Write(a) => { 
                let a = BufReader::new(a); // make buffer to write from
                if let Some(ref x) = self.find(&rqst.filepath){ //look for existing file
                    if x.has_permission(&rqst.user, &Permission::Write){ // check permission
                        let mut file = match File::create(Path::new(&rqst.filepath)){ // overwrite file
                            Ok(x) => Ok(x),
                            Err(..) => Err("Could not create"),
                        };
                        if let Ok(ref mut x) = file{
                            return match x.write_all(&a.buffer()) { // write to file from buffer
                                Ok(..) => file,
                                Err(..) => Err("Writing Failed")
                            }
                        }
                        file
                    }
                    else{
                        Err("You do not have permission to access this file")
                    }
                }
                else {
                    let mut file = match File::create(Path::new(&rqst.filepath)){ // create new file in location
                        Ok(x) => Ok(x),
                        Err(..) => Err("Could not create"),
                    };
                    if let Ok(ref mut x) = file{
                        return match x.write_all(&a.buffer()) { // write to file from buffer
                            Ok(..) => {
                                self.files.push(FileInfo::new(rqst.filepath.clone(), rqst.user.clone()));
                                file
                            },
                            Err(..) => Err("Writing Failed")
                        }
                    }
                    file
                }
            },
            Request::Copy(new_path) => {
                if let Some(x) = self.find(&rqst.filepath){ // look for existing old file
                    if x.has_permission(&rqst.user, &Permission::Read){ // check permission on old file
                        let ofile = match File::open(Path::new(&rqst.filepath)){ // open old file
                            Ok(x) => Ok(x),
                            Err(..) => Err("Could not open file to copy"),
                        };
                        if let Ok(x) = ofile{
                            let x = BufReader::new(x); // create buffer to write from old file
                            if let Some(ref y) = self.find(new_path){ // look for existing new file
                                if y.has_permission(&rqst.user, &Permission::Write){ // check permission on new file
                                    let mut nfile = match File::create(Path::new(&new_path)){ // overwrite new file
                                        Ok(y) => Ok(y),
                                        Err(..) => Err("Could not create file"),
                                    };
                                    if let Ok(ref mut y) = nfile{
                                        return match y.write_all(&x.buffer()) { // write to new file from old file buffer
                                            Ok(..) => nfile,
                                            Err(..) => Err("Writing Failed")
                                        };
                                    }
                                    nfile
                                }
                                else{
                                    Err("Protected file already in target location")
                                }
                            }
                            else {
                                let mut nfile = match File::create(Path::new(&new_path)){ // create file at new location
                                    Ok(y) => Ok(y),
                                    Err(..) => Err("Could not create file"),
                                };
                                if let Ok(ref mut y) = nfile{
                                    return match y.write_all(&x.buffer()) { // write to new file from old file buffer
                                        Ok(..) => {
                                            self.files.push(FileInfo::new(new_path.clone(), rqst.user.clone()));
                                            nfile
                                        },
                                        Err(..) => Err("Writing Failed")
                                    };
                                }
                                nfile
                            }
                        }
                        else{
                            ofile
                        }
                    }
                    else{
                        Err("You do not have permission to access this file")
                    }
                }
                else {
                    Err("No file to copy")
                }
            },
            Request::Move(new_path) => {
                let rqst = FileRqst{ // prep to copy original to new location
                    rqst_tp: Request::Copy(new_path.to_string()),
                    user: rqst.user.clone(),
                    filepath: rqst.filepath.clone(),
                };
                if let Ok(..) = self.file_rqst(&rqst){ // copy original to new location
                        let rqst = FileRqst{
                            rqst_tp: Request::Del,
                            user: rqst.user.clone(),
                            filepath: rqst.filepath.clone(),
                        };
                        let res = self.file_rqst(&rqst); // delete orignal on successful copy
                        res
                    }
                else{
                    Err("Move failed")
                }
            }
            Request::Del => {
                if let Some(ref x) = self.find(&rqst.filepath){ // look for file
                    if x.has_permission(&rqst.user, &Permission::Write){ // check permission
                        match fs::remove_file(rqst.filepath.clone()).map_err(|e| -> String { format!("{:?}", e.kind()) }){ // remove file
                            Ok(..) => {
                                //self.files.swap_remove(); // need to get index somehow
                                Err("File deleted")
                            },
                            Err(..) => Err("File could not be deleted"),
                        }
                    }
                    else{
                        Err("You do not have permission to access this file")
                    }
                }
                else{
                    Err("No file to delete")
                }
                
            },
            Request::MakeDir => {
                match fs::create_dir_all(rqst.filepath.clone()).map_err(|e| -> String { format!("{:?}", e.kind()) }){ // add directory
                    Ok(..) => Err("Directory added"),
                    Err(..) => Err("Directory could not be added"),
                }
            },
            Request::DelDir => {
                match fs::remove_dir_all(rqst.filepath.clone()).map_err(|e| -> String { format!("{:?}", e.kind()) }){ // remove directory
                    Ok(..) => Err("directory and contents removed"),
                    Err(..) => Err("directory could not be removed"),
                }
            },
        }
    }
}