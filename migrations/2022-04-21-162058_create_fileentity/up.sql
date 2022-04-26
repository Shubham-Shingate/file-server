-- Your SQL goes here
CREATE TABLE fileentity (
	file_id serial PRIMARY KEY,
	filepath VARCHAR ( 255 ) UNIQUE NOT NULL
);