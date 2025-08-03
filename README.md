# ggit

![CI](https://github.com/GabrielBarros36/ggit/actions/workflows/ci.yml/badge.svg)

A terminal-based Git client built with Rust and ratatui.

## About

ggit is a work-in-progress terminal Git client that uses The Elm Architecture (TEA) pattern for state management. It provides a text-based user interface for common Git operations.

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## Development

- **Format:** `cargo fmt`
- **Check:** `cargo check`
- **Test:** `cargo test`
- **Clippy:** `cargo clippy`

## Key Bindings

- `q` or `Esc` - Quit
- `1-4` - Switch views
- `j/k` or arrow keys - Navigate
- `r` or `F5` - Refresh

## Dependencies

- [git2](https://crates.io/crates/git2) - Git operations
- [ratatui](https://crates.io/crates/ratatui) - Terminal UI framework
