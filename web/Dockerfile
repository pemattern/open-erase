FROM rust:1.86.0 AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
WORKDIR /web
COPY ./web .
RUN trunk build --release

FROM nginx:1.28.0 AS runtime
WORKDIR /usr/share/nginx/html
COPY ./web/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /web/dist .
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
