FROM rust:1.91.0 AS builder
WORKDIR /app/server
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall bacon --no-confirm
EXPOSE 3000
CMD ["bash"]
