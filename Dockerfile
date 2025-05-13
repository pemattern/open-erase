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

FROM base AS client-builder
WORKDIR /client
COPY client/x86_64 ./
RUN cargo build --release

FROM nixos/nix AS iso-builder
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf
WORKDIR /client
COPY ./client/x86_64 .
COPY --from=client-builder /client/target/release/x86_64 /client/x86_64
RUN ls -a
RUN nix build .#nixosConfigurations.iso.config.system.build.isoImage

FROM debian:bookworm-slim AS runtime
COPY web/assets /dist/assets
COPY --from=css-builder /css/styles/output.css /dist/styles/output.css
COPY --from=server-builder /server/target/release/open-erase-server /open-erase-server
COPY --from=web-builder /web/dist /dist
COPY --from=iso-builder /client/result/iso /dist/artifacts
ENTRYPOINT [ "/open-erase-server" ]
EXPOSE 3000
