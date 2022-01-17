# Single-Api

You are correct, the model I've used in the `single-api` is making it more difficult to understand how the things are connected.

I've created a new model that might make things easier to understand.
This is example is aligned to your discussion example.

## Model

The model is two tables, one containing all the registered bowls `bowls` and the other containing all waterlevels `waterlevels` for all the bowls.

```SQL
CREATE TABLE bowls (
    id integer primary key AUTOINCREMENT,
    name TEXT not null
);

CREATE TABLE waterlevels (
    date DATETIME PRIMARY KEY,
    id integer not null,
    waterlevel integer not null
);
```

## Relations

A `Bowl` entry in the `bowls`-table is just an `id`, and `name` of the bowl.
All it's related `Waterlevel` data is stored in the `waterlevels` table for together with all the other bowls data.

## Structs

### Create

For each struct there is created a `BowlDTO` that is used to deserialize the unsafe object into a `Bowls`.
The same goes for `Waterlevels`.

