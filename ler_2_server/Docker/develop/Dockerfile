FROM ubuntu:20.04

RUN apt-get update && apt-get install -y curl
RUN apt-get install build-essential -y

RUN mkdir -p /user/turreta-rust-builder/src
WORKDIR /user/turreta-rust-builder/src

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup default nightly

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8080