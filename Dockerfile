FROM rust:1.53.0-alpine3.13

RUN apk update
RUN apk add alpine-sdk
RUN cargo install cargo-watch

COPY ./app /app
