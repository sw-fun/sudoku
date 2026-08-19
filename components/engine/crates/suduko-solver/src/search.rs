//! Depth-first search shared by solve and count.

use super::state::{Pick, State, pick_cell};
use suduko_grid::{CELL_COUNT, Cell};

/// Explores placements; records the first solution into `solution`;
/// returns true when `limit` solutions have been found (stop signal).
pub(crate) fn search(
    state: &mut State,
    limit: usize,
    found: &mut usize,
    solution: &mut Option<[Cell; CELL_COUNT]>,
) -> bool {
    match pick_cell(state) {
        Pick::Solved => {
            *found += 1;
            if solution.is_none() {
                *solution = Some(state.cells);
            }
            *found >= limit
        }
        Pick::DeadEnd => false,
        Pick::Cell(idx) => try_digits(state, idx, limit, found, solution),
    }
}

fn try_digits(
    state: &mut State,
    idx: usize,
    limit: usize,
    found: &mut usize,
    solution: &mut Option<[Cell; CELL_COUNT]>,
) -> bool {
    for digit in 1u8..=9 {
        if state.candidates(idx) & (1 << (digit - 1)) != 0 {
            state.place(idx, digit);
            if search(state, limit, found, solution) {
                return true;
            }
            state.unplace(idx, digit);
        }
    }
    false
}
