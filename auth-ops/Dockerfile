# Multi-stage build for auth-ops CLI tool
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY auth-ops ./auth-ops

# Build dependencies first (cache layer)
RUN mkdir -p auth-ops/src && echo "fn main() {}" > auth-ops/src/main.rs
RUN cargo build --release --package auth-ops
RUN rm -rf auth-ops/src

# Build actual application
COPY auth-ops/src ./auth-ops/src
RUN cargo build --release --package auth-ops

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc ca-certificates

# Create app user
RUN addgroup -g 1000 app && adduser -D -u 1000 -G app app

# Copy binary
COPY --from=builder /build/target/release/auth-ops /usr/local/bin/auth-ops

# Create directories
RUN mkdir -p /data /backup && chown app:app /data /backup

USER app

# Default command
ENTRYPOINT ["auth-ops"]