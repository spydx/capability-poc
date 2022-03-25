use actix_web::middleware::Logger;
use actix_web::web::{self};
use actix_web::{get, post,delete, App, HttpResponse, HttpServer, Responder};
#[allow(unused_imports)]
use actix_web_httpauth::middleware::HttpAuthentication;
use capabilities::capabilities_derive::capabilities;
use capabilities::service;
use capabilities::SqliteDb;
use capabilities::{capability, token_introspection};
#[allow(unused_imports)]
use capabilities::{Create, Delete, DeleteAll, Read, ReadAll, Update, UpdateAll};
use chrono::{serde::ts_seconds, NaiveDateTime, TimeZone, Utc};
use gnap_cli::GnapClient;
use serde::Serializer;
use serde::{Deserialize, Serialize};

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
    waterlevel: i64,
}
/*
   Capability structs
*/
#[capabilities(Create, Read, Delete, id = "id")]
#[derive(Serialize, Debug)]
pub struct Bowl {
    id: i64,
    name: String,
}

#[capabilities(Create, Read, Delete, ReadAll, id = "id")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Waterlevel {
    id: i64,
    bowl_id: i64,
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

    let rs_ref = "e8a2968a-f183-45a3-b63d-4bbbd1dad276".to_string();
    let basepath = "http://localhost:8000/gnap".to_string();
    let gnap_client = GnapClient::build(basepath, rs_ref);

    let bearer_auth = HttpAuthentication::bearer(token_introspection);
    HttpServer::new(move || {
        App::new()
            .app_data(gnap_client.clone())
            .wrap(bearer_auth.clone())
            .wrap(Logger::default())
            .service(create_new_bowl)
            .service(get_bowl_by_id)
            .service(add_bowl_waterlevel)
            .service(get_all_bowl_waterlevels)
            .service(delete_bowl_waterlevels_by_id)
            .app_data(web::Data::new(service.clone()))
    })
    .bind(binding)?
    .run()
    .await?;

    Ok(())
}

/*
    HTTP, prosessing data to the service layer.
*/
#[post("/bowls/")]
pub async fn create_new_bowl(
    json: web::Json<BowlsDTO>,
    svc: web::Data<CapService>,
    cap: Capability,
) -> impl Responder {
    let svc = svc.get_ref();
    let newbowl: Bowl = Bowl {
        id: 0,
        name: json.name.to_owned(),
    };

    println!("{:#?}", newbowl);
    println!("Cap: {:#?}", cap);
    match create_db_bowl(svc, newbowl, cap).await {
        Ok(bowl) => HttpResponse::Ok().json(bowl),
        _ => HttpResponse::BadRequest().json("{ \"request\": \"bad request\" "),
    }
}

#[get("/bowls/{id}")]
pub async fn get_bowl_by_id(
    bowl_id: web::Path<String>,
    svc: web::Data<CapService>,
    cap: Capability,
) -> impl Responder {
    let svc = svc.get_ref();

    let id = bowl_id.into_inner();

    let bowl_id = BowlId {
        id: id.parse::<i64>().unwrap(),
    };
    println!("Finding: {:?}", id);
    println!("Cap: {:?}", cap);
    match read_db_bowl_by_id(svc, bowl_id, cap).await {
        Ok(bowl) => HttpResponse::Ok().json(bowl),
        _ => HttpResponse::NoContent().json("{ msg : no content } "),
    }
}


#[post("/waterlevels/{id}")]
pub async fn add_bowl_waterlevel(
    bowl_id: web::Path<String>,
    json: web::Form<WaterlevelsDTO>,
    svc: web::Data<CapService>,
    cap: Capability,
) -> impl Responder {
    let svc = svc.get_ref();

    let bowl_id = bowl_id.parse::<i64>().unwrap();
    let json = json.into_inner();
    let date: NaiveDateTime = Utc::now().naive_utc();
    
    let waterlevel = Waterlevel {
        id:  0,
        bowl_id,
        waterlevel: json.waterlevel,
        date: Some(date)
    };
    match create_db_waterlevels(svc, waterlevel, cap).await {
        Ok(d) => HttpResponse::Ok().json(d),
        _ => HttpResponse::BadRequest().body("malformed request")
    }
}

#[get("/waterlevels/{id}")]
pub async fn get_all_bowl_waterlevels(
    bowl_id: web::Path<String>,
    svc: web::Data<CapService>, 
    cap: Capability) -> impl Responder {
    let svc = svc.get_ref();
    let bowl_id = WaterlevelId { id: bowl_id.parse::<i64>().unwrap() };
    
    match read_db_waterlevel_by_id(svc, bowl_id, cap).await {
        Ok(d) => HttpResponse::Ok().json(d),
        _ => HttpResponse::Forbidden().body("no access")
    }
}

#[delete("/waterlevels/{id}")]
pub async fn delete_bowl_waterlevels_by_id(
    bowl_id: web::Path<String>,
    svc: web::Data<CapService>,
    cap: Capability) -> impl Responder {
    let svc = svc.get_ref();

    let bowl_id = WaterlevelId { id: bowl_id.parse::<i64>().unwrap() };

    match delete_db_waterlevel_by_id(svc, bowl_id, cap).await {
        Ok(_) => HttpResponse::Ok().json("success"),
        _ => HttpResponse::Forbidden().json("no access")
    }
}


/*
    Service layer, storing data in database.
*/
#[capability(Create, Bowl)]
pub fn create_db_bowl(bowl: Bowl) -> Result<Bowl, CapServiceError> {
    let _res = sqlx::query!(r#"INSERT INTO bowls (name) VALUES ($1)"#, bowl.name)
        .execute(&self.db)
        .await
        .expect("unable to create bowl");
    let b = sqlx::query_as!(Bowl, r#"SELECT * FROM bowls WHERE name = $1"#, bowl.name)
        .fetch_one(&self.db)
        .await
        .expect("Didn't fint any bowls");

    Ok(b)
}

#[capability(Delete, Bowl)]
pub fn delete_db_bowl(bowl: Bowl) -> Result<(), CapServiceError> {
    match sqlx::query!(r#"DELETE FROM bowls WHERE name = $1"#, bowl.name)
        .execute(&self.db)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(CapServiceError),
    }
}

#[capability(Delete, Bowl, id = "i64")]
pub fn delete_db_bowl_by_id(bowl_id: Bowl) -> Result<(), CapServiceError> {
    match sqlx::query!(r#"DELETE FROM bowls WHERE id = $1"#, bowl_id.id)
        .execute(&self.db)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(CapServiceError),
    }
}

#[capability(Create, Waterlevel)]
pub fn create_db_waterlevels(waterlevel: Waterlevel) -> Result<Waterlevel, CapServiceError> {
    sqlx::query!(
        "INSERT INTO waterlevels (bowl_id, date, waterlevel) VALUES ($1, $2, $3)",
        waterlevel.bowl_id,
        waterlevel.date,
        waterlevel.waterlevel
    )
    .execute(&self.db)
    .await
    .expect("Failed to insert to database");
    Ok(waterlevel)
}

#[capability(Read, Waterlevel, id = "i64")]
pub fn read_db_waterlevel_by_id(
    waterlevel_id: WaterlevelId,
) -> Result<Waterlevel, CapServiceError> {
    let waterlevel = sqlx::query_as!(
        Waterlevel,
        r#"SELECT * FROM waterlevels WHERE bowl_id = $1"#,
        waterlevel_id.id
    )
    .fetch_one(&self.db)
    .await
    .unwrap_or_else(|_| 
        panic!("Failed to fetch bowl with id: {}", waterlevel_id.id)    
    );

    Ok(waterlevel)
}

#[capability(Read, Waterlevel)]
pub fn get_db_waterlevel(waterlevel: Waterlevel) -> Result<Waterlevel, CapServiceError> {
    let bowl = sqlx::query_as!(
        Waterlevel,
        r#"SELECT * FROM waterlevels WHERE id = $1"#,
        waterlevel.id
    )
    .fetch_one(&self.db)
    .await
    .unwrap_or_else(|_| panic!("Failed to fetch bowl with id: {}", waterlevel.id));
    Ok(bowl)
}

#[capability(ReadAll, Waterlevel)]
pub fn read_db_all_waterlevels() -> Result<Vec<Waterlevel>, CapServiceError> {
    let waterlevels: Vec<Waterlevel> =
        sqlx::query_as!(Waterlevel, r#"SELECT * FROM waterlevels"#)
            .fetch_all(&self.db)
            .await
            .expect("Failed to query database for all bowls");

    Ok(waterlevels)
}

#[capability(Delete, Waterlevel)]
pub fn delete_db_waterlevel(waterlevel: Waterlevel) -> Result<(), CapServiceError> {
    match sqlx::query!(r#"DELETE FROM waterlevels WHERE id = $1"#, waterlevel.id)
        .execute(&self.db)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(CapServiceError),
    }
}

#[capability(Delete, Waterlevel, id = "i64")]
pub fn delete_db_waterlevel_by_id(waterlevel: Waterlevel) -> Result<(), CapServiceError> {
    match sqlx::query!(r#"DELETE FROM waterlevels WHERE id = $1"#, waterlevel.id)
        .execute(&self.db)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(CapServiceError),
    }
}

#[capability(Read, Bowl, id = "i64")]
pub fn read_db_bowl_by_id(bowl_id: BowlId) -> Result<Bowl, CapServiceError> {
    let b = sqlx::query_as!(Bowl, r#"SELECT * FROM bowls WHERE id = $1"#, bowl_id.id)
        .fetch_one(&self.db)
        .await
        .expect("Failed to get a bowl");
    Ok(b)
}

#[capability(Read, Bowl)]
pub fn read_db_bowl(bowl: Bowl) -> Result<Bowl, CapServiceError> {
    let b = sqlx::query_as!(Bowl, r#"SELECT * FROM bowls WHERE name = $1"#, bowl.name)
        .fetch_one(&self.db)
        .await
        .expect("Failed to get a bowl");

    Ok(b)
}
