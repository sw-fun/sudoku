//! Input effects: entering and erasing values.

use super::Game;

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

/// Enters `digit` into the selected cell, if one is selected.
pub fn entry(game: &mut Game, digit: u8) {
    if let Some(&sel) = game.selected.as_ref() {
        set_value(game, sel, digit);
    }
}

/// Erases the selected cell, if one is selected.
pub fn clear_selected(game: &mut Game) {
    if let Some(&sel) = game.selected.as_ref() {
        erase(game, sel);
    }
}
