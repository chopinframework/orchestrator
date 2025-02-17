# Build stage
FROM rust:1.76-slim-bullseye as builder

WORKDIR /usr/src/app

# Install OpenSSL development packages and pkg-config
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

# Build the application with release optimizations
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Install OpenSSL runtime library
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder
COPY --from=builder /usr/src/app/target/release/sequencer .

# Expose the port the app runs on
EXPOSE 4001

# Run the binary
CMD ["./sequencer"] 