# Build stage
FROM rust:1.85-slim AS builder

# Install PostgreSQL client libraries and other required packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /app
RUN USER=root cargo new --bin rustic-notification
WORKDIR /app/rustic-notification

# Copy manifests
COPY Cargo.toml ./

# Build dependencies - this is the caching Docker layer
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY src ./src

# Build application
RUN rm ./target/release/deps/rustic_notification*
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m appuser
USER appuser
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/rustic-notification/target/release/rustic-notification .

# Set the entry point
CMD ["./rustic-notification"]
