#!/bin/sh
# Build the wasm UI for GitHub Pages into ./pages at the repo root.
#
# Steps:
#   1. Force the suduko-ui crate to recompile so build.rs bakes fresh
#      provenance (SHA/date/host) into the footer. The rerun trigger
#      only fires on HEAD movement, so a rebuild with an unchanged HEAD
#      (or after committing docs/pages only) would otherwise reuse a
#      stale binary; touching the trigger refreshes it every time.
#   2. trunk release build with --public-url /sudoku/ (the GitHub Pages
#      sub-path for the sw-fun/sudoku repository) so hashed asset URLs
#      resolve under https://sw-fun.github.io/sudoku/.
#   3. Output goes to ./pages (committed and pushed on main), replacing any
#      previous bundle.
#
# Usage: scripts/build-pages.sh
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST="$ROOT/pages"
UI="$ROOT/components/ui/crates/suduko-ui"

touch "$UI/build.rs"

rm -rf "$DIST"
mkdir -p "$DIST"

( cd "$UI" \
    && "$ROOT/scripts/serial.sh" trunk build --release \
        --public-url /sudoku/ --dist "$DIST" )

echo "=== pages bundle: $DIST ==="
ls "$DIST"
