FROM rust:1.86.0 AS builder
ENV SQLX_OFFLINE=true
WORKDIR /open-erase
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /open-erase/target/release/open-erase-server /open-erase-server
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
