use crate::grid::{Board, CELL_COUNT, Cell};
use crate::rng::Rng;
use crate::solver;

/// Removes clues in shuffled order while the board keeps exactly one
/// solution, stopping at `params.target_clues` or when every remaining clue
/// is load-bearing. Termination is structural: each candidate is visited once.
pub fn dig(full: &Board, params: super::DigParams, rng: &mut Rng) -> Board {
    let mut board = full.clone();
    let mut clues = CELL_COUNT;
    let mut candidates: Vec<usize> = (0..CELL_COUNT).collect();
    rng.shuffle(&mut candidates);
    for &idx in &candidates {
        if clues <= params.target_clues {
            break;
        }
        if board.get(idx).is_empty() {
            continue;
        }
        let partner = if params.symmetric {
            Some(CELL_COUNT - 1 - idx)
        } else {
            None
        };
        let mut trial = board.clone();
        trial.set(idx, Cell::Empty);
        let mut removed = 1;
        if let Some(p) = partner {
            if p == idx {
                removed = 1;
            } else if board.get(p).is_empty() {
                continue;
            } else {
                trial.set(p, Cell::Empty);
                removed = 2;
            }
        }
        if solver::count_solutions(&trial, 2) == 1 {
            board = trial;
            clues -= removed;
        }
    }
    board
}
