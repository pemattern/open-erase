FROM rust:1.88.0 AS base

FROM base AS web-builder
WORKDIR /web
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
COPY web ./
RUN trunk build --release

FROM base AS server-builder
WORKDIR /server
COPY server ./
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
COPY --from=server-builder /server/target/release/open-erase-server /open-erase-server
COPY --from=web-builder /web/dist /dist
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
