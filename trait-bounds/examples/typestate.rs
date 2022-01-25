use async_trait::async_trait;
use person::*;
use sqlx::{Pool, Sqlite, SqlitePool};

mod person {
    use async_trait::async_trait;
    use sqlx::{Pool, Sqlite};

    #[derive(Debug, PartialEq)]
    pub struct Person<Cap: Caps> {
        pub id: i64,
        pub firstname: String,
        pub lastname: String,
        pub cap: Cap,
    }

    #[derive(Debug, PartialEq)]
    pub struct CreateRead;
    #[async_trait]
    pub trait CreateReadCap {
        async fn create(
            db: &Pool<Sqlite>,
            firstname: String,
            lastname: String,
        ) -> Person<CreateRead>;
        async fn read(db: &Pool<Sqlite>, id: i64) -> Person<CreateRead>;
    }
    /*pub struct Delete;
    pub trait DeleteCap {
        fn delete(db: &Pool<Sqlite>,) -> Person<Delete>;
    }
    pub struct CreateDelete;
    pub trait CreateDeleteCap {
        fn create(db: &Pool<Sqlite>,firstname: String, lastname: String) -> Person<CreateDelete>;
        fn delete(db: &Pool<Sqlite>,) -> Person<CreateDelete>;
    }
    pub struct CreateUpdate;
    pub trait CreateUpdateCap {
        fn create(db: &Pool<Sqlite>,firstname: String, lastname: String) -> Person<CreateUpdate>;
        fn update(db: &Pool<Sqlite>) -> Person<CreateUpdate>;
    }
    pub struct CreateReadUpdate;
    pub trait CreateReadUpdateCap {
        fn create(db: &Pool<Sqlite>,firstname: String, lastname: String) -> Person<CreateReadUpdate>;
        fn read(db: &Pool<Sqlite>) -> Person<CreateReadUpdate>;
        fn update(db: &Pool<Sqlite>,fistname: String, lastname: String) -> Person<CreateReadUpdate>;
    }
    */
    mod __private {
        pub trait Caps {}
    }

    pub trait Caps: __private::Caps {}
    impl<__T: ?::core::marker::Sized> Caps for __T where __T: __private::Caps {}
    #[async_trait]
    impl __private::Caps for CreateRead {}
    /*
    impl __private::Caps for Delete {}
    impl __private::Caps for CreateDelete {}
    impl __private::Caps for CreateUpdate {}
    impl __private::Caps for CreateReadUpdate {}
    */
}

#[async_trait]
impl CreateReadCap for Person<CreateRead> {
    async fn create(db: &Pool<Sqlite>, fistname: String, lastname: String) -> Person<CreateRead> {
        let person = Person::<CreateRead> {
            id: rand::random(),
            firstname: fistname,
            lastname: lastname,
            cap: CreateRead,
        };

        sqlx::query!(
            r#"INSERT INTO person (id, firstname, lastname) VALUES ($1, $2, $3)"#,
            person.id,
            person.firstname,
            person.lastname
        )
        .execute(db)
        .await
        .expect("Failed to write to database");
        person
    }

    async fn read(db: &Pool<Sqlite>, id: i64) -> Person<CreateRead> {
        let r = sqlx::query!(
            r#"SELECT id, firstname, lastname FROM person WHERE id = $1"#,
            id
        )
        .fetch_one(db)
        .await
        .expect("Failed to read database");

        Person::<CreateRead> {
            id: r.id,
            firstname: r.firstname,
            lastname: r.lastname,
            cap: CreateRead,
        }
    }
}

#[tokio::main]
async fn main() {
    let connection_string = "sqlite:persons.db";
    let database = SqlitePool::connect(connection_string)
        .await
        .expect("Failed to get database");

    let p_created =
        Person::<CreateRead>::create(&database, "Kenneth".to_string(), "Fossen".to_string()).await;

    let p_read = Person::<CreateRead>::read(&database, p_created.id).await;

    println!("{:#?}", p_created);
    println!("{:#?}", p_read);
    assert_eq!(
        p_created, p_read,
        "The to persons {:#?} and {:#?} are not equal",
        p_created, p_read
    );
}
