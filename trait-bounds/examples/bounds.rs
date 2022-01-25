use async_trait::async_trait;
use rand;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
struct Person {
    id: i64,
    firstname: String,
    lastname: String,
}

trait CreateRead<S> {
    fn create(firstname: String, lastname: String) -> S;
    fn read(id: i64) -> S;
}

impl CreateRead<Person> for Person {
    fn read(id: i64) -> Person {
        Person {
            id: id,
            firstname: "kenneth".to_string(),
            lastname: "fossen".to_string(),
        }
    }

    fn create(firstname: String, lastname: String) -> Person {
        Person {
            id: rand::random(),
            firstname: firstname,
            lastname: lastname,
        }
    }
}

struct Service {
    con: Pool<Sqlite>,
}

#[async_trait]
trait DBCreateRead<T: CreateRead<T>> {
    async fn read_db(&self, id: i64) -> T;
    async fn create_db(&self, data: T) -> T;
}

#[async_trait]
impl DBCreateRead<Person> for Service {
    async fn read_db(&self, id: i64) -> Person {
        let r = sqlx::query!(
            r#"SELECT id, firstname, lastname FROM person WHERE id = $1"#,
            id
        )
        .fetch_one(&self.con)
        .await
        .expect("Failed to query database");

        Person {
            id: id,
            firstname: r.firstname,
            lastname: r.lastname,
        }
    }

    async fn create_db(&self, data: Person) -> Person {
        let _r = sqlx::query!(
            r#"INSERT INTO person (id, firstname, lastname) VALUES ($1, $2, $3)"#,
            data.id,
            data.firstname,
            data.lastname
        )
        .execute(&self.con)
        .await
        .expect("Failed to insert Person into database");
        data
    }
}

#[tokio::main]
async fn main() {
    let con_str = "sqlite:bounds_persons.db";
    let db: Pool<Sqlite> = SqlitePoolOptions::new()
        .connect(con_str)
        .await
        .expect("Failed to create database");

    let _service = Service { con: db };

    let _p1 = Person::create("Kenneth".to_string(), "Fossen".to_string());

    let create_res = Service::create_db(&_service, _p1).await;

    let read_res = Service::read_db(&_service, create_res.id).await;

    assert_eq!(create_res, read_res);
    println!("Create<{:#?}> -> Read<{:#?}>", create_res, read_res);
}
