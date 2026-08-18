use super::state::State;
use crate::grid::{CELL_COUNT, Cell};

pub enum Pick {
    Solved,
    DeadEnd,
    Cell(usize),
}

/// Depth-first search with most-constrained-cell selection. Records the first
/// solution found into `solution`; returns true when `limit` solutions have
/// been found and the caller should stop.
pub fn search(
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
        Pick::Cell(idx) => {
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
    }
}

fn pick_cell(state: &State) -> Pick {
    let mut best: Option<(usize, usize)> = None;
    for idx in 0..CELL_COUNT {
        if !state.cells[idx].is_empty() {
            continue;
        }
        let count = state.candidates(idx).count_ones() as usize;
        if count == 0 {
            return Pick::DeadEnd;
        }
        if best.is_none_or(|(_, best_count)| count < best_count) {
            best = Some((idx, count));
        }
    }
    match best {
        None => Pick::Solved,
        Some((idx, _)) => Pick::Cell(idx),
    }
}
