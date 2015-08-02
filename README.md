# github-hook-receiver
[![Build Status](https://api.travis-ci.org/denschub/github-hook-receiver.svg)](http://travis-ci.org/denschub/github-hook-receiver)

The `github-hook-receiver` is a simple GitHub Webhook receiver based written in
Rust. It basically listens for all incoming hooks, checks if there is a
configuration file for the incoming repository and executes a command.

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
  "handlers": {
    "pull_request": "/home/fancyapp/pr_quality_control.sh",
    "push": "/home/fancyapp/deploy.sh"
  },
  "refs": [
    "refs/heads/master"
  ],
  "secret": "supersecretsecret"
}
```

You can omit `secret` if you do not want to set one on GitHub. But you should.

## Available events

You can handle [all available Webhook
events](https://developer.github.com/webhooks/#events) provided by GitHub. Just
specify a handler using the events name as shown in the example above.
Additional data may be available inside environmental variables, see below for
more information.

## Environment variables inside handlers

Some environment variables get set by the receiver to allow further processing
by the handler script.

## Push

* `HEAD`: The new HEAD sha1 hash.

### Pull Request

* `ACTION`: The events action, see [GitHubs documentation](https://developer.github.com/v3/activity/events/types/#pullrequestevent).
* `BASE`: The PRs base head.
* `PR`: The number of the pull request.
