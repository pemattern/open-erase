FROM rust:1.86.0 AS builder
WORKDIR /build
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
COPY . .
RUN cargo build --release

FROM archlinux:base-20250511.0.348143 AS runtime
WORKDIR /app
COPY --from=builder /build/target/release/open-erase ./open-erase
COPY --from=builder /build/web/dist ./dist
ENTRYPOINT [ "/open-erase" ]
EXPOSE 3000
