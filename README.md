# Red 🔴

> Fast, reliable key-value storage. Inspired by Redis, built with Rust.

Red has a similar interface to Redis. It has two executables:

- `red` - an interactive shell (like `redis-cli`)
- `red-server` - a TCP server (like `redis-server`)

## Commands

* [`GET`](#get-key)
* [`SADD`](#sadd-key-member-member-)
* [`SET`](#set-key-value)

### GET key

Get the value of key. If the key does not exist the special value `nil` is returned.

### SADD key member [member ...]

Add the specified members to the set stored at key. Specified members that are already a member of this set are ignored. If key does not exist, a new set is created before adding the specified members.

### SET key value

Set key to hold the string `value`. If key already holds a value, it is overwritten.


## Installation

TODO: add crate

### Compile from source

Clone the repo, then compile the project with:

    rustc src/main.rs -o red  # Compile the CLI
    rustc src/server.rs -o red-server  # Compile the TCP server

Alternatively, you can start an interactive shell with:

    cargo run
