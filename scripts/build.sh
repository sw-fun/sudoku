#!/bin/sh
# Build every component workspace, then the wasm UI bundle.
#
# Steps:
#   1. cargo check + test across components/engine, components/tutor,
#      and components/ui (each under the shared build lock via
#      scripts/serial.sh).
#   2. trunk release build of the Yew app in components/ui/crates/suduko-ui,
#      emitting static assets to components/ui/crates/suduko-ui/dist/.
#
# Usage: scripts/build.sh          (checks + bundle)
#        scripts/build.sh --bundle (bundle only)
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [ "${1:-}" != "--bundle" ]; then
    echo "=== [engine] cargo check + test ==="
    ( cd "$ROOT/components/engine" && "$ROOT/scripts/serial.sh" cargo test --workspace )
    echo "=== [tutor] cargo check + test ==="
    ( cd "$ROOT/components/tutor" && "$ROOT/scripts/serial.sh" cargo test --workspace )
    echo "=== [ui] cargo check + test ==="
    ( cd "$ROOT/components/ui" && "$ROOT/scripts/serial.sh" cargo test --workspace )
fi

echo "=== [ui] trunk release build ==="
( cd "$ROOT/components/ui/crates/suduko-ui" && "$ROOT/scripts/serial.sh" trunk build --release )

echo "=== build done: components/ui/crates/suduko-ui/dist/ ==="
