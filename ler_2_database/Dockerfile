# Build Stage
FROM rust:latest AS builder
WORKDIR /usr/src/

# RUN apt-get update
# RUN apt-get install musl-tools -y
# RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new ler_2_database
WORKDIR /usr/src/ler_2_database
COPY Cargo.toml Cargo.lock ./
COPY ./diesel.toml ./
COPY ./migrations ./migrations
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
COPY ./src ./src
RUN cargo install --path .  

# Bundle Stage
FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/ler_2_database /usr/local/bin
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata postgresql postgresql-contrib \
    && rm -rf /var/lib/apt/lists/*

USER 1000
ENTRYPOINT [ "/usr/local/bin/ler_2_database" ]