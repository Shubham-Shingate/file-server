#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use diesel::PgConnection;
use models::Account;
use models::FileEntity;

pub struct PgPersistance {}

impl PgPersistance {
    pub fn get_connection() -> PgConnection {
        let db_url = "postgres://postgres:Shubham2234@localhost:5432/fileserverdb";
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

    pub fn find_all_files(connection: &PgConnection) -> Vec<FileEntity> {
        use schema::fileentity::dsl::*;

        let all_files = fileentity
            .load::<FileEntity>(connection)
            .expect("Error getting all the accounts");
        return all_files;
    }


}
