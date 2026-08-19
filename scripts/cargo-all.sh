#!/bin/sh
# Run one cargo command in every component workspace, sequentially, each
# under the shared build lock. Usage: scripts/cargo-all.sh test
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
COMMAND="${1:-check}"
shift 2>/dev/null || true

for ws in "$ROOT"/components/*/; do
    echo "=== [$(basename "$ws")] cargo $COMMAND $* ==="
    ( cd "$ws" && "$ROOT/scripts/serial.sh" cargo "$COMMAND" "$@" )
done
