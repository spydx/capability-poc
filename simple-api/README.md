# simple-api

## Update Queries

IMPORTANT: this lib must not be a part of the workspace, or sqlx-cli will fail.

```zsh
echo "DATABASE_URL=sqlite:test.db" >> .env
echo "SQLX_OFFLINE=true" >> .env
cargo sqlx db create
cargo sqlx mig run
cargo sqlx prepare --merged -- --all-targets --all-features
```
