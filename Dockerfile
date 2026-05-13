FROM rust:1.95 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM rust:1.95 AS debug
RUN apt-get update && apt-get install -y gdbserver && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# Source is mounted at runtime via docker-compose volume; build happens in entrypoint
COPY docker-debug-entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
EXPOSE 8080 1234
ENTRYPOINT ["/entrypoint.sh"]


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-playground /usr/local/bin/
EXPOSE 8080
CMD ["rust-playground"]