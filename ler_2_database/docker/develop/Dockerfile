FROM rustlang/rust:nightly as planner-database
WORKDIR app
RUN cargo --version
RUN rustc --version
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rustlang/rust:nightly as cacher-database
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner-database /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:nightly as builder-database
WORKDIR app
RUN ls
COPY . .
# Copy over the cached dependencies
COPY --from=cacher-database /app/target target
COPY --from=cacher-database /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin ler_2_database

FROM rustlang/rust:nightly as runtime-database
WORKDIR app
COPY --from=builder-database /app/target/release/ler_2_database /usr/local/bin

RUN rustup component add rustfmt