pub(crate) mod search;
pub(crate) mod state;

use crate::grid::{Board, CELL_COUNT};
use crate::validate::first_conflict;
use state::State;

/// Solves `board` with deterministic backtracking (ascending digit order,
/// most-constrained-cell selection). Returns the first solution found.
pub fn solve(board: &Board) -> Option<Board> {
    if first_conflict(board).is_some() {
        return None;
    }
    let mut state = State::from_board(board);
    let mut found = 0;
    let mut solution = None;
    search::search(&mut state, 1, &mut found, &mut solution);
    solution.map(Board::from_cells)
}

/// Counts solutions of `board`, stopping once `cap` are found. Returns the
/// number found, so a result of `cap` means "at least cap solutions".
pub fn count_solutions(board: &Board, cap: usize) -> usize {
    if cap == 0 || first_conflict(board).is_some() {
        return 0;
    }
    let mut state = State::from_board(board);
    let mut found = 0;
    let mut solution = None;
    search::search(&mut state, cap, &mut found, &mut solution);
    found
}

/// True when every cell is filled and no row, column, or block repeats a digit.
pub fn is_solved(board: &Board) -> bool {
    (0..CELL_COUNT).all(|i| !board.get(i).is_empty()) && first_conflict(board).is_none()
}
