CREATE TABLE IF NOT EXISTS users (
    uuid UUID PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    modified_at TIMESTAMPTZ NOT NULL
);-- Add up migration script here
