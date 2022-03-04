use std::collections::HashMap;

struct FileInfo{
    filename: String,
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
    filename: String,
    filepath: String,
    rqst_tp: Request,
}

enum Request{
    Read,
    Write,
    Del,
    Copy(String/*new name*/, String/*new path*/),
}

impl FileInfo{
    fn has_permission(&self, u: String, p: Permission) -> bool{
        self.permissions.get(&u) == Some(&p)
    }
    fn new(filename: String, filepath: String, o: String) -> FileInfo{
        let mut permissions = HashMap::new();
        permissions.insert(o, Permission::Owner);
        FileInfo{
            filename,
            filepath,
            permissions
        }
    }
}

fn file_rqst(rqst: FileRqst){
    // check permission & call fn for request type
}