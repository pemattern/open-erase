FROM rust:1.91.0 AS base

FROM base AS server-builder
WORKDIR /server
COPY server/Cargo.lock server/Cargo.toml ./
COPY lib /lib
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY server/src ./src
COPY server/migrations ./migrations
RUN touch src/main.rs && cargo build --release

FROM base AS web-builder
WORKDIR /web
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
COPY web ./
COPY lib /lib
RUN trunk build --release

FROM debian:trixie-slim AS runtime
COPY --from=server-builder /server/target/release/server /server
COPY --from=web-builder /web/dist /dist
ENTRYPOINT [ "/server" ]
EXPOSE 3000
