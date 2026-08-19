//! Random complete-grid generation via randomized backtracking.

use crate::rng::Rng;
use suduko_grid::Board;
use suduko_solver::{Pick, State, pick_cell};

/// Generates a complete, valid solution grid from `seed`. The random fill
/// always succeeds on an empty board; the loop+flag shape keeps the
/// recursion honest if reused on partial boards.
///
/// # Panics
///
/// Asserts the fill succeeded, which is guaranteed for an empty board.
#[must_use]
pub fn generate_full(seed: u64) -> Board {
    let mut state = State::empty();
    let mut rng = Rng::new(seed);
    assert!(
        random_fill(&mut state, &mut rng),
        "a full grid always exists for the empty board"
    );
    Board::from_cells(state.cells)
}

fn random_fill(state: &mut State, rng: &mut Rng) -> bool {
    match pick_cell(state) {
        Pick::Solved => true,
        Pick::DeadEnd => false,
        Pick::Cell(idx) => try_shuffled(state, idx, rng),
    }
}

fn try_shuffled(state: &mut State, idx: usize, rng: &mut Rng) -> bool {
    let mask = state.candidates(idx);
    let mut digits: Vec<u8> = (1..=9).filter(|d| mask & (1 << (d - 1)) != 0).collect();
    rng.shuffle(&mut digits);
    for digit in digits {
        state.place(idx, digit);
        if random_fill(state, rng) {
            return true;
        }
        state.unplace(idx, digit);
    }
    false
}
