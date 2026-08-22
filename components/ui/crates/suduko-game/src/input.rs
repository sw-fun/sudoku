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

/// Enters `digit` at `idx`. Wrong entries bump `bad_inputs` every time;
/// given cells ignore input; winning the last cell sets `won`.
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

/// Enters `digit` into the selected cell, if one is selected. In
/// notes mode the digit toggles a pencil-mark candidate instead of
/// placing (empty cells only; computed candidates are never restored
/// beyond what the rules allow).
pub fn entry(game: &mut Game, digit: u8) {
    let Some(&sel) = game.selected.as_ref() else {
        return;
    };
    if game.notes_mode {
        toggle_mark(game, sel, digit);
        return;
    }
    set_value(game, sel, digit);
}

/// Erases the selected cell, if one is selected. In notes mode it
/// restores the selected cell's computed candidates instead.
pub fn clear_selected(game: &mut Game) {
    let Some(&sel) = game.selected.as_ref() else {
        return;
    };
    if game.notes_mode {
        game.eliminated.retain(|&(i, _)| i != sel);
        return;
    }
    erase(game, sel);
}

/// Toggles one user candidate mark in the selected empty cell:
/// removals join the elimination layer, repeat-typing removes the
/// removal (marks that the rules exclude are ignored).
fn toggle_mark(game: &mut Game, idx: usize, digit: u8) {
    if game.shown(idx) != 0 {
        return;
    }
    let base = suduko_tutor::candidates_with(&game.shown_values(), &[]);
    if base.masks[idx] & (1 << (digit - 1)) == 0 {
        return;
    }
    let entry = (idx, digit);
    if game.eliminated.contains(&entry) {
        game.eliminated.retain(|&e| e != entry);
    } else {
        game.eliminated.push(entry);
    }
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
    same_value_cells(game, sel, game.shown(sel))
}

fn same_value_cells(game: &Game, sel: usize, value: u8) -> Vec<usize> {
    (0..CELL_COUNT)
        .filter(|&idx| idx != sel && game.shown(idx) == value && !game.is_wrong(idx))
        .collect()
}
