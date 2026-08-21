//! Game-to-tutor glue: shown values, learn-panel toggling, and
//! teaching-only pencil marks.

use super::Game;
use suduko_grid::CELL_COUNT;

impl Game {
    /// The shown values as tutor input (clues then user entries).
    #[must_use]
    pub fn shown_values(&self) -> [u8; CELL_COUNT] {
        core::array::from_fn(|idx| self.shown(idx))
    }

    /// Opens the learn panel with strategies for the current board.
    pub fn open_learn(&mut self) {
        self.teaching.open(&self.shown_values());
    }

    /// Closes the learn panel and clears the walkthrough.
    pub fn close_learn(&mut self) {
        self.teaching.close();
    }

    /// Toggles the learn panel for the current board.
    pub fn toggle_learn(&mut self) {
        if self.teaching.panel_open {
            self.close_learn();
        } else {
            self.open_learn();
        }
    }

    /// Pencil marks for the current board (empty cells only).
    #[must_use]
    pub fn pencil_marks(&self) -> [Vec<u8>; CELL_COUNT] {
        pencil_marks(&self.shown_values())
    }
}

/// Pencil marks for a board: the candidate digits of each empty cell.
#[must_use]
pub fn pencil_marks(shown: &[u8; CELL_COUNT]) -> [Vec<u8>; CELL_COUNT] {
    let cands = suduko_tutor::candidates(shown);
    core::array::from_fn(|idx| {
        if shown[idx] == 0 {
            (1..=9u8)
                .filter(|d| cands.masks[idx] & (1 << (d - 1)) != 0)
                .collect()
        } else {
            Vec::new()
        }
    })
}
