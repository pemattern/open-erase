FROM rust:1.86.0 AS chef
RUN cargo install cargo-chef 
WORKDIR /web

FROM chef AS planner
ENV SQLX_OFFLINE=true
COPY ./web .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /web/recipe.json /web/recipe.json
RUN cargo chef cook --release --recipe-path /web/recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
COPY --from=builder /web/target/release/open-erase-web /open-erase-web
ENTRYPOINT [ "/open-erase-web" ]
EXPOSE 3001
