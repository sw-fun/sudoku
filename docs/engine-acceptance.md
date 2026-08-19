# Engine Acceptance Report

Saga: `sudoku-engine`, accepted 2026-08-18. Evidence measured on the
post-decomposition component layout (steps 1-8; see `components/engine`)
with seeded, deterministic runs. The component split re-run reproduced the
pre-split table exactly (same means, same uniqueness), confirming the port
preserved behavior after its two fixed regressions (trial depth-0
semantics, inconsistent-board fail-fast grading).

## Gate evidence

- `just check` green immediately before the acceptance commit: per-workspace
  cargo fmt --check, clippy --workspace --all-targets -D warnings (pedantic),
  cargo test --workspace (41 tests across 8 crates), wasm32 build of
  suduko-ui, sw-markdown-checker over tracked markdown, sw-checklist
  (36 passed, 0 failed).
- `suduko-engine` tests: difficulty suite (band membership, uniqueness,
  clue windows, determinism, fail-closed budgets) plus statistical_gates
  (five samples per level; singles-only easy; never-singles hardest;
  strictly monotone mean scores). Suite completes in about 2s.

## Large sample run (30 puzzles per level, base seed 50000)

| Level   | Mean | Min | Max | Clues | Unique | Avg per puzzle |
|---------|------|-----|-----|-------|--------|----------------|
| easy    | 137  | 137 | 137 | 44    | 30/30  | 1 ms           |
| medium  | 356  | 354 | 360 | 28    | 30/30  | 97 ms          |
| hard    | 467  | 456 | 561 | 26-27 | 30/30  | 188 ms         |
| harder  | 679  | 656 | 859 | 26    | 30/30  | 229 ms         |
| hardest | 924  | 906 | 939 | 24-27 | 30/30  | 45 ms          |

- Mean scores strictly monotone: 137 < 356 < 467 < 679 < 924.
- Uniqueness: 150/150 samples have exactly one solution.
- Band membership: 150/150 graded inside their level's technique band.
- Generation cost stays interactive (worst average 229 ms).

## Reliability finding (open, step 010)

A 100-seed sweep per level (base 50000) against the default caps
(24 digs x 3 grids = 72 attempts) measured exhaustion rates:
medium 1 percent, hard 6 percent, harder 7 percent, hardest 0 percent.
Exhausting seeds recorded: medium [51063]; hard [52007, 52021, 52033,
52042, 52073, 52086]; harder [53001, 53013, 53042, 53055, 53056, 53060,
53096]. The large-sample table above used a documented seed-walk
mitigation (on Exhausted, retry with seed + 977), which the UI must apply
until step 010 `harden-generation-caps` raises the caps and re-measures.

## API notes for the UI saga

- `suduko_engine::generate(Level, seed) -> Result<Puzzle, LevelError>` is
  the game entry point; `Level` is `Copy + Eq` (cookie-storable via a
  stable name mapping the UI defines).
- `Puzzle` exposes `clues()`, `solution()`, `clue_count()`; wrong-value
  checking compares a user cell against `solution().get(idx)`.
- `suduko_grid::to_string`/`parse` round-trip boards as 81-char strings
  ('.' or '0' empty); use them inside the cookie state document.
- `suduko_grid::coords::{row_of, col_of, block_of, peers_of}` give the
  empty-cell highlight sets; `Board::get/set` are row-major over
  `CELL_COUNT`.
- Generation is deterministic per seed; the UI should derive fresh seeds
  (time or counter based) when serving the next board and must not reuse
  the solved puzzle's seed.
- The engine is pure (no WASM/browser/filesystem deps); the UI workspace
  consumes the crates by path across component workspaces.
- `grade` fails fast on clue-inconsistent boards and does not support
  near-empty boards (documented precondition; the UI never grades).

## Sagas

`sudoku-engine` closes after step 010 (inserted reliability fix);
`sudoku-yew-game` is promoted for replanning in `docs/sagas.md`.

