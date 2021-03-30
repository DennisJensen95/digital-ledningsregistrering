# Build Stage
FROM rust:latest AS builder
WORKDIR /usr/src/

RUN USER=root cargo new ler_2_database
WORKDIR /usr/src/ler_2_database
COPY Cargo.toml Cargo.lock ./
COPY ./diesel.toml ./
COPY ./migrations ./migrations
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
COPY ./src ./src
RUN cargo build --release --bin ler_2_database

# Bundle Stage
FROM rust:latest

COPY --from=builder /usr/src/ler_2_database/target/release/ler_2_database /usr/local/bin
USER 1000
ENTRYPOINT ["/usr/local/bin/ler_2_database"]