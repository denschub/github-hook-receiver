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

By default, the server listens to `127.0.0.1:3000`. The configuration root
directory and the server's listen address can be defined using command line
arguments:

```
./github-hook-receiver <config dir> [<listen address>]
```

Please do not add a trailing slash to the config dir. You can omit the listen
address if you are fine with `127.0.0.1:3000`.

## Configuration files

All configuration files have to be within the root directory mentioned above.
The file name is the repositories full name with the slash replaced by two
underscores. The config file for this repository would be
`denschub__github-hook-receiver.json`. All available fields:

```json
{
  "command": "/home/fancyapp/update.sh",
  "refs": [
    "refs/heads/master"
  ],
  "secret": "supersecretsecret"
}
```

You can omit `secret` if you do not want to set one on GitHub. But you should.
