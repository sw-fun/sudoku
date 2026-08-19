//! Highlight rules for the selected cell.

use super::Game;
use suduko_grid::{CELL_COUNT, peers_of};

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
