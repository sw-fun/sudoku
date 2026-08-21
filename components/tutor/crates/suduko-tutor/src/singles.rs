//! Single annotations: the two cheapest rungs.

use crate::all_units;
use crate::annotations::{Annotation, Effect, Step, Strategy, UnitRef, cell_name};
use suduko_grid::{CELL_COUNT, peers_of};
use suduko_techniques::Candidates;

/// Every cell whose candidates collapsed to one digit.
#[must_use]
pub fn naked_singles(shown: &[u8; CELL_COUNT]) -> Vec<Annotation> {
    let cands = crate::candidates(shown);
    (0..CELL_COUNT)
        .filter(|&idx| shown[idx] == 0 && cands.masks[idx].is_power_of_two())
        .map(|idx| naked_one(&cands, idx))
        .collect()
}

fn naked_one(cands: &Candidates, idx: usize) -> Annotation {
    let digit = u8::try_from(cands.masks[idx].trailing_zeros() + 1).expect("single bit 1..=9");
    let row = UnitRef::Row(idx / 9);
    let col = UnitRef::Col(idx % 9);
    let block = UnitRef::Block((idx / 27) * 3 + (idx % 9) / 3);
    let blockers: Vec<usize> = peers_of(idx)
        .into_iter()
        .filter(|&p| cands.placed[p] && cands.masks[p] == 1 << (digit - 1))
        .collect();
    Annotation {
        strategy: Strategy::NakedSingle,
        title: format!("Naked Single: {digit} in {}", cell_name(idx)),
        digits: vec![digit],
        pattern: vec![idx],
        units: vec![row, col, block],
        effect: Effect::Place { idx, digit },
        steps: naked_steps(idx, digit, [row, col, block], &blockers),
    }
}

fn naked_steps(idx: usize, digit: u8, units: [UnitRef; 3], blockers: &[usize]) -> Vec<Step> {
    vec![
        Step {
            cells: vec![idx],
            units: units.to_vec(),
            digits: vec![],
            text: format!(
                "Every empty cell starts with all digits it cannot see; \
                 look at {} and collect the digits it sees.",
                cell_name(idx)
            ),
        },
        Step {
            cells: blockers.to_vec(),
            units: vec![],
            digits: vec![digit],
            text: format!("Its row, column, and block already contain every digit except {digit}."),
        },
        Step {
            cells: vec![idx],
            units: vec![],
            digits: vec![digit],
            text: format!(
                "Only {digit} is left, so {} must be {digit}.",
                cell_name(idx)
            ),
        },
    ]
}

/// Every digit confined to one cell of a unit.
#[must_use]
pub fn hidden_singles(shown: &[u8; CELL_COUNT]) -> Vec<Annotation> {
    let cands = crate::candidates(shown);
    let mut out = Vec::new();
    for unit in all_units() {
        for digit in 1u8..=9 {
            let bit = 1 << (digit - 1);
            let spots: Vec<usize> = unit
                .cells()
                .into_iter()
                .filter(|&i| shown[i] == 0 && cands.masks[i] & bit != 0)
                .collect();
            if spots.len() == 1 {
                out.push(hidden_one(&cands, spots[0], digit, unit));
            }
        }
    }
    dedupe_placements(out)
}

fn hidden_one(cands: &Candidates, idx: usize, digit: u8, unit: UnitRef) -> Annotation {
    Annotation {
        strategy: Strategy::HiddenSingle,
        title: format!(
            "Hidden Single: {digit} in {} of {}",
            cell_name(idx),
            unit.label()
        ),
        digits: vec![digit],
        pattern: vec![idx],
        units: vec![unit],
        effect: Effect::Place { idx, digit },
        steps: hidden_steps(cands, idx, digit, unit),
    }
}

fn hidden_steps(cands: &Candidates, idx: usize, digit: u8, unit: UnitRef) -> Vec<Step> {
    let others: Vec<usize> = unit
        .cells()
        .into_iter()
        .filter(|&i| i != idx && cands.masks[i] & (1 << (digit - 1)) == 0)
        .collect();
    vec![
        Step {
            cells: unit.cells().into(),
            units: vec![unit],
            digits: vec![digit],
            text: format!(
                "Where can {digit} go in {}? Mark every cell that could hold it.",
                unit.label()
            ),
        },
        Step {
            cells: others,
            units: vec![unit],
            digits: vec![digit],
            text: format!(
                "Every other cell of {} is ruled out: it sees a {digit} already placed.",
                unit.label()
            ),
        },
        Step {
            cells: vec![idx],
            units: vec![unit],
            digits: vec![digit],
            text: format!(
                "{} is the only cell left, so it must be {digit}.",
                cell_name(idx)
            ),
        },
    ]
}

fn dedupe_placements(anns: Vec<Annotation>) -> Vec<Annotation> {
    // The same (idx, digit) surfaces once per unit; keep the first.
    let mut seen = std::collections::HashSet::new();
    anns.into_iter()
        .filter(|a| {
            let Effect::Place { idx, digit } = a.effect else {
                return false;
            };
            seen.insert((idx, digit))
        })
        .collect()
}
