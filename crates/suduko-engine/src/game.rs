use crate::grid::{Board, CELL_COUNT, Cell};
use crate::validate::first_conflict;

#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleError {
    IncompleteSolution,
    InconsistentSolution,
    ClueMismatch,
}

pub struct Puzzle {
    clues: Board,
    solution: Board,
}

impl Puzzle {
    /// Builds a puzzle from `clues` (empty cells where the player fills in)
    /// and a complete, consistent `solution` that agrees with every clue.
    pub fn new(clues: Board, solution: Board) -> Result<Self, PuzzleError> {
        if (0..CELL_COUNT).any(|i| solution.get(i).is_empty()) {
            return Err(PuzzleError::IncompleteSolution);
        }
        if first_conflict(&solution).is_some() {
            return Err(PuzzleError::InconsistentSolution);
        }
        if (0..CELL_COUNT).any(|i| clues.get(i) != Cell::Empty && clues.get(i) != solution.get(i)) {
            return Err(PuzzleError::ClueMismatch);
        }
        Ok(Puzzle { clues, solution })
    }

    pub fn solution(&self) -> &Board {
        &self.solution
    }

    pub fn clue_count(&self) -> usize {
        (0..CELL_COUNT)
            .filter(|&i| !self.clues.get(i).is_empty())
            .count()
    }
}
