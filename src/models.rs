use crate::schema::accounts;
use crate::schema::fileentity;

#[derive(Queryable)]
pub struct Account {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl NewAccount {
    pub fn new(username: String, password: String, email: String) -> NewAccount {
        NewAccount {
            username,
            password,
            email
        }
    }
}

//Siva
#[derive(Queryable)]
pub struct FileEntity {
    pub file_id: i32,
    pub filepath: String,
}

#[derive(Insertable)]
#[table_name = "fileentity"]
pub struct NewFileEntity {
    pub filepath: String
}

impl NewFileEntity {
    pub fn new(filepath: String) -> NewFileEntity {
        NewFileEntity {
            filepath
        }
    }
}