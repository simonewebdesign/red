# Red 🔴

> Fast, reliable key-value storage. Inspired by Redis, built with Rust.

Red has a similar interface to Redis. It has two executables:

- `red` - an interactive shell (like `redis-cli`)
- `red-server` - a TCP server (like `redis-server`)

## Installation

TODO: add crate

### Compile from source

Clone the repo, then compile the project with:

    rustc src/main.rs -o red  # Compile the CLI
    rustc src/server.rs -o red-server  # Compile the TCP server

Alternatively, you can start an interactive shell with:

    cargo run
