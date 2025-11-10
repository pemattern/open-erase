FROM rust:1.91.0
WORKDIR /server
COPY server/Cargo.lock server/Cargo.toml ./
COPY lib /lib
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY server/src ./src
COPY server/migrations ./migrations
RUN touch src/main.rs
EXPOSE 3000
CMD ["bash", "-c", "cargo run"]

