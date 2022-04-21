use crate::schema::accounts;

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
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl NewAccount {

    pub fn new(user_id: i32, username: String, password: String, email: String) -> NewAccount {
        NewAccount {
            user_id,
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