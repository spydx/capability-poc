use capabilities::capabilities_derive::capabilities;
use chrono::NaiveDateTime;
use async_trait::async_trait;

#[capabilities(Create, Read, Delete, ReadAll, id = "id")]
#[derive(Debug)]
pub struct BowlWaterlevel {
    #[warn(dead_code)]
    id: i64,
    date: NaiveDateTime,
    waterlevel: i64,
}