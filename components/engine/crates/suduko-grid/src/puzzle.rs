//! The puzzle type and fail-closed construction.

use crate::board::{Board, CELL_COUNT, Cell};
use crate::coords::{block_of, col_of, row_of};

#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleError {
    IncompleteSolution,
    InconsistentSolution,
    ClueMismatch,
}

/// A clues board plus its complete, consistent solution. Construction
/// validates the solution and that every clue agrees with it.
pub struct Puzzle {
    clues: Board,
    solution: Board,
}

impl Puzzle {
    /// Builds a puzzle from `clues` (empty cells where the player fills
    /// in) and a complete, consistent `solution` that agrees with every
    /// clue.
    ///
    /// # Errors
    ///
    /// Returns `IncompleteSolution`, `InconsistentSolution`, or
    /// `ClueMismatch` when validation fails; construction is fail-closed.
    pub fn new(clues: Board, solution: Board) -> Result<Self, PuzzleError> {
        if (0..CELL_COUNT).any(|i| solution.get(i).is_empty()) {
            return Err(PuzzleError::IncompleteSolution);
        }
        if first_conflict(&solution).is_some() {
            return Err(PuzzleError::InconsistentSolution);
        }
        if (0..CELL_COUNT).any(|i| !clues.get(i).is_empty() && clues.get(i) != solution.get(i)) {
            return Err(PuzzleError::ClueMismatch);
        }
        Ok(Puzzle { clues, solution })
    }

    #[must_use]
    pub fn solution(&self) -> &Board {
        &self.solution
    }

    #[must_use]
    pub fn clues(&self) -> &Board {
        &self.clues
    }

    #[must_use]
    pub fn clue_count(&self) -> usize {
        (0..CELL_COUNT)
            .filter(|&i| !self.clues.get(i).is_empty())
            .count()
    }
}

/// First row-major cell whose value duplicates one already seen in its row,
/// column, or block, as `(index, value)`; `None` when consistent so far.
#[must_use]
pub fn first_conflict(board: &Board) -> Option<(usize, u8)> {
    let mut row_seen = [[false; 9]; 9];
    let mut col_seen = [[false; 9]; 9];
    let mut block_seen = [[false; 9]; 9];
    for idx in 0..CELL_COUNT {
        if let Cell::Value(v) = board.get(idx) {
            let (r, c, b) = (row_of(idx), col_of(idx), block_of(idx));
            let d = (v - 1) as usize;
            if row_seen[r][d] || col_seen[c][d] || block_seen[b][d] {
                return Some((idx, v));
            }
            row_seen[r][d] = true;
            col_seen[c][d] = true;
            block_seen[b][d] = true;
        }
    }
    None
}
