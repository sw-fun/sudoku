//! Annotated strategy finders for the teaching mode: unlike the raw
//! ladder detectors, every finder returns the full pattern geometry and
//! a step-by-step explanation so the UI can teach the strategy.

mod annotations;
mod locked;
mod pairs;
mod singles;
mod x_wing;
mod xy_wing;

pub use annotations::{Annotation, Effect, Step, Strategy, UnitRef};
pub use locked::{claiming_all, pointing_all};
pub use pairs::{hidden_pairs, naked_pairs};
pub use singles::{hidden_singles, naked_singles};
pub use x_wing::x_wings;
pub use xy_wing::xy_wings;

use suduko_grid::{Board, CELL_COUNT, Cell};
use suduko_techniques::Candidates;

/// Candidate state for a board of shown values (0 = empty).
#[must_use]
pub fn candidates(shown: &[u8; CELL_COUNT]) -> Candidates {
    let mut board = Board::new();
    for (idx, &digit) in shown.iter().enumerate() {
        if digit != 0 {
            board.set(idx, Cell::Value(digit));
        }
    }
    Candidates::from_board(&board)
}

/// All 27 units: rows, then columns, then blocks.
#[must_use]
pub fn all_units() -> [UnitRef; 27] {
    core::array::from_fn(|k| match k {
        k if k < 9 => UnitRef::Row(k),
        k if k < 18 => UnitRef::Col(k - 9),
        k => UnitRef::Block(k - 18),
    })
}

/// Every applicable strategy for the board, ladder-ordered and deduped
/// by (strategy, pattern, digits, effect).
#[must_use]
pub fn find_all(shown: &[u8; CELL_COUNT]) -> Vec<Annotation> {
    let mut all = Vec::new();
    all.extend(singles::naked_singles(shown));
    all.extend(singles::hidden_singles(shown));
    all.extend(locked::pointing_all(shown));
    all.extend(locked::claiming_all(shown));
    all.extend(pairs::naked_pairs(shown));
    all.extend(pairs::hidden_pairs(shown));
    all.extend(x_wing::x_wings(shown));
    all.extend(xy_wing::xy_wings(shown));
    all.sort_by_key(|a| a.strategy.rung());
    let mut seen = std::collections::HashSet::new();
    all.into_iter()
        .filter(|a| {
            seen.insert((
                a.strategy,
                a.pattern.clone(),
                a.digits.clone(),
                a.effect.clone(),
            ))
        })
        .collect()
}
