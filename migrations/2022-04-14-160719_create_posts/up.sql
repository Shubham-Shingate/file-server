-- Your SQL goes here
CREATE TABLE accounts (
	user_id serial PRIMARY KEY,
	username VARCHAR ( 50 ) UNIQUE NOT NULL,
	password VARCHAR ( 50 ) NOT NULL,
	email VARCHAR ( 255 ) UNIQUE NOT NULL
);

CREATE TABLE fileentity (
	file_id serial PRIMARY KEY,
	filepath VARCHAR ( 255 ) UNIQUE NOT NULL
);