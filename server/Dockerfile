FROM rust:1.86.0 AS builder
WORKDIR server
ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
COPY --from=builder /server/target/release/open-erase-server /open-erase-server
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
