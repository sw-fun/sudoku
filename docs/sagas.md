# Saga Queue

Only one saga is active at a time. Steps are independently reviewable, use
red/green TDD, and follow the AGENTS.md completion checklist (pre-commit
gates, docs, .gitignore audit, named-file staging, detailed commit, AgentRail
completion metadata, verified push). Work is directly on `main`.

The active saga plan file in `.agentrail/plan.md` is authoritative for step
prompts; `docs/plan.md` is the delivery plan.

## Completed: sudoku-engine (closes after 010)

Purpose: deliver the complete headless game engine as component workspaces:
grid model, uniqueness-proving solver, generator, human-technique grader,
band-targeted generation with statistical gates, then the sw-mlpl-style
decomposition into per-component cargo workspaces.

Status: steps 1-9 complete; see `docs/engine-acceptance.md` for gate
evidence, the large-sample table (150/150 unique, monotone means
137/356/467/679/924), and API notes for the UI saga. Step 010
`harden-generation-caps` closes the saga.

1. `repo-scaffold` - done
2. `grid-model` - done
3. `solver-uniqueness` - done
4. `generator` - done
5. `technique-grader` - done (added XY-wing to make the harder tier
   reachable; pure fish-requiring puzzles do not occur from this digger)
6. `difficulty-banding` - done
7. `statistical-gates` - done (no band tuning needed)
8. `decompose-engine-components` - done (sw-mlpl pattern; two port
   regressions found and fixed: trial depth-0 semantics, inconsistent
   board grading fail-fast)
9. `engine-acceptance` - done (report in docs/engine-acceptance.md)
10. `harden-generation-caps` - pending: raise the 24x3 caps so measured
    exhaustion is zero across a seed sweep per level; regression-test the
    recorded exhausting seeds; update `docs/difficulty-stats.md`.

## Active: sudoku-yew-game (replanned, two steps)

Purpose: build the Yew/WASM application on the accepted engine. Replanned
from eight steps into two at the user's direction; the first step ends
with a playable board and README screen captures.

1. `yew-playable-game` - Menu screen (five difficulty buttons, stats
   view), game screen (board render with givens vs user values, selection,
   row/column/block highlight for an empty cell, same-number highlight
   excluding bad guesses, red wrong values, bad-input counter, keyboard
   1..9 and spacebar, button pad 1..9 and erase, elapsed time, win flash
   with next-board rotation at the same difficulty, Escape and menu
   navigation). Pure game-state transitions TDD'd in a state module;
   components stay thin. Trunk bundle served by scripts/serve.sh
   (basic-http-server -a 0.0.0.0:9501). Capture menu and in-play board
   screenshots with playwright-cli and embed them in README.
2. `yew-persistence-polish` - Versioned cookie persistence (load on mount,
   save on change, corrupt-cookie tolerance), per-level stats persisted,
   next-board rotation continuity, final acceptance report.

Acceptance: every specified interaction works in the browser; pure logic
is unit-tested; the gate is green; docs match delivered behavior.
