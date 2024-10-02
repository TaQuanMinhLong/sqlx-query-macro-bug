-- Add migration script here
CREATE TABLE IF NOT EXISTS
    users_2 (id SERIAL PRIMARY KEY NOT NULL, name TEXT NOT NULL, age INT NOT NULL);