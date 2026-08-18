use crate::rng::Rng;
use crate::solver::search::{Pick, pick_cell};
use crate::solver::state::State;

/// Depth-first fill with shuffled candidate digits; always succeeds on the
/// empty board, so the boolean exists only to satisfy the recursion shape.
pub fn random_full(state: &mut State, rng: &mut Rng) -> bool {
    match pick_cell(state) {
        Pick::Solved => true,
        Pick::DeadEnd => false,
        Pick::Cell(idx) => {
            let mask = state.candidates(idx);
            let mut digits: Vec<u8> = (1..=9).filter(|d| mask & (1 << (d - 1)) != 0).collect();
            rng.shuffle(&mut digits);
            for digit in digits {
                state.place(idx, digit);
                if random_full(state, rng) {
                    return true;
                }
                state.unplace(idx, digit);
            }
            false
        }
    }
}
