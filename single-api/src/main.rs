use capabilities::capabilities_derive::capabilities;
use capabilities::capability;
use capabilities::service;
use capabilities::SqliteDb;
use chrono::{NaiveDateTime, Utc};

#[capabilities(Create, Read, Delete, ReadAll, id = "id")]
#[derive(Debug)]
pub struct BowlWaterlevel {
    #[warn(dead_code)]
    id: i64,
    date: NaiveDateTime,
    waterlevel: i64,
}

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

#[capability(Create, BowlWaterlevel)]
fn create_waterbowl(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
    sqlx::query!(
        "INSERT INTO bowls (date, waterlevel) VALUES ($1, $2)",
        bowl.date,
        bowl.waterlevel
    )
    .execute(&self.db)
    .await
    .expect("Failed to insert to database");
    Ok(bowl)
}

#[capability(Read, BowlWaterlevel, id = "i64")]
fn get_by_id(bowl_id: i64) -> Result<BowlWaterlevel, CapServiceError> {
    let bowl = sqlx::query_as!(
        BowlWaterlevel,
        r#"SELECT * FROM bowls WHERE id = $1"#,
        bowl_id
    )
    .fetch_one(&self.db)
    .await
    .expect(format!("Failed to fetch bowl with id: {}", bowl_id).as_str());
    Ok(bowl)
}

#[capability(Read, BowlWaterlevel)]
fn get(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
    let bowl = sqlx::query_as!(
        BowlWaterlevel,
        r#"SELECT * FROM bowls WHERE id = $1"#,
        bowl.id
    )
    .fetch_one(&self.db)
    .await
    .expect(format!("Failed to fetch bowl with id: {}", bowl.id).as_str());
    Ok(bowl)
}

#[capability(ReadAll, BowlWaterlevel)]
fn get_all() -> Result<Vec<BowlWaterlevel>, CapServiceError> {
    let bowls: Vec<BowlWaterlevel> = sqlx::query_as!(BowlWaterlevel, r#"SELECT * FROM bowls"#)
        .fetch_all(&self.db)
        .await
        .expect("Failed to query database for all bowls");

    Ok(bowls)
}


#[capability(Delete, BowlWaterlevel, id = "i64")]
fn delete_bowl_by_id(bowl_id: i64) -> Result<(), CapServiceError> {
    let b = sqlx::query_as!(BowlWaterlevel,r#"SELECT * FROM bowls WHERE id = $1"#,
        bowl_id)
        .fetch_one(&self.db)
        .await
        .expect("Failed to find bowl");

    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE id = $1"#,
        bowl_id)
        .execute(&self.db)
        .await
        .expect("Failed to delete");
    Ok(b)
}


#[capability(Delete, BowlWaterlevel)]
fn delete_bowl(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE id = $1"#,
            bowl.id)
        .execute(&self.db)
        .await
        .expect("Failed to delete");
    Ok(BowlWaterlevel { id: bowl.id, date: bowl.date , waterlevel: bowl.waterlevel })
}