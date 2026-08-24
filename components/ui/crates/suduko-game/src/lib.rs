//! Pure game state types and queries. Construction lives in `build`,
//! input effects and highlight rules in `input`, digit
//! completion here, learn mode in `teaching`/`learn`, show-me solving
//! in `showme` with the trial fallback in `trial`.

pub mod showme;

mod build;
mod input;
mod learn;
mod teaching;
mod trial;

pub use build::{Save, from_strings, restore, save};
pub use input::{Outcome, clear_selected, entry, erase, highlight_set, keypad_visible, set_value};
pub use learn::{NoteOp, Pulse, StepView, note};
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
    /// Show-me solver mode active.
    pub show_me: bool,
    /// Show-me advances automatically on the timer tick.
    pub show_me_auto: bool,
    /// Extra ticks to wait between auto-advances.
    pub show_me_delay_ticks: u32,
    /// Ticks waited since the last auto-advance.
    pub show_me_wait: u32,
    /// Solver eliminations: candidates removed but not yet placed.
    pub eliminated: Vec<(usize, u8)>,
    /// Notes mode: digit entry toggles pencil marks instead of
    /// placing values.
    pub notes_mode: bool,
    /// Cell keypad popup is open on the selected cell.
    pub keypad_open: bool,
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
        if self.selected == Some(idx) {
            // Retapping the selected cell toggles the keypad popup.
            self.keypad_open = !self.keypad_open;
        } else {
            self.selected = Some(idx);
            self.keypad_open = true;
        }
    }

    /// True when every cell is filled and agrees with the solution.
    pub fn is_won(&self) -> bool {
        (0..CELL_COUNT).all(|idx| self.shown(idx) == self.solution[idx])
    }
}

/// True when `digit` is correctly shown in all nine of its cells.
/// Wrong placements never count; erasing reopens the digit.
pub fn digit_complete(game: &Game, digit: u8) -> bool {
    (0..CELL_COUNT)
        .filter(|&idx| game.shown(idx) == digit && !game.is_wrong(idx))
        .count()
        == 9
}
