# Image for the `migrate` Job: diesel_cli plus the migrations folder.
# Kept separate from the main Dockerfile so production builds don't compile diesel_cli.
# Build from the repo root:  docker build -f k8s/migrate.Dockerfile -t mtmulch-migrate:local .
FROM rust:1.95-bookworm AS builder
RUN cargo install diesel_cli --no-default-features --features postgres --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends libpq5 ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
WORKDIR /app
COPY diesel.toml ./
COPY migrations ./migrations
# diesel.toml regenerates src/schema.rs after migrating; give it somewhere to write.
COPY src/schema.rs ./src/schema.rs
CMD ["diesel", "migration", "run"]
