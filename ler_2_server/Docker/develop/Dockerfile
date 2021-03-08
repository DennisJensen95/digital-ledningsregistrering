FROM ubuntu:20.04

RUN apt-get update && apt-get install -y curl
RUN apt-get install -y wget
RUN apt-get install build-essential -y

# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs  -tlsv1 -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup default nightly

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8080