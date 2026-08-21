//! Pure game state types and queries. Construction lives in `build`,
//! input effects in `input`, highlight rules in `highlights`, digit
//! completion in `complete`, learn mode in `teaching` and `learn`.

mod build;
mod complete;
mod highlights;
mod input;
mod learn;
mod teaching;

pub use build::from_strings;
pub use complete::digit_complete;
pub use highlights::highlight_set;
pub use input::{Outcome, erase, set_value};
pub use learn::pencil_marks;
pub use teaching::Teaching;

use suduko_grid::CELL_COUNT;

/// Why puzzle construction from strings failed.
#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    BadStrings,
}

#[derive(Clone)]
pub struct Game {
    /// Clue digits (0 = player cell).
    pub clues: [u8; CELL_COUNT],
    /// The unique solution digits.
    pub solution: [u8; CELL_COUNT],
    /// Player-entered digits (0 = empty).
    pub user: [u8; CELL_COUNT],
    /// Currently highlighted (selected) cell.
    pub selected: Option<usize>,
    /// Count of wrong entries so far (every wrong entry bumps it).
    pub bad_inputs: u32,
    /// True once every cell is filled correctly.
    pub won: bool,
    /// Seconds elapsed on this board.
    pub elapsed_secs: u32,
    /// Learn-mode (teaching) state.
    pub teaching: Teaching,
}

impl Game {
    /// Builds a fresh, empty game from an engine puzzle.
    pub fn from_puzzle(puzzle: &suduko_grid::Puzzle) -> Self {
        build::from_puzzle(puzzle)
    }

    /// The digit currently shown: the clue, else the user value, else 0.
    pub fn shown(&self, idx: usize) -> u8 {
        if self.clues[idx] != 0 {
            self.clues[idx]
        } else {
            self.user[idx]
        }
    }

    pub fn is_given(&self, idx: usize) -> bool {
        self.clues[idx] != 0
    }

    pub fn is_wrong(&self, idx: usize) -> bool {
        !self.is_given(idx) && self.user[idx] != 0 && self.user[idx] != self.solution[idx]
    }

    pub fn select(&mut self, idx: usize) {
        self.selected = Some(idx);
    }

    /// True when every cell is filled and agrees with the solution.
    pub fn is_won(&self) -> bool {
        (0..CELL_COUNT).all(|idx| self.shown(idx) == self.solution[idx])
    }
}
