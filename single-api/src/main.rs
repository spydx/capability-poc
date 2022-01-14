use capabilities::service;
use capabilities::SqliteDb;
use chrono::{NaiveDateTime, Utc};
use single_api::service::{create_waterbowl};
use single_api::model::BowlWaterlevel;


#[service(SqliteDb, name = "db")]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let con_str = "sqlite:test.db".to_string();
    let service = CapService::build(con_str)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&service.db)
        .await
        .expect("Failed to run sql mig on database");

    let time = Utc::now().naive_utc();

    let b = BowlWaterlevel {
        id: 0,
        date: time,
        waterlevel: 100,
    };

    let c = create_waterbowl(&service, b)
        .await
        .expect("Failed to create bowl");
    println!("{:?}", c);

    Ok(())
}