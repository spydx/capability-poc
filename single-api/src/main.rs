use capabilities::service;
use capabilities::capability;
use capabilities::SqliteDb;
//use chrono::{NaiveDateTime, Utc};
use capabilities::capabilities_derive::capabilities;
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use actix_web::{App, HttpServer, HttpResponse,  Responder};
use actix_web::web::{self};
#[allow(unused_imports)]
use actix_web_httpauth::middleware::HttpAuthentication;
#[allow(unused_imports)]
use capabilities::{Create, Delete, Update, Read, ReadAll, DeleteAll, UpdateAll};

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

    //let middleware = HttpAuthentication::bearer(auth::token_validator);
    HttpServer::new(move || {
        App::new()
            //.wrap(middleware.clone())
            .wrap(Logger::default())
            //.route("/bowls/", web::get().to())
            .route("/bowls/", web::post().to(create_new_bowl))
            //.route("/bowls/{id}", web::get().to())
            //.route("/bowls/{id}/waterlevels/", web::get().to())
            //.route("/bowls/{id}/waterlevels/", web::post().to())
            .app_data(service.clone())
    })
    .bind("0.0.0.0:8080")?
    .run().await?;

    Ok(())
}


pub async fn create_new_bowl(form: web::Form<BowlsDTO>, pool: web::Data<SqliteDb>) -> impl Responder {
    let _db = pool.get_ref();
    let _b : BowlsDTO = BowlsDTO {
        name: form.name.to_owned(),
    };

    HttpResponse::Ok()

}

#[derive(Deserialize)]
pub struct BowlsDTO {
    name: String
}

#[capabilities(Create, Delete, id = "id")]
#[derive(Serialize)]
pub struct Bowls {
    id: i64, 
    name: String,
}

#[capability(Create, Bowls)]
pub fn create_bowl(bowl: BowlsDTO) -> Result<Bowls, CapServiceError> {
    let _res = sqlx::query!(r#"INSERT INTO bowls (name) VALUES ($1)"#,
            bowl.name)
            .execute(&self.db)
            .await
            .expect("unable to create bowl");

    Ok(Bowls { id: 0, name: bowl.name})
}


#[capability(Delete, Bowls)]
pub fn delete_bowl(bowl: BowlsDTO) -> Result<Bowls, CapServiceError> {
    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE name = $1"#,
            bowl.name)
            .execute(&self.db)
            .await
            .expect("unable to delete bowl");

    Ok(Bowls { id: 0, name: bowl.name})
}

#[capability(Delete, Bowls, id = "i64")]
pub fn delete_bowl_by_id(bowl_id: i64) -> Result<Bowls, CapServiceError> {
    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE id = $1"#,
            bowl_id)
            .execute(&self.db)
            .await
            .expect("unable to delete bowl");

    Ok(Bowls { id: bowl_id, name: "DELETED".to_string()})
}

use chrono::NaiveDateTime;
#[capabilities(Create, Read, Delete, ReadAll, id = "id")]
#[derive(Debug, Deserialize, Serialize)]
pub struct BowlWaterlevel {
    #[warn(dead_code)]
    id: i64,
    date: NaiveDateTime,
    waterlevel: i64,
}

#[capability(Create, BowlWaterlevel)]
pub fn create_waterbowl(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
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
pub fn get_waterlevel_by_id(bowl_id: i64) -> Result<BowlWaterlevel, CapServiceError> {
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
pub fn get_waterlevel(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
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
pub fn get_all_waterlevels() -> Result<Vec<BowlWaterlevel>, CapServiceError> {
    let bowls: Vec<BowlWaterlevel> = sqlx::query_as!(BowlWaterlevel, r#"SELECT * FROM bowls"#)
        .fetch_all(&self.db)
        .await
        .expect("Failed to query database for all bowls");

    Ok(bowls)
}


#[capability(Delete, BowlWaterlevel, id = "i64")]
pub fn delete_waterlevel_by_id(bowl_id: i64) -> Result<(), CapServiceError> {
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
pub fn delete_waterlevel(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE id = $1"#,
            bowl.id)
        .execute(&self.db)
        .await
        .expect("Failed to delete");
    Ok(BowlWaterlevel { id: bowl.id, date: bowl.date , waterlevel: bowl.waterlevel })
}