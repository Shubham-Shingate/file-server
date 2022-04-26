use crate::schema::accounts;
use crate::schema::fileentity;
use crate::schema::accounts_file_mapping;

//Model i.e Entity for Account
#[derive(Queryable, Clone, Debug)]
pub struct Account {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable, Clone, Debug)]
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

//Model i.e Entity for FileEntity
#[derive(Queryable, Clone, Debug)]
pub struct FileEntity {
    pub file_id: i32,
    pub filepath: String,
}

#[derive(Insertable, Clone, Debug)]
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

//Model i.e Entity for AccountsFileMapping
#[derive(Queryable, Clone, Debug)]
pub struct AccountsFileMapping {
    pub mapping_id: i32,
    pub user_id: i32,
    pub file_id: i32,
    pub permissions: String,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "accounts_file_mapping"]
pub struct NewAccountsFileMapping {
    pub user_id: i32,
    pub file_id: i32,
    pub permissions: String
}

impl NewAccountsFileMapping {
    pub fn new(user_id: i32, file_id: i32, permissions: String) -> NewAccountsFileMapping {
        NewAccountsFileMapping {
            user_id,
            file_id,
            permissions
        }
    }
}