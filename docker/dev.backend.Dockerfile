FROM rust:1.91.0 AS builder
WORKDIR /server
COPY server/Cargo.lock server/Cargo.toml ./
COPY lib /lib
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build && rm -rf src
COPY server/src ./src
COPY server/migrations ./migrations
RUN touch src/main.rs
RUN cargo build
EXPOSE 3000
CMD ["/server/target/debug/server"]
