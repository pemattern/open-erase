FROM rust:1.86.0 AS chef
RUN cargo install cargo-chef 
WORKDIR /server

FROM chef AS planner
ENV SQLX_OFFLINE=true
COPY ./server .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /server/recipe.json /server/recipe.json
RUN cargo chef cook --release --recipe-path /server/recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
COPY --from=builder /server/target/release/open-erase-server /open-erase-server
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
