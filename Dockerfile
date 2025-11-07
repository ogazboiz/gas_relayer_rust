# syntax=docker/dockerfile:1.7

FROM rustlang/rust:nightly-bullseye AS builder

# Install build dependencies for sqlx/postgres
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev libpq-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Pre-copy manifests to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./
COPY bins/relayer/Cargo.toml bins/relayer/
COPY crates/config/Cargo.toml crates/config/
COPY crates/db/Cargo.toml crates/db/
COPY crates/routes/Cargo.toml crates/routes/

# Minimal sources so cargo recognizes targets during dependency fetch
COPY bins/relayer/src bins/relayer/src
COPY crates/config/src crates/config/src
COPY crates/db/src crates/db/src
COPY crates/routes/src crates/routes/src

# Pre-fetch dependencies
RUN cargo fetch

# Copy the full workspace
COPY . .

RUN cargo build --release --bin relayer

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libpq5 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --uid 10001 --create-home appuser

WORKDIR /app

COPY --from=builder /app/target/release/relayer /usr/local/bin/relayer

ENV APP_ENVIRONMENT=Local \
    APP_PORT=8080 \
    MAX_DB_CONNECTION=5 \
    DATABASE_URL=postgres://postgres:postgres@db:5432/relayer \
    RUST_LOG=info

EXPOSE 8080

USER appuser

CMD ["relayer"]

