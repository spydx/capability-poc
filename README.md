# Capability PoC

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

- Cargo : [install](https://www.rust-lang.org/tools/install)
- SQLx-cli : `cargo install sqlx-cli` [install](https://lib.rs/crates/sqlx-cli)
- sqlx needs SQLite support, so either specify or leave for default.

### Docker (TODO)

After installing Docker:

```sh
> cd single-api
> docker build . -t single-api
> docker run -p 8080:8080 --name single-api single-api
```

### Docker-compose (TODO)

After installing Docker:

```sh
> docker-compose up
```
