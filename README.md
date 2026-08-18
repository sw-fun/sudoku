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
