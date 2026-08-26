//! Input effects and highlight rules on the game state.

use super::Game;
use suduko_grid::{CELL_COUNT, peers_of};

/// Result of a set_value attempt.
pub struct Outcome {
    /// The input targeted a given cell and was discarded.
    pub ignored: bool,
    /// The entered digit disagrees with the solution.
    pub wrong: bool,
}

/// Enters `digit` at `idx`. Wrong entries bump `bad_inputs` every time
/// and never disturb pencil notes; given cells ignore input; winning
/// the last cell sets `won`. A correct placement prunes that digit's
/// note from all twenty peers (wrong guesses leave notes untouched,
/// so correcting or erasing them needs no restore).
pub fn set_value(game: &mut Game, idx: usize, digit: u8) -> Outcome {
    if game.is_given(idx) {
        return Outcome {
            ignored: true,
            wrong: false,
        };
    }
    let wrong = digit != game.solution[idx];
    if wrong {
        game.bad_inputs += 1;
    }
    game.user[idx] = digit;
    if !wrong {
        for p in peers_of(idx) {
            game.user_marks[p] &= !(1 << (digit - 1));
        }
    }
    game.won = game.is_won();
    Outcome {
        ignored: false,
        wrong,
    }
}

/// Clears the user value at `idx`; clues survive.
pub fn erase(game: &mut Game, idx: usize) {
    if !game.is_given(idx) {
        game.user[idx] = 0;
        game.won = game.is_won();
    }
}

/// Enters `digit` into the selected cell, if one is selected. With
/// the pencil toggle on it toggles a note (empty cells, legal digits
/// only); otherwise it places a value (pruning peers' notes) and
/// closes the cell keypad.
pub fn entry(game: &mut Game, digit: u8) {
    let Some(&sel) = game.selected.as_ref() else {
        return;
    };
    if game.pencil {
        if game.shown(sel) != 0 {
            return;
        }
        let base = suduko_tutor::candidates_with(&game.shown_values(), &[]);
        if base.masks[sel] & (1 << (digit - 1)) != 0 {
            game.user_marks[sel] ^= 1 << (digit - 1);
        }
        return;
    }
    set_value(game, sel, digit);
    game.keypad_open = false;
}

/// Erases the selected cell, if one is selected: with pencil on it
/// clears the cell's notes, otherwise the value. Value erase closes
/// the cell keypad.
pub fn clear_selected(game: &mut Game) {
    let Some(&sel) = game.selected.as_ref() else {
        return;
    };
    if game.pencil {
        game.user_marks[sel] = 0;
    } else {
        erase(game, sel);
        game.keypad_open = false;
    }
}

impl Game {
    /// User-entered marks for each cell, as sorted digits.
    #[must_use]
    pub fn user_marks_view(&self) -> [Vec<u8>; CELL_COUNT] {
        core::array::from_fn(|idx| {
            (1..=9u8)
                .filter(|d| self.user_marks[idx] & (1 << (d - 1)) != 0)
                .collect()
        })
    }
}

/// True when the cell keypad should render: explicitly open, a
/// non-given selection, and none of the modes where a value popup
/// would be wrong or distracting. Button rules live in suduko-uikit.
pub fn keypad_visible(game: &Game) -> bool {
    game.keypad_open
        && game.selected.is_some_and(|sel| !game.is_given(sel))
        && !game.pencil
        && !game.show_me
        && !game.won
}

/// Highlight set for the selected cell: its 20 peers when empty, else
/// every other correctly-shown cell with the same digit (bad guesses
/// never highlight).
pub fn highlight_set(game: &Game) -> Vec<usize> {
    let Some(&sel) = game.selected.as_ref() else {
        return Vec::new();
    };
    if game.shown(sel) == 0 {
        return peers_of(sel).to_vec();
    }
    (0..CELL_COUNT)
        .filter(|&idx| idx != sel && game.shown(idx) == game.shown(sel) && !game.is_wrong(idx))
        .collect()
}
