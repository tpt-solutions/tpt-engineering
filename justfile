# tpt-engineering — developer tasks
#
# Usage: `just <task>`  (install with `cargo install just`)
# Each task mirrors `cargo xtask` so the surface is the same locally and in CI.

# Run the full local hygiene gate: fmt check + clippy + cargo-deny.
check:
    cargo xtask check

# Run all unit + integration tests.
test:
    cargo test --workspace --all-features

# Run documentation tests (includes the tpt-eng-examples scenario).
doctest:
    cargo xtask doctest

# Build the docs.
doc:
    cargo xtask doc

# Build the no_std-capable crates for thumbv6m-none-eabi.
no-std:
    cargo xtask no-std-matrix

# Replicate the entire CI matrix locally (requires the wasm32 + thumbv6m
# targets: `rustup target add wasm32-unknown-unknown thumbv6m-none-eabi`).
ci: check test doctest doc no-std
    cargo build -p tpt-eng-props-water --no-default-features --target wasm32-unknown-unknown
    cargo build -p tpt-eng-props-air   --no-default-features --target wasm32-unknown-unknown
    cargo build -p tpt-eng-props-fuels --no-default-features --target wasm32-unknown-unknown
    cargo build -p tpt-eng-props       --no-default-features --target wasm32-unknown-unknown

# Scaffold a new tpt-eng-* crate.
new name:
    cargo xtask new-crate {{name}}

# List available tasks.
default:
    @just --list
