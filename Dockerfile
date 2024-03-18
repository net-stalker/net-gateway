# Start from Ubuntu 22.04 base image
FROM ubuntu:22.04

# Install required packages
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set environment variables
ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTUP_HOME="/root/.rustup"

# Create a new directory for the application
WORKDIR /net-gateway

# Copy the Rust source code
COPY . .

# Build the application
RUN cargo build --release

# Set the binary as the entrypoint
ENTRYPOINT ["/app/target/release/net-gateway"]