-- Give signup process states an explicit, persistent insertion order.
-- The implicit rowid is not stable schema state (VACUUM may renumber it);
-- an INTEGER PRIMARY KEY aliases the rowid and makes it persistent.
CREATE TABLE signup_process_states_new (
    history_id INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    username TEXT,
    email TEXT,
    password TEXT,
    error TEXT,
    state TEXT NOT NULL,
    entered_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO signup_process_states_new (id, username, email, password, error, state, entered_at)
    SELECT id, username, email, password, error, state, entered_at
    FROM signup_process_states
    ORDER BY rowid ASC;
DROP TABLE signup_process_states;
ALTER TABLE signup_process_states_new RENAME TO signup_process_states;
