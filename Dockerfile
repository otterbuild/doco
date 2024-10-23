# Build stage
FROM rust:1.82.0 AS builder

# Install system-level dependencies
RUN apt-get update && apt-get install -y \
    nodejs \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Install tools
RUN cargo install cargo-binstall
RUN cargo binstall cargo-run-bin -y
RUN cargo binstall cargo-leptos -y
