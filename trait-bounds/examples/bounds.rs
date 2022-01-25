use std::fmt::Debug;
use std::marker::{Send, Sync};
use rand;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::{SqlitePoolOptions};
use async_trait::async_trait;
use tokio::task::LocalSet;

#[derive(Debug)]
struct Person {
    id: i64, 
    firstname: String,
    lastname: String,
}

struct Save<T, U>;

#[async_trait]
impl CapSave for Save {
    async fn save<T,U: CapSaveDB>(data: T, db: &U) {
        db.save(db, data: data).await;
    }
}


#[async_trait]
trait CapSave {
    async fn save(&self, db: &Pool<Sqlite>);
}

struct Service {
    con: Pool<Sqlite>
}

#[async_trait]
trait CapSaveDB<U>{
    async fn save<T: Sync + Send + CapSave + Debug>(db: U, data: &T);
}

#[async_trait]
impl CapSaveDB for Service {
    async fn save<T: Sync + Send + CapSave + Debug>(&self,data: &T) {
        println!("{:#?}", data);
        sqlx::query!(r#"INSERT  INTO person (id, firstname, lastname) VALUES ($1, $2, $3)"#,
            data.id,
            data.fistname,
            data.lastname)
            .execute(db)
            .await
            .expect("Failed to write to db");
    }
}

trait CapCreate { 
    fn create(firstname: String, lastname: String) -> Self;
}

impl CapCreate for Person {
    fn create(firstname: String, lastname: String) -> Self {
        let id = rand::random();
        Self {
            id: id,
            firstname: firstname,
            lastname: lastname,
        }
    }
}


#[tokio::main]
async fn main() {
    let con_str = "sqlite:../persons.db";
    let db: Pool<Sqlite> = SqlitePoolOptions::new()
        .connect(con_str)
        .await
        .expect("Failed to create database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let _service = Service { con: db };

    let _p1 = Person::create("Kenneth".to_string(), "Fossen".to_string());

    _p1.save(&_service.con).await;
    Service::save(&_service, &_p1).await;
}