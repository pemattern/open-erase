FROM rust:1.86.0 AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
WORKDIR web
COPY web .
RUN trunk build
WORKDIR ../server
COPY server .
RUN cargo build

FROM nixos/nix AS runtime
COPY --from=builder /server/target/release/open-erase-server /open-erase-server
COPY --from=builder /web/dist /dist
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
