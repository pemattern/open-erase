FROM rust:1.86.0 AS base

FROM base AS web
WORKDIR /web
COPY web ./
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
RUN trunk build --release

FROM base AS server
WORKDIR /server
COPY server ./
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
COPY --from=server /server/target/release/open-erase-server /open-erase-server
COPY --from=web /web/dist /dist
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
