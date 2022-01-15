-- Add migration script here
CREATE TABLE bowls (
    id integer primary key AUTOINCREMENT,
    name TEXT not null
);

CREATE TABLE waterlevels (
    date DATETIME PRIMARY KEY,
    id integer not null,
    waterlevel integer not null
);