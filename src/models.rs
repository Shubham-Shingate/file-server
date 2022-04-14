
#[derive(Queryable)]
pub struct Account {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}