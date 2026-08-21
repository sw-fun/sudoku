# Sudoku Delivery Plan

Browser-based Sudoku built with Rust, Yew, and WASM. Pure game logic lives in
a headless, fully unit-tested engine crate; the Yew crate renders and routes.
Development is TDD and AgentRail saga driven, modeled on the conventions in
`../../sw-ml-study/demo-extensions` (AGENTS.md gate, `just check` pre-commit,
direct-main commits, docs updated with behavior).

## Product summary

- Difficulty select screen: easy, medium, hard, harder, hardest.
- 9x9 grid. Click a cell to highlight it.
  - If the cell is empty, lightly highlight its row, column, and 3x3 block.
  - If the cell holds a number, highlight every other cell currently showing
    that same number, excluding bad guesses (wrong user values never trigger
    same-number highlighting).
- Enter a value into the highlighted cell by typing 1..9 or clicking a 1..9
  button. An erase button (and spacebar on the highlighted cell) clears it.
- Givens are fixed; user values are editable.
- A user value that differs from the solution is colored red.
- Each wrong entry bumps a displayed bad-inputs counter.
- When the final empty cell is filled correctly: flash the screen and show
  elapsed time; the next board at the same difficulty is a different board.
- Escape quits the board and returns to the difficulty and stats view. A menu
  button returns to the initial screen at any time.
- Game state (current board, user values, elapsed time, bad count, per-level
  stats) persists in browser cookies and survives reload.

## Architecture

Top-level component workspaces (the sw-mlpl pattern), each its own cargo
workspace; one shared root `target/` via `.cargo/config.toml`; every
cargo/trunk call serialized through `scripts/serial.sh`; `scripts/
cargo-all.sh` runs one cargo command across all workspaces.

- `components/engine` - pure Rust, no WASM/browser dependencies.
  - `suduko-grid` - board model, coords, serialization, Puzzle
  - `suduko-solver` - backtracking solve + capped count (uniqueness)
  - `suduko-generator` - seeded RNG, random full grids, safe digging
  - `suduko-techniques` - candidates, effects, singles/intersections/
    subsets
  - `suduko-techniques-advanced` - XY-wing, X-wing, swordfish
  - `suduko-grader` - ladder chain of responsibility, trial fallback,
    grade
  - `suduko-engine` - facade: Level, band config, generate dispatch
- `components/ui` - `suduko-ui` Yew/WASM application. Components, input
  handling, highlighting, cookie persistence. All rules expressible
  without a browser (state transitions, selection/highlight sets, win
  detection, cookie round-trip) are implemented as pure functions or
  reducer-style logic in modules with their own tests; Yew components
  stay thin. Served by `basic-http-server -a 0.0.0.0:9501` (Rust file
  server; never Python).


## Engine design

### Grid model

- `Cell` = given or user value 1..9 or empty; `Board` = 81 cells plus the
  solution.
- Row, column, and block index helpers; peer-set computation.
- Serialization: compact string forms for puzzles, solutions, and state.

### Solver

- Deterministic backtracking solver with candidate ordering.
- Solution counter capped at 2 to prove uniqueness without exhaustively
  counting all solutions.

### Generation

- Generate a random full valid grid with randomized backtracking.
- Dig clues from the full grid while preserving a unique solution.
- Keep givens count as a secondary constraint per difficulty band.

### Difficulty grading (the robustness core)

Grade with a human-technique logical solver, not givens count alone:

1. naked single
2. hidden single
3. locked candidates (pointing and claiming)
4. naked pair / naked triple
5. hidden pair / hidden triple
6. X-wing
7. swordfish / simpler coloring (hardest tier)

Each technique carries a weight. The grader always applies the cheapest
available technique and records which techniques were required. The puzzle
score is derived from technique weights used (max tier plus use count).

Level bands:

- easy: solvable with naked and hidden singles only; generous givens.
- medium: singles plus locked candidates; moderate givens.
- hard: also requires pairs/triples.
- harder: also requires X-wing-class techniques.
- hardest: not solvable by the full technique ladder (requires bounded
  trial); fewest givens.

Generation loop: generate full grid, dig, verify uniqueness, grade; accept
only when the score falls inside the level band and the givens count stays in
range; otherwise re-dig or regenerate. A per-level attempt cap plus regenerate
keeps generation bounded.

Statistical gate tests (deterministic with a seeded RNG): sampled puzzles per
level stay inside their bands, mean scores are strictly monotone across
levels, every published puzzle has exactly one solution, and an easy puzzle
is always singles-solvable (an easy board is never too hard) while a hardest
puzzle is never singles-solvable (a hard board is never too easy).

## UI design

- Screens: menu (difficulty select + stats), game, solved overlay. The
  solved overlay stacks above the board (z-index above the selected cell)
  so no board numbers show through the dialog. The board carries row and
  column coordinate labels (1-9) so teaching references like "r6c1" and
  "row 6" are locatable.
- Teaching (learn mode): `components/tutor` is a separate pure workspace
  (`suduko-tutor`) whose finders return every applicable strategy for the
  current board - naked/hidden single, pointing/claiming, naked/hidden
  pair, X-Wing, XY-Wing - each with pattern cells, units, digits, the
  safe inference (placement or eliminations), and ordered teaching steps;
  `find_all` returns them ladder-ordered and deduplicated. The game
  screen has a Learn toggle: opening it computes the offers for the
  current board, shows candidate pencil marks in empty cells (teaching
  only), lists every applicable strategy, and selecting one opens a
  step-by-step walkthrough shell (Back/Next/Close with clamped cursor);
  the pure state lives in `suduko-game::Teaching`. During a walkthrough
  the board renders per-step emphasis: pattern cells outlined blue,
  involved units tinted green, focus digits emphasized in pencil marks,
  elimination candidates pulsing red with strike-through, and placement
  targets pulsing green (CSS keyframes; pure derivation in
  `Game::step_view`). Show-me mode makes the game solve the current
  board itself, strategy by strategy: a Show me toggle starts the
  cheapest applicable walkthrough, Next turns into Apply-and-continue,
  placements land on the board (via the normal input rules) and
  eliminations accumulate in a persistent solver layer that feeds
  pencil marks and future strategy discovery (`suduko-game::showme`);
  an Auto checkbox with a 1s/3s/6s speed selector (default 3s) rides
  the tick, and a manual Next pauses Auto for step-wise control. When
  the eight taught rungs are exhausted, a Trial fallback teaches a
  solution-confirmed placement in the most constrained cell so show-me
  finishes any level.
- Selection state: optional highlighted cell index.
- Highlight sets computed by pure functions:
  - empty selected cell -> row, column, block peer set (light highlight).
  - valued selected cell -> set of cells showing the same value, minus
    incorrect user guesses.
- Wrong-value detection compares user value against the stored solution;
  wrong cells render red and increment the bad-inputs counter on entry.
- Input: keyboard 1..9 and spacebar via keydown; button pad 1..9 plus erase.
  A pad button grays out and disables once all nine instances of its digit
  are placed correctly (wrong placements never count; erasing reopens it).
- Win detection: no empty cells and no wrong values -> flash animation,
  elapsed-time display, record stats, offer next board (different puzzle,
  same difficulty; generator rejects the previous puzzle seed).
- Escape on the game screen returns to difficulty and stats; menu button is
  always available.
- Cookie persistence: versioned, compact state document; loaded on mount;
  written on every state change; tolerant of missing/corrupt cookies.
- Serving: `trunk build` produces static WASM assets served by a Rust file
  server only - `just serve` runs `basic-http-server -a 0.0.0.0:9501` (or an
  equivalent in-repo Rust server script). Never Python.

## TDD and gates

- Every behavior starts as a focused failing test in the smallest crate.
- `just check` runs: `cargo fmt --check`, `cargo clippy -- -D warnings`,
  `cargo test` (workspace), `sw-markdown-checker` on changed docs, and
  `sw-checklist`. No commit while any gate is red.
- Statistical tests use seeded RNGs so CI is deterministic.

## Sagas

See `docs/sagas.md` for the queue. Only one saga is active at a time.

1. `sudoku-engine` (created) - repo scaffold, grid model, solver, generator,
   technique grader, band-targeted generation, statistical gates, acceptance
   report.
2. `sudoku-yew-game` (queued) - Yew shell, board render, input handling,
   highlight rules, win flow, cookie persistence, navigation/stats,
   acceptance report.

## Acceptance

- Every published puzzle has exactly one solution and a score inside its
  level band; level bands are statistically separated and monotone.
- All specified interactions work in the browser: selection, both highlight
  rules, red wrong values, bad-input counter, erase via button and spacebar,
  keyboard and button entry, win flash with elapsed time, next-board
  rotation, Escape and menu navigation, and cookie-persisted state.
- `just check` is green; docs and AGENTS.md reflect delivered behavior.
