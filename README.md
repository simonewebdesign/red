# Red 🔴

> Fast, reliable key-value storage. Inspired by Redis, built with Rust.

Red has a similar interface to Redis. It has two executables:

- `red` - an interactive shell (like `redis-cli`)
- `red-server` - a TCP server (like `redis-server`)

## Commands

* [`GET`](#get-key)
* [`SADD`](#sadd-member)
* [`SET`](#set-key-value)
* [`SMEMBERS`](#smembers)
* [`SREM`](#srem-member)

### GET key

Get the value of key. If the key does not exist the special value `nil` is returned.

### SADD member

Add the member to the set. If the member is already in the set, it will be ignored.

### SET key value

Set key to hold the string `value`. If key already holds a value, it is overwritten.

### SMEMBERS

Returns all the members of the set.

### SREM member

Remove the specified member from the set. If the member is not in the set, it will be ignored.


## Installation

TODO: add crate

### Compile from source

Clone the repo, then compile the project with:

    rustc src/main.rs -o red  # Compile the CLI
    rustc src/server.rs -o red-server  # Compile the TCP server

Alternatively, you can start an interactive shell with:

    cargo run
