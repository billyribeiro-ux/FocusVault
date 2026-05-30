# ── Stage 1: Build Frontend ──
FROM node:24-alpine AS frontend-builder
RUN corepack enable && corepack prepare pnpm@11 --activate
WORKDIR /app

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY apps/web/package.json apps/web/
RUN pnpm install --frozen-lockfile

COPY apps/web apps/web
RUN pnpm --filter web build

# ── Stage 2: Build Rust Backend ──
FROM rust:1.96-bookworm AS backend-builder
WORKDIR /app

# Cache dependencies: copy manifests first, build a dummy to populate cache
COPY Cargo.toml Cargo.lock ./
COPY crates/focusvault-core/Cargo.toml crates/focusvault-core/
COPY crates/focusvault-db-postgres/Cargo.toml crates/focusvault-db-postgres/
COPY crates/focusvault-db-sqlite/Cargo.toml crates/focusvault-db-sqlite/
COPY crates/focusvault-server/Cargo.toml crates/focusvault-server/

# Create dummy lib/main files for dependency caching
RUN mkdir -p crates/focusvault-core/src && echo "pub fn stub(){}" > crates/focusvault-core/src/lib.rs && \
    mkdir -p crates/focusvault-db-postgres/src && echo "pub fn stub(){}" > crates/focusvault-db-postgres/src/lib.rs && \
    mkdir -p crates/focusvault-db-sqlite/src && echo "pub fn stub(){}" > crates/focusvault-db-sqlite/src/lib.rs && \
    mkdir -p crates/focusvault-server/src && echo "fn main(){}" > crates/focusvault-server/src/main.rs && \
    echo "pub fn stub(){}" > crates/focusvault-server/src/lib.rs

RUN cargo build --release --bin focusvault-server 2>/dev/null || true

# Copy real sources and rebuild
COPY crates crates
RUN touch crates/focusvault-core/src/lib.rs \
          crates/focusvault-db-postgres/src/lib.rs \
          crates/focusvault-db-sqlite/src/lib.rs \
          crates/focusvault-server/src/main.rs \
          crates/focusvault-server/src/lib.rs && \
    cargo build --release --bin focusvault-server

# ── Stage 3: Runtime ──
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 1001 focusvault && \
    useradd --uid 1001 --gid focusvault --create-home focusvault

WORKDIR /app

COPY --from=backend-builder /app/target/release/focusvault-server /app/focusvault-server
COPY --from=frontend-builder /app/apps/web/build /app/static

# Migrations needed at runtime
COPY crates/focusvault-db-postgres/migrations /app/migrations/postgres
COPY crates/focusvault-db-sqlite/migrations /app/migrations/sqlite

USER focusvault

ENV HOST=0.0.0.0
ENV PORT=3000
ENV RUST_LOG=info,focusvault=debug

EXPOSE 3000

HEALTHCHECK --interval=15s --timeout=3s --retries=3 \
    CMD ["/app/focusvault-server", "--health-check"] || exit 1

ENTRYPOINT ["/app/focusvault-server"]
