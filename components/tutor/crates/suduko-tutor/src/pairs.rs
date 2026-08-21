//! Pair annotations: naked and hidden.

use crate::all_units;
use crate::annotations::{Annotation, Effect, Step, Strategy, UnitRef, cell_name, name_list};
use suduko_grid::CELL_COUNT;

/// Naked pairs: two cells of a unit sharing exactly two candidates.
#[must_use]
pub fn naked_pairs(shown: &[u8; CELL_COUNT]) -> Vec<Annotation> {
    let cands = crate::candidates(shown);
    let mut out = Vec::new();
    for unit in all_units() {
        let two: Vec<usize> = unit
            .cells()
            .into_iter()
            .filter(|&i| !cands.placed[i] && cands.masks[i].count_ones() == 2)
            .collect();
        for window in two.windows(2) {
            let (a, b) = (window[0], window[1]);
            if cands.masks[a] != cands.masks[b] {
                continue;
            }
            let mask = cands.masks[a];
            let removals: Vec<(usize, u8)> = unit
                .cells()
                .into_iter()
                .filter(|&i| i != a && i != b && !cands.placed[i] && cands.masks[i] & mask != 0)
                .flat_map(|i| {
                    mask_digits(mask & cands.masks[i])
                        .into_iter()
                        .map(move |d| (i, d))
                })
                .collect();
            if !removals.is_empty() {
                out.push(naked_one(unit, a, b, mask, &removals));
            }
        }
    }
    out
}

fn naked_one(unit: UnitRef, a: usize, b: usize, mask: u16, removals: &[(usize, u8)]) -> Annotation {
    let digits = mask_digits(mask);
    Annotation {
        strategy: Strategy::NakedPair,
        title: format!(
            "Naked Pair: {{ {} }} at {} in {}",
            digits
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
            name_list(&[a, b]),
            unit.label()
        ),
        digits: digits.clone(),
        pattern: vec![a, b],
        units: vec![unit],
        effect: Effect::Eliminate {
            removals: removals.to_vec(),
        },
        steps: naked_steps(unit, a, b, &digits, removals),
    }
}

fn naked_steps(
    unit: UnitRef,
    a: usize,
    b: usize,
    digits: &[u8],
    removals: &[(usize, u8)],
) -> Vec<Step> {
    let targets: Vec<usize> = removals.iter().map(|&(i, _)| i).collect();
    let digit_text = digits
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" or ");
    vec![
        Step {
            cells: vec![a, b],
            units: vec![unit],
            digits: digits.to_vec(),
            text: format!(
                "{} and {} both hold exactly {digit_text} as candidates.",
                cell_name(a),
                cell_name(b)
            ),
        },
        Step {
            cells: vec![a, b],
            units: vec![unit],
            digits: digits.to_vec(),
            text: format!(
                "One of them must be {} and the other takes the remaining \
                 digit - between them the pair uses up both.",
                digits[0]
            ),
        },
        Step {
            cells: targets,
            units: vec![unit],
            digits: digits.to_vec(),
            text: format!(
                "No third cell of {} can be {digit_text} anymore, so those \
                 candidates go.",
                unit.label()
            ),
        },
    ]
}

/// Hidden pairs: two digits of a unit confined to the same two cells.
#[must_use]
pub fn hidden_pairs(shown: &[u8; CELL_COUNT]) -> Vec<Annotation> {
    let cands = crate::candidates(shown);
    let mut out = Vec::new();
    for unit in all_units() {
        for da in 1u8..=9 {
            for db in (da + 1)..=9 {
                let mask = (1 << (da - 1)) | (1 << (db - 1));
                let spots: Vec<usize> = unit
                    .cells()
                    .into_iter()
                    .filter(|&i| !cands.placed[i] && cands.masks[i] & mask != 0)
                    .collect();
                let confined = spots.len() == 2
                    && [da, db].into_iter().all(|d| {
                        let bit = 1 << (d - 1);
                        unit.cells()
                            .into_iter()
                            .filter(|&i| cands.masks[i] & bit != 0)
                            .eq(spots.iter().copied())
                    });
                if !confined {
                    continue;
                }
                let removals: Vec<(usize, u8)> = spots
                    .iter()
                    .flat_map(|&i| {
                        (1..=9u8)
                            .filter(move |d| {
                                mask & (1 << (d - 1)) == 0 && cands.masks[i] & (1 << (d - 1)) != 0
                            })
                            .map(move |d| (i, d))
                    })
                    .collect();
                if !removals.is_empty() {
                    out.push(hidden_one(unit, &[da, db], &spots, &removals));
                }
            }
        }
    }
    out
}

fn hidden_one(
    unit: UnitRef,
    digits: &[u8],
    spots: &[usize],
    removals: &[(usize, u8)],
) -> Annotation {
    Annotation {
        strategy: Strategy::HiddenPair,
        title: format!(
            "Hidden Pair: {} locked into {} in {}",
            digits
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" and "),
            name_list(spots),
            unit.label()
        ),
        digits: digits.to_vec(),
        pattern: spots.to_vec(),
        units: vec![unit],
        effect: Effect::Eliminate {
            removals: removals.to_vec(),
        },
        steps: hidden_steps(unit, digits, spots, removals),
    }
}

fn hidden_steps(
    unit: UnitRef,
    digits: &[u8],
    spots: &[usize],
    removals: &[(usize, u8)],
) -> Vec<Step> {
    let targets: Vec<usize> = removals.iter().map(|&(i, _)| i).collect();
    let digit_text = digits
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" and ");
    vec![
        Step {
            cells: spots.to_vec(),
            units: vec![unit],
            digits: digits.to_vec(),
            text: format!(
                "In {}, both {digit_text} can only land in {}.",
                unit.label(),
                name_list(spots)
            ),
        },
        Step {
            cells: spots.to_vec(),
            units: vec![unit],
            digits: digits.to_vec(),
            text: format!(
                "Whichever way they split, those two cells are used up by \
                 the {digit_text}."
            ),
        },
        Step {
            cells: targets,
            units: vec![unit],
            digits: digits.to_vec(),
            text: format!(
                "So every other candidate in those two cells can be erased \
                 - they will never be anything but {digit_text}."
            ),
        },
    ]
}

fn mask_digits(mask: u16) -> Vec<u8> {
    (1..=9u8).filter(|d| mask & (1 << (d - 1)) != 0).collect()
}
