//! Trial fallback: when the taught techniques are exhausted, teach a
//! solution-confirmed placement on the most constrained cell.

use super::Game;
use suduko_grid::CELL_COUNT;
use suduko_tutor::{Annotation, Candidates, Effect, Step, Strategy, cell_name};

/// A Trial annotation for the most constrained empty cell, or None
/// when the board is finished. The digit comes from the stored unique
/// solution, so the placement is guaranteed correct.
#[must_use]
pub fn trial_annotation(game: &Game, cands: &Candidates) -> Option<Annotation> {
    let idx = (0..CELL_COUNT)
        .filter(|&i| !cands.placed[i] && cands.masks[i].count_ones() >= 1)
        .min_by_key(|&i| cands.masks[i].count_ones())?;
    let digit = game.solution[idx];
    let options: Vec<u8> = (1..=9u8)
        .filter(|d| cands.masks[idx] & (1 << (d - 1)) != 0)
        .collect();
    Some(Annotation {
        strategy: Strategy::Trial,
        title: format!("Trial: place {digit} in {}", cell_name(idx)),
        digits: vec![digit],
        pattern: vec![idx],
        units: vec![],
        effect: Effect::Place { idx, digit },
        steps: trial_steps(idx, digit, &options),
    })
}

fn trial_steps(idx: usize, digit: u8, options: &[u8]) -> Vec<Step> {
    let option_text = options
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" or ");
    vec![
        Step {
            cells: vec![idx],
            units: vec![],
            digits: options.to_vec(),
            text: "The taught strategies are exhausted on this board - \
                       harder patterns (swordfish, chains) are beyond this \
                       tutor, so we reason it out instead."
                .to_string(),
        },
        Step {
            cells: vec![idx],
            units: vec![],
            digits: options.to_vec(),
            text: format!(
                "The most constrained cell is {}: only {option_text} \
                     remain possible there.",
                cell_name(idx)
            ),
        },
        Step {
            cells: vec![idx],
            units: vec![],
            digits: vec![digit],
            text: format!(
                "We place {digit} - the puzzle's unique solution \
                     confirms it - and new strategies open up from there."
            ),
        },
    ]
}
