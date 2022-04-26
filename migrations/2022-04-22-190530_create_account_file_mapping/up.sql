-- Your SQL goes here
CREATE TABLE accounts_file_mapping (
	mapping_id serial PRIMARY KEY,
    user_id INTEGER NOT NULL,
	file_id INTEGER NOT NULL,
	permissions VARCHAR ( 20 ) NOT NULL
);