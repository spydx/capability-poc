FROM rust:1.58 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install sqlx-cli
RUN sqlx database create
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install ca-certificates openssl -y && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
WORKDIR /usr/local/bin/
COPY --from=builder /usr/src/myapp/test.db ./test.db
COPY --from=builder /usr/local/cargo/bin/single-api ./single-api
EXPOSE 8080
CMD ["single-api"]%        