-- Add migration script here
CREATE TABLE wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    remark TEXT,
    balance INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL,
    color TEXT,
    icon TEXT
);