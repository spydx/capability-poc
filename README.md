# Capability PoC - Instructions

PS: remember to clone with `--recurse-submodules`

```sh
git clone --recurse-submodules https://github.com/spydx/capability-poc.git
cd capability-poc
git submodule update --remote --recursive
```

## Running the example

The recommended way to start this example is to use `docker-compose`.
The build process will depend on your internet connection take anything from 15 min to 20+ min to build all the components.

### Docker-compose

After installing Docker:

Bring everything up

```sh
> docker-compise build # approx 15+ min
> docker-compose up 
```

Launch a browser and access http://localhost:3000/

The process to be done is:

- 1. GnapRequest
- 2. Login at the AS
- 3. Continuation Request (Authorization)

- One or more of the desired actions.

## Users

| Username | Password |
|---|---|
| kenneth | password |
| alice | password |
| bob | password |

### Manually

You will need:

- Rust/Cargo : [install](https://www.rust-lang.org/tools/install)
- SQLx-cli : `cargo install sqlx-cli` [install](https://lib.rs/crates/sqlx-cli)
- sqlx needs SQLite support, so either specify or leave for default.
- npm for nextjs.
- Add `host.docker.internal` pointing to `127.0.0.1` to in the local hosts file.

### Ubuntu / Linux

You will need:

```sh
sudo apt install build-essential
sudo apt install libssl-dev
sudo apt install pkg-config
rustup toolchain install nightly # do this if cargo doesn't do it automatically
cargo check # after all above, this should work from the project root.
```

### Building order

1. GNAP

```sh
> cd gnap
> docker-compose up -d # bring up REDIS and MongoDB.
> cd gnap_as
> cargo run 
```

2. simple-api
```sh
> cd simple-api
> cargo run
```

3. cap-client

```sh
> cd cap-client
> npm install
> npm run dev
> open http://localhost:3000/
```

## Database errors

Sometimes the Mongo database stops working correctly.
If it does, then removing the data in the folder `./gnap/data/mongodb`
will cause a rebuild of the database to the clean working state with `docker-compose up`.