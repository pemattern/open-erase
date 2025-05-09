FROM rust:1.86.0 AS base

FROM node:23.11-bookworm AS css-builder
WORKDIR /css
RUN npm install tailwindcss @tailwindcss/cli
COPY web ./
RUN npx @tailwindcss/cli -i ./styles/input.css -o ./styles/output.css

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
COPY web/assets /dist/assets
COPY --from=css-builder /css/styles/output.css /dist/styles/output.css
COPY --from=server-builder /server/target/release/open-erase-server /open-erase-server
COPY --from=web-builder /web/dist /dist
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
