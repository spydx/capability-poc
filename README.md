# Capability PoC

PS: remember to clone with `--recurse-submodules`

```sh
git clone --recurse-submodules <repo>
git submodule update --init --recursive # first time
git submodule update --recursive --remote # other option
git pull --recurse-submodules
```

## Important setting

When using `cargo` and you are trying to fetch the lib repo from GitHub,
you might get problems with your credentials.
Workaround for this is to set this environmental variable:

```sh
> export CARGO_NET_GIT_FETCH_WITH_CLI=true
```

This will make you able to run the following commands without problems:

```sh
> cargo update
> cargo check / clippy / build # only the first time
```

## To run any of these samples

### Manually

You need:

- Rust/Cargo : [install](https://www.rust-lang.org/tools/install)
- SQLx-cli : `cargo install sqlx-cli` [install](https://lib.rs/crates/sqlx-cli)
- sqlx needs SQLite support, so either specify or leave for default.

### Ubuntu

You will need:

```sh
sudo apt install build-essential
sudo apt install libssl-dev
sudo apt install pkg-config
rustup toolchain install nightly # do this if cargo doesn't do it automatically
cargo check # after all above, this should work from the project root.
```

### Docker

After installing Docker, from root:

```sh
> docker build . --file dockerfiles/single-api -t single-api
> docker run -p 8080:8080 --name single-api single-api
```

### Docker-compose

After installing Docker:

Bring everything up

```sh
> docker-compose up
```

Just simple-api

```sh
> docker-compose up simple-api
```


