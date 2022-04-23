#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use diesel::PgConnection;
use models::{Account, NewAccount, FileEntity, NewFileEntity, AccountsFileMapping, NewAccountsFileMapping};

pub struct PgPersistance {}

impl PgPersistance {
    pub fn get_connection() -> PgConnection {
        let db_url = "postgres://postgres:Shubham2234@fileserverdb.ceqjhfvbhjd1.us-east-2.rds.amazonaws.com:5432/FILE_SERVER_DB";
        let db_connection: PgConnection =
            PgConnection::establish(db_url).expect(&format!("Error connecting to {}", db_url));
        return db_connection;
    }

    pub fn save_new_acc(connection: &PgConnection, username: String, password: String, email: String) -> Account {
        use schema::accounts;
        let new_acc = NewAccount::new(username, password, email);
        diesel::insert_into(accounts::table)
                    .values(&new_acc)
                    .get_result::<Account>(&*connection)
                    .expect("Error adding new account")
    }
    
    pub fn find_all_acc(connection: &PgConnection) -> Vec<Account> {
        use schema::accounts::dsl::*;

        let all_accounts = accounts
            .load::<Account>(connection)
            .expect("Error getting all the accounts");
        return all_accounts;
    }
 
    pub fn find_by_username(connection: &PgConnection, usr_name: &str) -> Option<Account> {
        use schema::accounts::dsl::*;

        let acc_found: Vec<Account> = accounts
                                        .filter(username.eq(usr_name))
                                        .load::<Account>(connection).unwrap();
        if acc_found.len() == 0 {
            Option::None
        } else{
            Option::Some(acc_found[0].clone())
        }
    }

    //This is just fetching all file paths from PostGreSQL DB (not the actual file as its stored in file system)
    pub fn find_all_files(connection: &PgConnection) -> Vec<FileEntity> {
        use schema::fileentity::dsl::*;

        let all_files = fileentity
            .load::<FileEntity>(connection)
            .expect("Error getting all the accounts");
        return all_files;
    }

    //This is just saving a new file path to PostGreSQL DB (not the actual file as its stored in file system)
    pub fn save_new_file(connection: &PgConnection, filepath: String) {
        use schema::fileentity;

        let new_file_entity = NewFileEntity::new(filepath);
        diesel::insert_into(fileentity::table)
                    .values(&new_file_entity)
                    .get_result::<FileEntity>(&*connection)
                    .expect("Error adding new file path");

    }

    pub fn save_new_acc_file_mapping(connection: &PgConnection, user_id: i32, file_id: i32, permissions: String) {
        use schema::accounts_file_mapping;

        let new_acc_file_mapping = NewAccountsFileMapping::new(user_id, file_id, permissions);
        diesel::insert_into(accounts_file_mapping::table)
                    .values(&new_acc_file_mapping)
                    .get_result::<AccountsFileMapping>(&*connection)
                    .expect("Error adding a new account-file mapping");

    }


}
