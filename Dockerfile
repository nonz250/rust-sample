FROM rust:1.53.0-alpine3.13

RUN ls -al

COPY ./app /app
