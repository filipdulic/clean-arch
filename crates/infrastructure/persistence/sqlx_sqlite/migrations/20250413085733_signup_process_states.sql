-- Add migration script here
CREATE TABLE IF NOT EXISTS signup_process_states (
    id TEXT NOT NULL,
    username TEXT,
    email TEXT,
    password TEXT,
    error TEXT,
    state TEXT NOT NULL,
    entered_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
