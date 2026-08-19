# Saga Queue

Only one saga is active at a time. Steps are independently reviewable, use
red/green TDD, and follow the AGENTS.md completion checklist (pre-commit
gates, docs, .gitignore audit, named-file staging, detailed commit, AgentRail
completion metadata, verified push). Work is directly on `main`.

The active saga plan file in `.agentrail/plan.md` is authoritative for step
prompts; `docs/plan.md` is the delivery plan.

## Active: sudoku-engine

Purpose: deliver the complete headless game engine - grid model, uniqueness-
proving solver, generator, human-technique difficulty grader, and band-
targeted generation with statistical gates - with every behavior TDD'd.

1. `repo-scaffold` - Cargo workspace with engine and UI crate skeletons,
   justfile gate (fmt, clippy, test, markdown checker, checklist), AGENTS.md
   discipline, and structural tests.
2. `grid-model` - cell/board model, peers, constraint validation, compact
   serialization with round-trip tests.
3. `solver-uniqueness` - backtracking solver plus capped solution counter
   proving uniqueness, with performance sanity tests.
4. `generator` - random full-grid generation and uniqueness-preserving clue
   digging with seeded deterministic tests.
5. `technique-grader` - human-technique ladder (naked/hidden single, locked
   candidates, pairs/triples, X-wing, trial depth) with fixture puzzles of
   known required techniques.
6. `difficulty-banding` - level bands, accept/reject generation loop, givens
   ranges, attempt caps.
7. `statistical-gates` - seeded sample tests proving band membership,
   strict monotone mean scores across levels, easy-is-singles-solvable,
   hardest-is-not.
8. `engine-acceptance` - run full gates, publish engine acceptance report in
   docs, reconcile the next saga, stop.

Acceptance: every generated puzzle is unique-solution and inside its band;
levels are statistically separated and monotone; `just check` green.

## Queued: sudoku-yew-game

Purpose: build the Yew/WASM application on the accepted engine.

1. `yew-shell` - app scaffold, build/run scripts, menu and game screens,
   difficulty selection; `just serve` runs the built assets under
   `basic-http-server -a 0.0.0.0:9501` (Rust file server, never Python).
2. `board-render` - grid component, givens vs user values, selection
   highlight for row/column/block of an empty cell.
3. `input-rules` - keyboard 1..9 and spacebar, button pad 1..9, erase button,
   pure input reducer tests.
4. `wrong-value-and-highlights` - same-number highlighting minus bad
   guesses, red wrong values, bad-input counter.
5. `win-flow` - completion detection, screen flash, elapsed time, stats
   update, next different board at same difficulty.
6. `cookie-persistence` - versioned state document in cookies, load on
   mount, save on change, corrupt-cookie tolerance.
7. `navigation` - Escape quits to difficulty/stats view, menu button to
   initial screen.
8. `game-acceptance` - full gate run, browser smoke checklist, acceptance
   report.

Replan this saga when `sudoku-engine` completes; adjust steps to the engine
API actually accepted.
