#!/bin/sh
# Serve the built wasm UI with a Rust file server on 0.0.0.0:9501.
#
# Steps:
#   1. Refuse to serve a stale tree: build the bundle unless dist/ already
#      exists (run scripts/build.sh to refresh it).
#   2. basic-http-server (a Rust static file server; never Python) bound
#      to 0.0.0.0:9501 serving components/ui/crates/suduko-ui/dist/.
#
# Usage: scripts/serve.sh            (serve existing dist/, build if absent)
#        scripts/serve.sh --build    (rebuild, then serve)
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST="$ROOT/components/ui/crates/suduko-ui/dist"

if [ "${1:-}" = "--build" ] || [ ! -d "$DIST" ]; then
    "$ROOT/scripts/build.sh" --bundle
fi

command -v basic-http-server >/dev/null 2>&1 || {
    echo "basic-http-server not found; install with: cargo install basic-http-server" >&2
    exit 1
}

echo "=== serving $DIST on http://0.0.0.0:9501 ==="
cd "$DIST" && exec basic-http-server -a 0.0.0.0:9501
