# Hexagonal Rust

A skeleton Rust command-line application setup with hexagonal architecture in mind.

## Test

Run print command

```
cargo run print
```

Set command arguments:

```
cargo run print --prefix "hello world" --value "my value"
```

## Dependencies

### CLI

Added [clap](https://github.com/clap-rs/clap) for command line argument parsing.

```
cargo add clap -F derive
```

### Errors

Added [anyhow](https://github.com/dtolnay/anyhow) to enable multiple error type returns from commands.

### Async

Added [tokio](https://github.com/tokio-rs/tokio) to enabled async execution and return different future
values for subcommands when using anyhow result.
