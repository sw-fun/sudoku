# AGENTS.md

This repository uses the AgentRail saga/step process. `AGENTS.md` is the
single instruction target; do not create a duplicate `CLAUDE.md`.

## Project

Browser-based Sudoku in Rust: `crates/suduko-engine` is the pure, headless,
fully tested game engine; `crates/suduko-ui` is the Yew/WASM frontend.
See `docs/plan.md` for the delivery plan and `docs/sagas.md` for the queue.

## Session protocol

1. Run `agentrail next` first; it names the active step and its prompt.
2. Run `agentrail begin` before implementation work.
3. Work only the active step; do not silently fold in unplanned work (use
   `agentrail insert`/`add` instead).
4. Commit source and `.agentrail/` metadata together by named file before
   `agentrail complete`.
5. Stop after `agentrail complete`; further work belongs to the next step.
6. Never hand-edit append-only `.agentrail/` state (`saga.toml`, `step.toml`,
   `plan.md`, `sessions/`, `trajectories/`).

## Rules

- TDD is mandatory for executable behavior. Start with a focused failing
  test in the smallest affected crate; run it red, implement minimally, run
  it green. Statistical tests must use seeded RNGs so runs are deterministic.
- The pre-commit gate is `just check`: `cargo fmt --all -- --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, the wasm-target build of `suduko-ui`,
  `scripts/check-docs` (sw-markdown-checker over tracked markdown), and
  `sw-checklist`. Never commit with a red gate; never use `--no-verify`.
- Keep the engine pure: no WASM, browser, or filesystem dependencies in
  `crates/suduko-engine`. All rules expressible without a browser (state
  transitions, highlight sets, win detection, cookie round-trip) live in
  tested pure modules; Yew components stay thin.
- Stage files by explicit name (`git add path/to/file`). Never `git add -A`
  or `git add .`.
- Work directly on `main`. No feature branches, PRs, `gh`, or force pushes.
  Push with `git push origin main` and verify it before reporting complete.
- Documentation updates with the behavior it describes; every step leaves
  `docs/` consistent with the code.
- Run `sw-checklist` and fix every finding before the completion commit.

## Completion checklist per step

1. Focused red/green tests ran during development.
2. `just check` green immediately before commit.
3. Affected docs updated.
4. `.gitignore` audited for new artifacts; it stays narrow.
5. `git status --short` shows every intended file tracked.
6. Named-file staging; detailed commit message stating behavior, tests run,
   and limitations.
7. `agentrail complete` only after the commit; metadata committed if changed.
8. `git push origin main` verified.
