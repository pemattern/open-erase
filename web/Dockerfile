FROM rust:1.86.0 AS chef
RUN cargo install cargo-chef
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
WORKDIR /web

FROM chef AS planner
ENV SQLX_OFFLINE=true
COPY ./web .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /web/recipe.json /web/recipe.json
RUN cargo chef cook --release --recipe-path /web/recipe.json
COPY . .
RUN trunk build --release

FROM debian:bookworm-slim AS runtime
COPY --from=builder /web/dist /open-erase-web/
# TODO RUN WITH NGINX or another server
ENTRYPOINT [ "/open-erase-web" ]
EXPOSE 8080
