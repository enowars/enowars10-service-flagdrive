CREATE TABLE IF NOT EXISTS server_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    username TEXT PRIMARY KEY,
    user_password TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    encryption_key TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS follows (
    followee TEXT NOT NULL,
    follower TEXT NOT NULL,
    PRIMARY KEY (followee, follower),
    FOREIGN KEY (followee) REFERENCES users (username) ON DELETE CASCADE,
    FOREIGN KEY (follower) REFERENCES users (username) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    owner TEXT NOT NULL,
    visibility INTEGER NOT NULL,
    size INTEGER NOT NULL,
    content BLOB NOT NULL,
    created_at INTEGER NOT NULL,
    protection_key TEXT NOT NULL,
    is_protected INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (owner) REFERENCES users (username) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS gdpr_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    nonce TEXT NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (username) REFERENCES users (username) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS auth_token (
    token TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (username) REFERENCES users (username) ON DELETE CASCADE
);
