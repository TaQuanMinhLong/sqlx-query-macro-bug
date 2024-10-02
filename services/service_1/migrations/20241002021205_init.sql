-- Add migration script here
CREATE TABLE IF NOT EXISTS
    users_1 (id SERIAL PRIMARY KEY NOT NULL, name TEXT NOT NULL, age INT NOT NULL);