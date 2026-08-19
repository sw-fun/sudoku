#!/bin/sh
# Build the wasm UI for GitHub Pages into ./pages at the repo root.
#
# Steps:
#   1. trunk release build with --public-url /sudoku/ (the GitHub Pages
#      sub-path for the sw-fun/sudoku repository) so hashed asset URLs
#      resolve under https://sw-fun.github.io/sudoku/.
#   2. Output goes to ./pages (committed and pushed on main), replacing any
#      previous bundle.
#
# Usage: scripts/build-pages.sh
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST="$ROOT/pages"

rm -rf "$DIST"
mkdir -p "$DIST"

( cd "$ROOT/components/ui/crates/suduko-ui" \
    && "$ROOT/scripts/serial.sh" trunk build --release \
        --public-url /sudoku/ --dist "$DIST" )

echo "=== pages bundle: $DIST ==="
ls "$DIST"
