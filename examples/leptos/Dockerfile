# Build stage
FROM rust:1.80.1-slim as builder

# Install system-level dependencies
RUN apt-get update && apt-get install -y \
    wget \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Install cargo-binstall
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && mv cargo-binstall /usr/local/cargo/bin \
    && rm cargo-binstall-x86_64-unknown-linux-musl.tgz

# Install cargo-run-bin
RUN cargo binstall cargo-leptos -y

WORKDIR /app

# Create an empty Rust crate
RUN cargo new --lib --vcs none crates/doco \
    && cargo new --lib --vcs none --name leptos-ssr examples/leptos

# Copy the Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Cache the workspace
RUN cargo build --release

# Copy the example's Cargo.toml and Cargo.lock
COPY examples/leptos/Cargo.toml examples/leptos/Cargo.lock examples/leptos/

# Cache the example
RUN cargo build --release

# Copy the source code and build the project
COPY . .
RUN cd examples/leptos && cargo leptos build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Copy server binary
COPY --from=builder /app/examples/leptos/target/release/leptos-ssr server

# Copy WASM module, CSS stylesheets, and other static files
COPY --from=builder /app/examples/leptos/target/leptos site

# Configure the Leptos app
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Start the server
CMD ["/app/server"]
