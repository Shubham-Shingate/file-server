#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use diesel::PgConnection;
use models::{Account, NewAccount, FileEntity};

pub struct PgPersistance {}

impl PgPersistance {
    pub fn get_connection() -> PgConnection {
        let db_url = "postgres://postgres:Shubham2234@fileserverdb.ceqjhfvbhjd1.us-east-2.rds.amazonaws.com:5432/FILE_SERVER_DB";
        let db_connection: PgConnection =
            PgConnection::establish(db_url).expect(&format!("Error connecting to {}", db_url));
        return db_connection;
    }

    pub fn find_all(connection: &PgConnection) -> Vec<Account> {
        use schema::accounts::dsl::*;

        let all_accounts = accounts
            .load::<Account>(connection)
            .expect("Error getting all the accounts");
        return all_accounts;
    }
 
    pub fn save_new_acc(connection: &PgConnection, user_id: i32, username: String, password: String, email: String) {
        use schema::accounts;
        let new_acc = NewAccount::new(user_id, username, password, email);
        diesel::insert_into(accounts::table)
                    .values(&new_acc)
                    .get_result::<Account>(&*connection)
                    .expect("Error adding new account");

    }


    pub fn find_all_files(connection: &PgConnection) -> Vec<FileEntity> {
        use schema::fileentity::dsl::*;

        let all_files = fileentity
            .load::<FileEntity>(connection)
            .expect("Error getting all the accounts");
        return all_files;
    }


}
