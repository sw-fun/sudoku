//! Uniqueness-preserving clue removal.

use crate::rng::Rng;
use suduko_grid::{Board, CELL_COUNT, Cell};
use suduko_solver::count_solutions;

/// Digging controls: stop removing clues at `target_clues`, optionally
/// removing point-symmetric cell pairs (idx and 80 - idx) only.
#[derive(Clone, Copy, Debug)]
pub struct DigParams {
    pub target_clues: usize,
    pub symmetric: bool,
}

/// Removes clues in shuffled order while the board keeps exactly one
/// solution, stopping at the target or when every remaining clue is
/// load-bearing. Termination is structural: each candidate is visited once.
pub fn dig(full: &Board, params: DigParams, rng: &mut Rng) -> Board {
    let mut board = full.clone();
    let mut clues = CELL_COUNT;
    let mut candidates: Vec<usize> = (0..CELL_COUNT).collect();
    rng.shuffle(&mut candidates);
    for &idx in &candidates {
        if clues > params.target_clues
            && board.get(idx) != Cell::Empty
            && let Some((trial, removed)) = try_remove(&board, idx, params)
            && count_solutions(&trial, 2) == 1
        {
            board = trial;
            clues -= removed;
        }
    }
    board
}

/// Trial removal of `idx` (and its point-symmetric partner when configured),
/// returning the trial board and how many clues it removes.
fn try_remove(board: &Board, idx: usize, params: DigParams) -> Option<(Board, usize)> {
    let partner = params.symmetric.then_some(CELL_COUNT - 1 - idx);
    let mut trial = board.clone();
    trial.set(idx, Cell::Empty);
    let mut removed = 1;
    if let Some(p) = partner {
        if p == idx {
            removed = 1;
        } else if board.get(p) != Cell::Empty {
            trial.set(p, Cell::Empty);
            removed = 2;
        } else {
            return None;
        }
    }
    Some((trial, removed))
}
