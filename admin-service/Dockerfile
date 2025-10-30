# Multi-stage build for admin-service
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY admin-service ./admin-service

# Build dependencies first (cache layer)
RUN mkdir -p admin-service/src && echo "fn main() {}" > admin-service/src/main.rs
RUN cargo build --release --package admin-service
RUN rm -rf admin-service/src

# Build actual application
COPY admin-service/src ./admin-service/src
RUN cargo build --release --package admin-service

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc ca-certificates

# Create app user
RUN addgroup -g 1000 app && adduser -D -u 1000 -G app app

# Copy binary
COPY --from=builder /build/target/release/admin-service /usr/local/bin/admin-service

# Copy static files
COPY admin-service/static /static

# Create directories
RUN mkdir -p /data /certs && chown app:app /data /certs

USER app

# Expose port
EXPOSE 8001

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8001/health || exit 1

# Default command
CMD ["admin-service"]