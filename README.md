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
- `rng` - SplitMix64 with a pinned golden stream; seeding an `Rng` with the
  same seed reproduces the same numbers forever.
- `generator` - `generate_full(seed)` builds a complete valid grid via
  randomized backtracking; `generate_puzzle(seed, DigParams)` digs clues in
  shuffled order (optionally point-symmetric pairs), removing a clue only
  when the capped counter still proves a unique solution. Digging stops at
  `target_clues` or when every remaining clue is load-bearing, so it always
  terminates. Same seed, same puzzle.
- `grader` - human-technique logical solver that always applies the cheapest
  available technique from the ladder naked single, hidden single, locked
  candidates (pointing/claiming), naked pair/triple, hidden pair/triple,
  XY-wing, X-wing, swordfish, then a bounded-trial fallback (depth 8).
  `grade(board)` reports per-technique counts, the hardest technique
  required, and a score of `weight * 100 + total applications`, giving
  hundred-wide bands: 100-299 singles only, 300-399 locked candidates,
  400-599 subsets, 600-699 XY-wing, 700-799 basic fish, 800+ trial. Hidden
  subsets are rare in practice because a hidden k-subset implies a
  complementary naked set that the ladder applies first; the technique is
  pinned by crafted-mask tests (fires on a real hidden pair, stays silent on
  a degenerate one), and a truth-tracking test proves no elimination ever
  removes a solution digit.
- `difficulty` - five published levels banded by the hardest technique
  required: easy = singles only, medium = locked candidates, hard = naked or
  hidden subsets, harder = XY-wing/X-wing/swordfish, hardest = beyond the
  ladder (bounded trial). `generate(level, seed)` runs the acceptance loop:
  fresh seeded grids, independent digs per grid, accept only in-band puzzles
  with a clue count inside the level window (easy 38-55 down to hardest
  22-30), failing closed with `LevelError::Exhausted` after the attempt
  caps (24 digs x 3 grids). Same seed, same puzzle. Empirically the
  "harder" tier exists because XY-wing (added as the common wing-class
  technique) unlocks puzzles the basic-fish ladder would otherwise send to
  trial; pure X-wing-required puzzles appear at roughly 1 per 80 deep digs.

Cell values are stored as 1..=9; any other `Value` payload violates the
documented invariant and is never produced by `parse` or generation.
