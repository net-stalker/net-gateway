FROM messense/rust-musl-cross:x86_64-musl as builder

# Install required packages
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    musl-tools \
    libzmq3-dev \
    nano \
    htop \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl
RUN export PKG_CONFIG_SYSROOT_DIR=/usr/include
RUN export OPENSSL_DIR=/usr/include/x86_64-linux-gnu/openssl
RUN ln -s /bin/g++ /bin/musl-g++

# Create a new directory for the application
WORKDIR /net-gateway

# Copy the Rust source code
COPY . .

# Build the application
RUN cargo build --package net-gateway --release --target=x86_64-unknown-linux-musl

# Final stage
FROM debian:latest
COPY --from=builder /net-gateway/target/x86_64-unknown-linux-musl/release/net-gateway /
COPY --from=builder /net-gateway/net-gateway/config.toml /
ENV CONFIG_PATH=/
ENTRYPOINT ["/net-gateway"]
