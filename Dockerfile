FROM rust:1.53.0

RUN apt-get update -y && apt-get upgrade -y
RUN cargo install diesel_cli --no-default-features --features mysql
RUN cargo install cargo-watch

COPY ./app/migrations /app/migrations
COPY ./app/src /app/src
COPY ./app/Cargo.toml /app/Cargo.toml
COPY ./app/Cargo.lock /app/Cargo.lock
COPY ./app/diesel.toml /app/diesel.toml

WORKDIR /app

RUN cargo build --release
