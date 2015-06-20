# github-hook-receiver
[![Build Status](https://api.travis-ci.org/denschub/github-hook-receiver.svg)](http://travis-ci.org/denschub/github-hook-receiver)

The `github-hook-receiver` is a simple GitHub Webhook receiver based written in
Rust. It basically listens for all incoming hooks, checks if there is a
configuration file for the incoming repository and execute a command on `push`
notifications.

## Building

Assuming the Rust toolchain (the Rust compiler and Cargo) is already set up,
building `github-hook-receiver` is as easy as running `cargo build --release`.
The built binary will be stored as `target/release/github-hook-receiver`.

## Using

By default, the server listens to `127.0.0.1:3000`. To change that, change the
`LISTEN` environment variable, `LISTEN=0.0.0.0:8888` for example.

## Configuration

ToDo.
