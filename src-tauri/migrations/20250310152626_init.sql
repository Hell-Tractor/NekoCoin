-- Add migration script here
CREATE TABLE wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    remark TEXT,
    balance INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL,
    color TEXT NOT NULL,
    icon TEXT NOT NULL
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    remark TEXT,
    color TEXT NOT NULL,
    icon TEXT NOT NULL,
    type INTEGER NOT NULL,
    parent_id INTEGER DEFAULT NULL,
    FOREIGN KEY(parent_id) REFERENCES tags(id)
);

CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    remark TEXT,
    wallet_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    date TEXT NOT NULL,
    FOREIGN KEY(wallet_id) REFERENCES wallets(id),
    FOREIGN KEY(tag_id) REFERENCES tags(id)
);