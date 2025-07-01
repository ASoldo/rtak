# --------- Stage 1: Build RTAK ---------
FROM rust:1.79 as builder

WORKDIR /usr/src/rtak

# Pre-cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo fetch

# Now build the actual app
COPY . .
RUN cargo build --release

# --------- Stage 2: Runtime image ---------
FROM debian:bookworm-slim

# Install minimal runtime dependencies (ca-certificates for HTTPS if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder stage
COPY --from=builder /usr/src/rtak/target/release/rtak /usr/local/bin/rtak

# Optional: copy your config if you want defaults inside the container
# COPY rtak.toml /etc/rtak.toml

# Expose REST/WebSocket port and UDP CoT port
EXPOSE 8080/tcp
EXPOSE 6969/udp

# Run the binary by default
ENTRYPOINT ["rtak"]
