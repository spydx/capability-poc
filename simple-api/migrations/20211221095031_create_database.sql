-- Add migration script here
CREATE TABLE bowls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE waterlevels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bowl_id INTEGER NOT NULL,
    date DATETIME,
    waterlevel INTEGER NOT NULL
);