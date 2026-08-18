mod dig;
mod full;

use crate::game::Puzzle;
use crate::grid::Board;
use crate::rng::Rng;
use crate::solver::state::State;

/// Digging controls: stop removing clues at `target_clues`, optionally
/// removing point-symmetric cell pairs (idx and 80 - idx) only.
#[derive(Clone, Copy, Debug)]
pub struct DigParams {
    pub target_clues: usize,
    pub symmetric: bool,
}

/// Generates a complete, valid solution grid from `seed`.
pub fn generate_full(seed: u64) -> Board {
    let mut rng = Rng::new(seed);
    let mut state = State::from_board(&Board::new());
    assert!(
        full::random_full(&mut state, &mut rng),
        "a full grid always exists for the empty board"
    );
    Board::from_cells(state.cells)
}

/// Generates a puzzle whose clues are dug from a seeded full grid while
/// preserving a unique solution, stopping at the target or when no clue can
/// be removed without creating a second solution.
pub fn generate_puzzle(seed: u64, params: DigParams) -> Puzzle {
    let mut rng = Rng::new(seed);
    let mut state = State::from_board(&Board::new());
    assert!(
        full::random_full(&mut state, &mut rng),
        "a full grid always exists for the empty board"
    );
    let full = Board::from_cells(state.cells);
    let clues = dig::dig(&full, params, &mut rng);
    Puzzle::new(clues, full).expect("dug clues always agree with the full grid")
}

/// Digs clues out of an existing full grid with its own seeded shuffle,
/// preserving a unique solution. Used by the difficulty loop to draw many
/// independent puzzles from one grid.
pub fn dig_with(full: &Board, params: DigParams, seed: u64) -> Puzzle {
    let clues = dig::dig(full, params, &mut Rng::new(seed));
    Puzzle::new(clues, full.clone()).expect("dug clues always agree with the full grid")
}
