FROM rust:1.85-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

COPY src ./src

#Build the app in release mode - why release?
RUN cargo build --release

# Runtime state - what is this?
FROM debian:boookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder
COPY --from=builder /app/target/release/HeckerProject /app/backend

# Expose defaut port
EXPOSE 8000

CMD ["./backend"]