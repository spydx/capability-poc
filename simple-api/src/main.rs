use actix_web::middleware::Logger;
use actix_web::web::{self};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
#[allow(unused_imports)]
use actix_web_httpauth::middleware::HttpAuthentication;
use capabilities::capabilities_derive::capabilities;
use capabilities::capability;
use capabilities::service;
use capabilities::SqliteDb;
#[allow(unused_imports)]
use capabilities::{Create, Delete, DeleteAll, Read, ReadAll, Update, UpdateAll};
use chrono::{serde::ts_seconds, NaiveDateTime, TimeZone, Utc};
use serde::Serializer;
use serde::{Deserialize, Serialize};
pub mod filter;

/*
    DTO - Data Transfere Objects
    Insecure by default
*/
#[derive(Deserialize)]
pub struct BowlsDTO {
    name: String,
}

#[derive(Deserialize)]

pub struct WaterlevelsDTO {
    #[allow(dead_code)] //TODO: remove once this is used.
    id: i64,
    #[allow(dead_code)] //TODO: remove once this is used.
    waterlevel: i64,
}

/*
   Capability structs
*/
#[capabilities(Create, Delete, id = "id")]
#[derive(Serialize, Debug)]
pub struct Bowls {
    id: i64,
    name: String,
}

#[capabilities(Create, Read, Delete, ReadAll, id = "id")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Waterlevels {
    #[warn(dead_code)]
    id: i64,
    #[serde(serialize_with = "serialize_dt")]
    date: Option<NaiveDateTime>,
    waterlevel: i64,
}

pub fn serialize_dt<S>(nt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = &Utc.from_local_datetime(&nt.unwrap()).unwrap();
    ts_seconds::serialize(dt, serializer)
}

#[service(SqliteDb, name = "db")]
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");

    let root = "/api";
    let binding = "0.0.0.0:8080";

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let con_str = "sqlite:test.db".to_string();
    let service = CapService::build(con_str)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&service.db)
        .await
        .expect("Failed to run sql mig on database");

    let bearer_auth = HttpAuthentication::bearer(filter::token_introspection);
    HttpServer::new(move || {
        App::new()
            .wrap(bearer_auth.clone())
            .wrap(Logger::default())
            .service(
                web::scope(&root)
                    .service(hello)
                    .service(create_new_bowl)
                    .service(get_bowl)
                    .service(get_bowl_waterlevel)
                    .service(add_bowl_waterlevel)
                    .service(get_all_waterlevels),
            )
            .app_data(web::Data::new(service.clone()))
    })
    .bind(binding)?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello from server")
}

/*
    HTTP, prosessing data to the service layer.
*/
#[post("/bowls/")]
pub async fn create_new_bowl(
    json: web::Json<BowlsDTO>,
    svc: web::Data<CapService>,
) -> impl Responder {
    let svc = svc.get_ref();
    let newbowl: Bowls = Bowls {
        id: 0,
        name: json.name.to_owned(),
    };

    println!("{:#?}", newbowl);
    let res_bowl = create_db_bowl(svc, newbowl).await;

    match res_bowl {
        Ok(bowl) => HttpResponse::Ok().json(bowl),
        _ => HttpResponse::BadRequest().json("{ \"request\": \"bad request\" "),
    }
}

#[get("/bowls/{id}")]
pub async fn get_bowl(_bowl_id: web::Path<String>, svc: web::Data<CapService>) -> impl Responder {
    let _svc = svc.get_ref();

    HttpResponse::Ok().json("Not implemented")
    /*let name = BowlsId { id: bowl_id.as_str().to_string()};
    let res_bowl = read_db_bowl_by_id(svc,name ).await;

    match res_bowl {
        Ok(bowl) => HttpResponse::Ok().json(bowl),
        _ => HttpResponse::NoContent().json("{ msg : no content } ")
    }
    */
}

#[get("/bowls/waterlevels/{id}")]
pub async fn get_bowl_waterlevel(
    _bowl_id: web::Path<String>,
    _pool: web::Data<CapService>,
) -> impl Responder {
    HttpResponse::Ok().body("Not Implemented")
}

#[post("/bowls/waterlevels/{id}")]
pub async fn add_bowl_waterlevel(
    _bowl_id: web::Path<String>,
    _json: web::Form<WaterlevelsDTO>,
    _pool: web::Data<CapService>,
) -> impl Responder {
    HttpResponse::Ok().body("Not Implemented")
}

#[get("/bowls/waterlevels/")]
pub async fn get_all_waterlevels(_pool: web::Data<CapService>) -> impl Responder {
    HttpResponse::Ok().body("Not Implemented")
}

/*
    Service layer, storing data in database.
*/
#[capability(Create, Bowls)]
pub fn create_db_bowl(bowl: Bowls) -> Result<Bowls, CapServiceError> {
    let _res = sqlx::query!(r#"INSERT INTO bowls (name) VALUES ($1)"#, bowl.name)
        .execute(&self.db)
        .await
        .expect("unable to create bowl");
    let b = sqlx::query_as!(Bowls, r#"SELECT * FROM bowls WHERE name = $1"#, bowl.name)
        .fetch_one(&self.db)
        .await
        .expect("Didn't fint any bowls");

    Ok(b)
}

#[capability(Delete, Bowls)]
pub fn delete_db_bowl(bowl: Bowls) -> Result<Bowls, CapServiceError> {
    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE name = $1"#, bowl.name)
        .execute(&self.db)
        .await
        .expect("unable to delete bowl");

    Ok(Bowls {
        id: 0,
        name: bowl.name,
    })
}

#[capability(Delete, Bowls, id = "i64")]
pub fn delete_db_bowl_by_id(bowl_id: BowlsId) -> Result<Bowls, CapServiceError> {
    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE id = $1"#, bowl_id.id)
        .execute(&self.db)
        .await
        .expect("unable to delete bowl");

    Ok(Bowls {
        id: bowl_id.id,
        name: "DELETED".to_string(),
    })
}

#[capability(Create, Waterlevels)]
pub fn create_db_waterlevels(waterlevel: Waterlevels) -> Result<Waterlevels, CapServiceError> {
    sqlx::query!(
        "INSERT INTO waterlevels (id, date, waterlevel) VALUES ($1, $2, $3)",
        waterlevel.id,
        waterlevel.date,
        waterlevel.waterlevel
    )
    .execute(&self.db)
    .await
    .expect("Failed to insert to database");
    Ok(waterlevel)
}

#[capability(Read, Waterlevels, id = "i64")]
pub fn get_db_waterlevel_by_id(
    waterlevel_id: WaterlevelsId,
) -> Result<Waterlevels, CapServiceError> {
    let waterlevel = sqlx::query_as!(
        Waterlevels,
        r#"SELECT * FROM waterlevels WHERE id = $1"#,
        waterlevel_id.id
    )
    .fetch_one(&self.db)
    .await
    .expect(format!("Failed to fetch bowl with id: {}", waterlevel_id.id).as_str());

    Ok(waterlevel)
    /*let time = Utc::now().to_string();
    let nt = NaiveDateTime::parse_from_str(&time, "%m-%d-%Y %H:%M:%S").expect("parsed not ok");

    Ok(Waterlevels { id: waterlevel_id.id, date: Some(nt), waterlevel: 78})*/
}

#[capability(Read, Waterlevels)]
pub fn get_db_waterlevel(waterlevel: Waterlevels) -> Result<Waterlevels, CapServiceError> {
    let bowl = sqlx::query_as!(
        Waterlevels,
        r#"SELECT * FROM waterlevels WHERE date = $1"#,
        waterlevel.date
    )
    .fetch_one(&self.db)
    .await
    .expect(format!("Failed to fetch bowl with id: {}", waterlevel.id).as_str());
    Ok(bowl)
}

#[capability(ReadAll, Waterlevels)]
pub fn get_db_all_waterlevels() -> Result<Vec<Waterlevels>, CapServiceError> {
    let waterlevels: Vec<Waterlevels> =
        sqlx::query_as!(Waterlevels, r#"SELECT * FROM waterlevels"#)
            .fetch_all(&self.db)
            .await
            .expect("Failed to query database for all bowls");

    Ok(waterlevels)
}

#[capability(Delete, Waterlevels)]
pub fn delete_db_waterlevel(waterlevel: Waterlevels) -> Result<Waterlevels, CapServiceError> {
    let _res = sqlx::query!(
        r#"DELETE FROM waterlevels WHERE date = $1"#,
        waterlevel.date
    )
    .execute(&self.db)
    .await
    .expect("Failed to delete");
    Ok(Waterlevels {
        id: waterlevel.id,
        date: waterlevel.date,
        waterlevel: waterlevel.waterlevel,
    })
}

#[capability(Delete, Waterlevels, id = "DateTime")]
pub fn delete_db_waterlevel_by_id(waterlevel: WaterlevelsId) -> Result<(), CapServiceError> {
    let _res = sqlx::query!(r#"DELETE FROM waterlevels WHERE id = $1"#, waterlevel.id)
        .execute(&self.db)
        .await
        .expect("Failed to delete");
    let time = Utc::now().to_string();
    let nt = NaiveDateTime::parse_from_str(&time, "%m-%d-%Y %H:%M:%S").expect("parsed not ok");

    Ok(Waterlevels {
        id: waterlevel.id,
        date: Some(nt),
        waterlevel: 0,
    })
}

/*
#[capability(Read, Bowls, id = "i64")]
pub fn read_db_bowl_by_id(bowl_id: i64) -> Result<Bowls, CapServiceError> {
    let b = sqlx::query_as!(Bowls, r#"SELECT * FROM bowls WHERE name = $1"#,
        bowl_id)
        .fetch_one(&self.db)
        .await
        .expect("Failed to get a bowl");

    Ok(b)
}


#[capability(Read, Bowls)]
pub fn read_db_bowl(bowl: Bowls) -> Result<Bowls, CapServiceError> {
    let b = sqlx::query_as!(Bowls, r#"SELECT * FROM bowls WHERE name = $1"#,
        bowl.name)
        .fetch_one(&self.db)
        .await
        .expect("Failed to get a bowl");

    Ok(b)
}
*/
