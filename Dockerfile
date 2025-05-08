FROM rust:1.86.0 AS base

FROM node:23.11-bookworm AS css
WORKDIR /css
RUN npm install tailwindcss @tailwindcss/cli
COPY web ./
RUN npx @tailwindcss/cli -i ./input.css -o ./output.css

FROM base AS web
WORKDIR /web
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
COPY web ./
RUN trunk build --release
COPY --from=css /css/output.css ./dist/output.css

FROM base AS server
WORKDIR /server
COPY server ./
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
COPY --from=server /server/target/release/open-erase-server /open-erase-server
COPY --from=web /web/dist /dist
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
