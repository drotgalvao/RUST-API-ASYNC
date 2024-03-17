FROM rust:1.75

RUN apt-get update && apt-get install -y musl-dev

RUN cargo install cargo-watch

WORKDIR /app

COPY . .

EXPOSE 8000
