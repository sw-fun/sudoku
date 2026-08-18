# Suduko

Browser-based Sudoku built with Rust, Yew, and WASM.

- `crates/suduko-engine` - pure Rust engine: generation, uniqueness-proving
  solver, and human-technique difficulty grading for five levels.
- `crates/suduko-ui` - Yew/WASM frontend: board rendering, input, stats,
  and cookie-persisted game state.

Development is saga driven (AgentRail) and TDD.

- Delivery plan: `docs/plan.md`
- Saga queue: `docs/sagas.md`
- Agent rules: `AGENTS.md`

## Commands

- `just check` - full pre-commit gate (format, clippy, tests, wasm build,
  markdown validation, sw-checklist).
- `just fmt` - apply canonical formatting.
- `cargo test -p suduko-engine` - engine tests only.

## Engine modules

- `grid` - `Board` (81 row-major `Cell`s: `Empty` or `Value(1..=9)`),
  `CELL_COUNT`, and `coords` helpers `row_of`/`col_of`/`block_of`/`peers_of`
  (a cell has exactly 20 peers).
- `validate` - `first_conflict(board)` reports the first row/column/block
  duplicate as `(index, value)`; `None` means consistent so far.
- `format` - compact 81-char serialization (`.` or `0` = empty, `1`-`9` =
  value) via `to_string`/`parse`; `parse` rejects wrong lengths and bad
  characters.
- `game` - `Puzzle::new(clues, solution)` fails closed on incomplete,
  inconsistent, or mismatched solutions; exposes `solution()` and
  `clue_count()`.
- `solver` - deterministic backtracking (`solve`) with ascending digit order
  and most-constrained-cell selection; `count_solutions(board, cap)` stops
  at `cap` finds so `cap = 2` decides uniqueness; `is_solved` checks
  complete and consistent. Solves AI Escargot well under the 2s test bound.

Cell values are stored as 1..=9; any other `Value` payload violates the
documented invariant and is never produced by `parse` or generation.
