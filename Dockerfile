FROM rust:1.48.0-buster as build

COPY ./src /app/src
COPY ./.env /app
COPY ./Cargo.toml /app
COPY ./rustfmt.toml /app

WORKDIR /app

RUN cargo build

EXPOSE 8000

ENTRYPOINT ["cargo", "run"]
