//! Digit completion: a digit is complete once all nine of its
//! instances are placed correctly.

use super::Game;
use suduko_grid::CELL_COUNT;

/// True when `digit` is correctly shown in all nine of its cells.
/// Wrong placements never count; erasing reopens the digit.
pub fn digit_complete(game: &Game, digit: u8) -> bool {
    (0..CELL_COUNT)
        .filter(|&idx| game.shown(idx) == digit && !game.is_wrong(idx))
        .count()
        == 9
}
