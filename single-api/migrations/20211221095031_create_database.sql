-- Add migration script here
CREATE TABLE bowls (
    id integer primary key AUTOINCREMENT,
    date DATETIME not null,
    waterlevel integer not null
);