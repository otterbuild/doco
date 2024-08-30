# Leptos

[Leptos](https://leptos.dev/) is a full-stack web development framework that
uses Rust and WebAssembly. This example is an offical example taken straight
from the [`leptos-rs/leptos`][leptos-repo] repository on GitHub, and only
modified slightly to with `doco`.

## Installing cargo-leptos

The example can be built and run with the [cargo-leptos] command-line tool. This
example has been configured to install a specific version using [cargo-run-bin].

```shell
# Install the required tools
cargo install cargo-run-bin cargo-binstall

# Install cargo-leptos
cargo bin --install
cargo bin --sync-aliases
```

## Running your project

To run your project, you can use the following command:

```shell
cargo leptos watch
```

This will compile the app, start a web server, and watch for changes to the
source files to automatically recompile and reload the server.

[cargo-leptos]: https://github.com/leptos-rs/cargo-leptos
[leptos-repo]: https://github.com/leptos-rs/leptos
