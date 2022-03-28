use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::*;
use std::path::Path;
use std::fs;

struct FileInfo{
    filepath: String,
    permissions: HashMap<String, Permission>
}

#[derive(PartialEq)]
enum Permission{
    Owner,
    Read,
    Write,
}

struct FileRqst{
    user: String,
    filepath: String,
    rqst_tp: Request,
}

enum Request{
    Read,
    Write(File),
    Del,
    Copy(String/*new path*/),
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
    fn new(filename: String, filepath: String, o: String) -> FileInfo{
        let mut permissions = HashMap::new();
        permissions.insert(o, Permission::Owner);
        FileInfo{
            filepath,
            permissions
        }
    }
}

pub struct Files{
    files: Vec<FileInfo>,
}

impl Files{
    pub fn new() -> Files{
        Files{
            files: Vec::new()
        }
    }
    fn find(&self, s: &String) -> Option<&FileInfo>{
        for i in &self.files{
            if s == &i.filepath{
                return Some(i)
            }
        }
        None
    }
    pub fn file_rqst(&mut self, rqst: FileRqst) -> std::result::Result<File, &str>{
        match &rqst.rqst_tp {
            Request::Read => {
                if let Some(ref x) = self.find(&rqst.filepath){
                    if x.has_permission(&rqst.user, &Permission::Read){
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
                let a = BufReader::new(a);
                if let Some(ref x) = self.find(&rqst.filepath){
                    if x.has_permission(&rqst.user, &Permission::Write){
                        let mut file = match File::create(Path::new(&rqst.filepath)){
                            Ok(x) => Ok(x),
                            Err(..) => Err("Could not create"),
                        };
                        if let Ok(ref mut x) = file{
                            return match x.write_all(&a.buffer()) {
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
                    let mut file = match File::create(Path::new(&rqst.filepath)){
                        Ok(x) => Ok(x),
                        Err(..) => Err("Could not create"),
                    };
                    if let Ok(ref mut x) = file{
                        return match x.write_all(&a.buffer()) {
                            Ok(..) => {
                                self.files.push(FileInfo::new(rqst.filepath.clone(), rqst.filepath.clone(), rqst.user.clone()));
                                file
                            },
                            Err(..) => Err("Writing Failed")
                        }
                    }
                    file
                }
            },
            Request::Copy(new_path) => {
                if let Some(x) = self.find(&rqst.filepath){
                    if x.has_permission(&rqst.user, &Permission::Write){
                        let mut ofile = match File::open(Path::new(&rqst.filepath)){
                            Ok(x) => Ok(x),
                            Err(..) => Err("Could not open file to copy"),
                        };
                        if let Ok(x) = ofile{
                            let x = BufReader::new(x);
                            if let Some(ref y) = self.find(new_path){
                                if y.has_permission(&rqst.user, &Permission::Write){
                                    let mut nfile = match File::create(Path::new(&new_path)){
                                        Ok(y) => Ok(y),
                                        Err(..) => Err("Could not create file"),
                                    };
                                    if let Ok(ref mut y) = nfile{
                                        return match y.write_all(&x.buffer()) {
                                            Ok(..) => nfile,
                                            Err(..) => Err("Writing Failed")
                                        };
                                        //self.files.push(FileInfo::new(new_path.clone(), new_path.clone(), rqst.user.clone()));
                                    }
                                    nfile
                                }
                                else{
                                    Err("Protected file already in target location")
                                }
                            }
                            else {
                                let mut nfile = match File::create(Path::new(&new_path)){
                                    Ok(y) => Ok(y),
                                    Err(..) => Err("Could not create file"),
                                };
                                if let Ok(ref mut y) = nfile{
                                    
                                    return match y.write_all(&x.buffer()) {
                                        Ok(..) => {
                                            self.files.push(FileInfo::new(new_path.clone(), new_path.clone(), rqst.user.clone()));
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
            Request::Del => {
                if let Some(ref x) = self.find(&rqst.filepath){
                    if x.has_permission(&rqst.user, &Permission::Write){
                        fs::remove_file(rqst.filepath).unwrap_or_else(|e| {println!("{:?}", e.kind())});
                        //self.files.swap_remove();
                        Err("File deleted")
                    }
                    else{
                        Err("You do not have permission to access this file")
                    }
                }
                else{
                    Err("No file to delete")
                }
                
            },
        }
    }
}