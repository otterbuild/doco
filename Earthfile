VERSION 0.8

IMPORT github.com/earthly/lib/rust AS rust

FROM rust:1.84.1-slim
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

base-node-container:
    FROM node:alpine
    WORKDIR /doco

    # Install prettier
    RUN npm install -g prettier markdownlint-cli

    # Copy the source code into the container
    DO +COPY_SOURCES

base-rust-container:
    # Install clippy and rustfmt
    RUN rustup component add clippy rustfmt

    # Install system-level dependencies
    RUN apt update && apt upgrade -y && apt install -y curl libssl-dev pkg-config

    # Initialize Rust
    DO rust+INIT --keep_fingerprints=true

base-rust-tarpaulin-container:
    FROM +base-rust-container

    # Install system-level dependencies
    RUN apt update && apt upgrade -y && apt install -y curl libssl-dev pkg-config

    # Install cargo-tarpaulin
    DO rust+CARGO --args="install cargo-tarpaulin"

    # Cache the container
    SAVE IMAGE --cache-hint

base-example-axum-container:
    FROM +base-rust-sources

    # Copy the example
    COPY --keep-ts --dir examples/axum-postgres examples/axum-postgres

    # Change the working directory
    WORKDIR examples/axum-postgres

example-axum-docker:
    FROM DOCKERFILE -f examples/axum-postgres/Dockerfile .

example-axum-format:
    FROM +base-example-axum-container

    # Check the code formatting
    DO rust+CARGO --args="fmt --all --check"

example-axum-lint:
    FROM +base-example-axum-container

    # Check the code for linting errors
    DO rust+CARGO --args="clippy --all-targets --all-features -- -D warnings"

example-axum-test:
    FROM +base-example-axum-container

    # Run the tests
    WITH DOCKER --load doco:axum-postgres=+example-axum-docker
        RUN cargo test --all-features --all-targets --locked
    END

base-example-leptos-container:
    FROM +base-rust-sources

    # Copy the example
    COPY --keep-ts --dir examples/leptos examples/leptos

    # Change the working directory
    WORKDIR examples/leptos

example-leptos-docker:
    FROM DOCKERFILE -f examples/leptos/Dockerfile .

example-leptos-format:
    FROM +base-example-leptos-container

    # Check the code formatting
    DO rust+CARGO --args="fmt --all --check"

example-leptos-lint:
    FROM +base-example-leptos-container

    # Check the code for linting errors
    DO rust+CARGO --args="clippy --all-targets --all-features -- -D warnings"

example-leptos-test:
    FROM +base-example-leptos-container

    # Run the tests
    WITH DOCKER --load doco:leptos=+example-leptos-docker
        RUN cargo test --all-features --all-targets --locked
    END

json-format:
    FROM +base-node-container

    # Check the JSON formatting
    RUN prettier --check **/*.{json,json5}

markdown-format:
    FROM +base-node-container

    # Check the formatting of Markdown files
    RUN prettier --check **/*.md

markdown-lint:
    FROM +base-node-container

    # Check the Markdown files for linting errors
    RUN markdownlint **/*.md

base-rust-sources:
    FROM +base-rust-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

base-rust-build:
    FROM +base-rust-sources

    # Build the project
    DO rust+CARGO --args="build --all-features --locked"

rust-deps-latest:
    FROM +base-rust-sources

    # Switch to beta toolchain
    RUN rustup default beta

    # Update the dependencies to the latest versions
    DO rust+CARGO --args="update"

    # Compile code to ensure the latest versions are compatible
    RUN RUSTFLAGS="-D deprecated" cargo check --all-features --all-targets --locked

rust-deps-minimal:
    FROM +base-rust-sources

    # Switch to nightly toolchain
    RUN rustup default nightly

    # Set minimal versions for dependencies
    DO rust+CARGO --args="update -Z direct-minimal-versions"

    # Compile code to ensure the minimal versions are compatible
    DO rust+CARGO --args="check --all-features --all-targets --locked"

rust-doc:
    FROM +base-rust-sources

    # Generate the documentation
    RUN cargo doc --all-features --no-deps

    # Save the documentation to the local filesystem
    SAVE ARTIFACT target/doc AS LOCAL target/doc

rust-features:
    FROM +base-rust-build

    # Install cargo-hack
    DO rust+CARGO --args="install cargo-hack"

    # Test combinations of features
    DO rust+CARGO --args="hack --feature-powerset check --lib --tests"

rust-format:
    FROM +base-rust-sources

    # Check the code formatting
    DO rust+CARGO --args="fmt --all --check"

rust-lint:
    FROM +base-rust-build

    # Check the code for linting errors
    DO rust+CARGO --args="clippy --all-targets --all-features -- -D warnings"

rust-test:
    # Optionally save the report to the local filesystem
    ARG SAVE_REPORT=""

    FROM +base-rust-tarpaulin-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

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
    FROM +base-node-container

    # Check the YAML formatting
    RUN prettier --check **/*.{yml,yaml}

yaml-lint:
    FROM pipelinecomponents/yamllint:latest
    WORKDIR /doco

    # Copy the source code into the container
    DO +COPY_SOURCES

    # Check the YAML files for linting errors
    RUN yamllint .
