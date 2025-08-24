FROM rust:1.88-alpine AS builder

# Install OpenSSL dev dependencies and pkg-config
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /app

# Copy the entire workspace
COPY Cargo.toml Cargo.lock ./
COPY rinha_api/ ./rinha_api/
COPY rinha_worker/ ./rinha_worker/

# Build the worker project with release optimizations
RUN cargo build --release --bin rinha_worker

FROM alpine:3.21

# Install runtime SSL libraries
RUN apk add --no-cache \
    libgcc \
    openssl

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/rinha_worker .

CMD ["./rinha_worker"]
