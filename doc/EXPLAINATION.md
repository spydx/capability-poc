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

Each table has its own struct decorated with the Capabilites we would like.

Right now, the `Bowls` has only, `Create` and `Delete`.
```rust
#[capabilities(Create, Delete, id = "id")]
#[derive(Serialize, Debug)]
pub struct Bowls {
    id: i64, 
    name: String,
}
````

`Waterlevels` has `Create`, `Read`, `Delete`, and `ReadAll`

```rust
#[capabilities(Create, Read, Delete, ReadAll, id = "id")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Waterlevels {
    #[warn(dead_code)]
    id: i64,                    // Id of the bowl
    #[serde(serialize_with = "serialize_dt")]
    date: Option<NaiveDateTime>,
    waterlevel: i64,
}
```

### Create

For the struct `Bowls` there is created a `BowlDTO`, this  is used to deserialize the unsafe object into a `Bowls`.

```sh
curl -v  --header "Content-Type: application/json" \
    --request POST \
    --data '{ "name": "Boisy" }' \
    http://localhost:8080/api/bowls/
```

User sends -> `BowlsDto` -> JWT filter -> fn create_new_bowl -> fn create_db_bowls -> to disk

Note that the JWT filter is not implemented yet.

When we look at the expanded code for `Bowls`, this is what is created:
It will generate all the traits for both capabilities.

```rust
pub struct Bowls {
    id: i64,
    name: String,
}
pub struct BowlsId { // this is a struct that contains the id field
    id: i64,
}
pub trait CapCreateBowls: 
    Capability<Create<Bowls>, Data = Bowls, Error = CapServiceError> {} // Create interface
impl CapCreateBowls for CapService {}

pub trait CapDeleteBowlsId: 
    Capability<Delete<BowlsId>, Data = Bowls, Error = CapServiceError> {} // Delete by Id interface
impl CapDeleteBowlsId for CapService {}

pub trait CapDeleteBowls: 
    Capability<Delete<Bowls>, Data = Bowls, Error = CapServiceError> {} // Delete interface

impl CapDeleteBowls for CapService {}
```


```rust
pub async fn create_db_bowl<Service>(
    service: &Service,
    param: Bowls,
) -> Result<Bowls, CapServiceError>
where
    Service: CapCreateBowls, //the trait that we need to do this job
{
    service
        .perform(::capabilities::Create { data: param }) // the action itself
        .await
}
impl Capability<Create<Bowls>> for CapService { //the trait that we are implementing
    type Data = Bowls;
    type Error = CapServiceError;

    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Create<Bowls>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let bowl: Bowls = action.data;
                {   // this is the code block that we create and return a Bowl with
                    let _res = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("INSERT INTO bowls (name) VALUES ($1)" , query_args) } } } . execute (& __self . db) . await . expect ("unable to create bowl") ;
                    let b = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM bowls WHERE name = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (0usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (1usize) ? ; Ok (Bowls { id : sqlx_query_as_id , name : sqlx_query_as_name , }) }) } } } . fetch_one (& __self . db) . await . expect ("Didn't fint any bowls") ;
                    Ok(b)
                }
            };
            #[allow(unreachable_code)]
            __ret // return value that is either a Self::Data -> Bowls or CapServiceError
        })
    }
}
```


HTTP calling code

```rust
#[allow(non_camel_case_types, missing_docs)]
pub struct create_new_bowl;
impl actix_web::dev::HttpServiceFactory for create_new_bowl {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn create_new_bowl(
            json: web::Json<BowlsDTO>,
            svc: web::Data<CapService>,
        ) -> impl Responder {
            let svc = svc.get_ref();
            let newbowl: Bowls = Bowls {
                id: 0,
                name: json.name.to_owned(),
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &match (&newbowl,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Debug::fmt,
                        )],
                    },
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 4u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Implied,
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            let res_bowl = create_db_bowl(svc, newbowl).await;
            match res_bowl {
                Ok(bowl) => HttpResponse::Ok().json(bowl),
                _ => HttpResponse::BadRequest().json("{ \"request\": \"bad request\" "),
            }
        }
        let __resource = actix_web::Resource::new("/bowls/")
            .name("create_new_bowl")
            .guard(actix_web::guard::Post())
            .to(create_new_bowl);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
```