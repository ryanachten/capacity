# Sprint 🏃‍♀️

A dead simple CLI tool for calculating sprint capacity

## Usage

- `sprint`
  - First time use will set sprint configuration (team members, number of sprint days etc)
  - Subsequent use will just require number of days leave to calculate sprint capacity
- `sprint config`
  - Prints current sprint configuration
  - `--reset`
    - Resets sprint configuration

## Development

- [Install Rust and Cargo](https://doc.rust-lang.org/book/ch01-01-installation.html)
- Build via `cargo build`
- Run tests via `cargo test`
