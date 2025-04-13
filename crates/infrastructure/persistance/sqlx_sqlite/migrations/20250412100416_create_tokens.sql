-- Add migration script here
CREATE TABLE IF NOT EXISTS tokens (
    token TEXT NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);