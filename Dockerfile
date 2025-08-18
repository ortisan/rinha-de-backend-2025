# Build stage
FROM rust:1.84-slim as builder
LABEL authors="Marcelo Ortiz de Santana"

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime certificates (no libpq or OpenSSL)
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/payments_api /app/payments_api

# Expose the port the app will run on
EXPOSE 8000

# Command to run the application
CMD ["/app/payments_api"]