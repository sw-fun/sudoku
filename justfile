set shell := ["sh", "-cu"]

# Show available repository tasks.
default:
    @just --list

# Full pre-commit gate: format, lint, tests, wasm build, docs, checklist.
check:
    just fmt-check
    just clippy
    just tests
    just docs-check
    sw-checklist

# Verify canonical formatting across all component workspaces.
fmt-check:
    ./scripts/cargo-all.sh fmt --all --check

# Apply canonical formatting across all component workspaces.
fmt:
    ./scripts/cargo-all.sh fmt --all

# Clippy with warnings denied across all component workspaces.
clippy:
    ./scripts/cargo-all.sh clippy --workspace --all-targets -- -D warnings

# Run tests across all component workspaces.
tests:
    ./scripts/cargo-all.sh test --workspace

# Ensure the UI crate builds for the wasm target.
wasm-check:
    cd components/ui && ../../scripts/serial.sh cargo build -p suduko-ui --target wasm32-unknown-unknown

# Validate every tracked markdown file.
docs-check:
    ./scripts/check-docs

# Check everything, then build the wasm UI bundle (scripts/build.sh).
build:
    just check
    ./scripts/build.sh

# Serve the built wasm UI with the Rust file server on 0.0.0.0:9501
# (scripts/serve.sh runs basic-http-server; never Python).
serve:
    ./scripts/serve.sh
