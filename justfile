set shell := ["sh", "-cu"]

# Show available repository tasks.
default:
    @just --list

# Full pre-commit gate: format, lint, tests, wasm build, docs, checklist.
check:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    just wasm-check
    just docs-check
    sw-checklist

# Ensure the UI crate builds for the wasm target.
wasm-check:
    cargo build -p suduko-ui --target wasm32-unknown-unknown

# Validate every tracked markdown file.
docs-check:
    ./scripts/check-docs

# Apply canonical formatting.
fmt:
    cargo fmt --all
