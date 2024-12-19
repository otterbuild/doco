VERSION 0.8

FROM rust:1.83.0-slim
WORKDIR /doco

all:
    BUILD +json-format
    BUILD +markdown-format
    BUILD +markdown-lint
    BUILD +rust-deps-latest
    BUILD +rust-deps-minimal
    BUILD +rust-doc
    BUILD +rust-features
    BUILD +rust-format
    BUILD +rust-lint
    BUILD +rust-test
    BUILD +yaml-format
    BUILD +yaml-lint

COPY_SOURCES:
    FUNCTION

    # Copy the source code into the container
    COPY . .

COPY_RUST_SOURCES:
    FUNCTION

    # Copy the source code in a cache-friendly way
    COPY --keep-ts Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir crates ./

node-container:
    FROM node:alpine
    WORKDIR /doco

    # Install prettier
    RUN npm install -g prettier markdownlint-cli

    # Copy the source code into the container
    DO +COPY_SOURCES

rust-container:
    # Install clippy and rustfmt
    RUN rustup component add clippy rustfmt

    # Install system-level dependencies
    RUN apt update && apt upgrade -y && apt install -y curl libssl-dev pkg-config

json-format:
    FROM +node-container

    # Check the JSON formatting
    RUN prettier --check **/*.{json,json5}

markdown-format:
    FROM +node-container

    # Check the formatting of Markdown files
    RUN prettier --check **/*.md

markdown-lint:
    FROM +node-container

    # Check the Markdown files for linting errors
    RUN markdownlint **/*.md

rust-sources:
    FROM +rust-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

rust-build:
    FROM +rust-sources

    # Build the project
    RUN cargo build --all-features --locked

rust-deps-latest:
    FROM +rust-sources

    # Switch to beta toolchain
    RUN rustup default beta

    # Update the dependencies to the latest versions
    RUN cargo update

    # Compile code to ensure the latest versions are compatible
    RUN RUSTFLAGS="-D deprecated" cargo check --all-features --all-targets --locked

rust-deps-minimal:
    FROM +rust-sources

    # Switch to nightly toolchain
    RUN rustup default nightly

    # Set minimal versions for dependencies
    RUN cargo update -Z direct-minimal-versions

    # Compile code to ensure the minimal versions are compatible
    RUN cargo check --all-features --all-targets --locked

rust-doc:
    FROM +rust-sources

    # Generate the documentation
    RUN cargo doc --all-features --no-deps

    # Save the documentation to the local filesystem
    SAVE ARTIFACT target/doc AS LOCAL target/doc

rust-features:
    FROM +rust-build

    # Install cargo-hack
    RUN cargo install cargo-hack

    # Test combinations of features
    RUN cargo hack --feature-powerset check --lib --tests

rust-format:
    FROM +rust-sources

    # Check the code formatting
    RUN cargo fmt --all --check

rust-lint:
    FROM +rust-build

    # Check the code for linting errors
    RUN cargo clippy --all-targets --all-features -- -D warnings

rust-test:
    # Optionally save the report to the local filesystem
    ARG SAVE_REPORT=""

    FROM +rust-build

    # Install cargo-binstall
    RUN cargo install cargo-binstall

    # Install cargo-tarpaulin
    RUN cargo binstall cargo-tarpaulin

    # Run the tests and measure the code coverage
    WITH DOCKER --pull selenium/standalone-firefox:latest
        RUN cargo tarpaulin \
            --all-features \
            --engine llvm \
            --exclude doco-derive \
            --out Xml \
            --skip-clean \
            --timeout 120 \
            --verbose \
            --workspace
    END

    # Save the coverage report
    IF [ "$SAVE_REPORT" != "" ]
        SAVE ARTIFACT cobertura.xml AS LOCAL cobertura.xml
    END

yaml-format:
    FROM +node-container

    # Check the YAML formatting
    RUN prettier --check **/*.{yml,yaml}

yaml-lint:
    FROM pipelinecomponents/yamllint:latest
    WORKDIR /doco

    # Copy the source code into the container
    DO +COPY_SOURCES

    # Check the YAML files for linting errors
    RUN yamllint .
