//! Public solve and count entry points.

use super::search::search;
use super::state::State;
use suduko_grid::{Board, CELL_COUNT, Cell, puzzle::first_conflict};

/// Solves `board` with deterministic backtracking (ascending digit order,
/// most-constrained-cell selection). Returns the first solution found.
pub fn solve(board: &Board) -> Option<Board> {
    if first_conflict(board).is_some() {
        return None;
    }
    let mut state = State::from_board(board);
    let (mut found, mut solution) = (0, None);
    search(&mut state, 1, &mut found, &mut solution);
    solution.map(Board::from_cells)
}

/// Counts solutions of `board`, stopping once `cap` are found. A result of
/// `cap` means "at least cap solutions", so cap 2 decides uniqueness.
#[must_use]
pub fn count_solutions(board: &Board, cap: usize) -> usize {
    if cap == 0 || first_conflict(board).is_some() {
        return 0;
    }
    let mut state = State::from_board(board);
    let (mut found, mut solution) = (0, None::<[Cell; CELL_COUNT]>);
    search(&mut state, cap, &mut found, &mut solution);
    found
}
