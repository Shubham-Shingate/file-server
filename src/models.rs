
#[derive(Queryable)]
pub struct Account {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

//Siva
#[derive(Queryable)]
pub struct FileEntity {
    pub file_id: i32,
    pub filepath: String,
}