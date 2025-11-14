FROM rust:1.91.0 AS builder
WORKDIR /app/server
EXPOSE 3000
CMD ["bash"]
