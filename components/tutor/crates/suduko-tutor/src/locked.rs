//! Locked-candidate annotations: pointing and claiming.

use crate::all_units;
use crate::annotations::{Annotation, Effect, Step, Strategy, UnitRef, name_list};
use suduko_techniques::Candidates;

/// Pointing: a digit confined to one line inside a block leaves that
/// line everywhere else.
#[must_use]
pub fn pointing_all(cands: &Candidates) -> Vec<Annotation> {
    let mut out = Vec::new();
    for unit in all_units() {
        let UnitRef::Block(block) = unit else {
            continue;
        };
        for digit in 1u8..=9 {
            let bit = 1 << (digit - 1);
            let cells: Vec<usize> = unit
                .cells()
                .into_iter()
                .filter(|&i| !cands.placed[i] && cands.masks[i] & bit != 0)
                .collect();
            if cells.len() < 2 {
                continue;
            }
            for line in [line_of(&cells, true), line_of(&cells, false)] {
                let Some(line) = line else { continue };
                let removals: Vec<(usize, u8)> = line
                    .cells()
                    .into_iter()
                    .filter(|&i| {
                        !unit.cells().contains(&i) && !cands.placed[i] && cands.masks[i] & bit != 0
                    })
                    .map(|i| (i, digit))
                    .collect();
                if !removals.is_empty() {
                    out.push(pointing_one(digit, block, line, &cells, &removals));
                }
            }
        }
    }
    out
}

fn line_of(cells: &[usize], rows: bool) -> Option<UnitRef> {
    let key = |i: usize| if rows { i / 9 } else { i % 9 };
    let first = key(cells[0]);
    cells.iter().all(|&i| key(i) == first).then_some(if rows {
        UnitRef::Row(first)
    } else {
        UnitRef::Col(first)
    })
}

fn pointing_one(
    digit: u8,
    block: usize,
    line: UnitRef,
    cells: &[usize],
    removals: &[(usize, u8)],
) -> Annotation {
    Annotation {
        strategy: Strategy::Pointing,
        title: format!(
            "Pointing: the {digit}s of block {} point along {}",
            block + 1,
            line.label()
        ),
        digits: vec![digit],
        pattern: cells.to_vec(),
        units: vec![UnitRef::Block(block), line],
        effect: Effect::Eliminate {
            removals: removals.to_vec(),
        },
        steps: pointing_steps(digit, block, line, cells, removals),
    }
}

fn pointing_steps(
    digit: u8,
    block: usize,
    line: UnitRef,
    cells: &[usize],
    removals: &[(usize, u8)],
) -> Vec<Step> {
    let targets: Vec<usize> = removals.iter().map(|&(i, _)| i).collect();
    vec![
        Step {
            cells: cells.to_vec(),
            units: vec![UnitRef::Block(block)],
            digits: vec![digit],
            text: format!(
                "Inside block {}, only {} can hold the {digit} - and all \
                 of those cells sit in {}.",
                block + 1,
                name_list(cells),
                line.label()
            ),
        },
        Step {
            cells: cells.to_vec(),
            units: vec![line],
            digits: vec![digit],
            text: format!(
                "Wherever the {digit} of block {} goes, it lands in {}, so \
                 the {digit} is already used up there.",
                block + 1,
                line.label()
            ),
        },
        Step {
            cells: targets,
            units: vec![line],
            digits: vec![digit],
            text: format!(
                "Every other cell of {} can therefore drop the {digit} - \
                 it will never be {digit}.",
                line.label()
            ),
        },
    ]
}

/// Claiming: a digit confined to one block inside a line leaves that
/// block everywhere else.
#[must_use]
pub fn claiming_all(cands: &Candidates) -> Vec<Annotation> {
    let mut out = Vec::new();
    for line in all_units() {
        let (UnitRef::Row(_) | UnitRef::Col(_)) = line else {
            continue;
        };
        for digit in 1u8..=9 {
            let bit = 1 << (digit - 1);
            let cells: Vec<usize> = line
                .cells()
                .into_iter()
                .filter(|&i| !cands.placed[i] && cands.masks[i] & bit != 0)
                .collect();
            let blocks: Vec<usize> = cells.iter().map(|&i| (i / 27) * 3 + (i % 9) / 3).collect();
            if cells.len() < 2 || !blocks.iter().all(|&b| b == blocks[0]) {
                continue;
            }
            let block = UnitRef::Block(blocks[0]);
            let removals: Vec<(usize, u8)> = block
                .cells()
                .into_iter()
                .filter(|&i| {
                    !line.cells().contains(&i) && !cands.placed[i] && cands.masks[i] & bit != 0
                })
                .map(|i| (i, digit))
                .collect();
            if !removals.is_empty() {
                out.push(claiming_one(digit, line, block, &cells, &removals));
            }
        }
    }
    out
}

fn claiming_one(
    digit: u8,
    line: UnitRef,
    block: UnitRef,
    cells: &[usize],
    removals: &[(usize, u8)],
) -> Annotation {
    Annotation {
        strategy: Strategy::Claiming,
        title: format!(
            "Claiming: the {digit}s of {} claim {}",
            line.label(),
            block.label()
        ),
        digits: vec![digit],
        pattern: cells.to_vec(),
        units: vec![line, block],
        effect: Effect::Eliminate {
            removals: removals.to_vec(),
        },
        steps: claiming_steps(digit, line, block, cells, removals),
    }
}

fn claiming_steps(
    digit: u8,
    line: UnitRef,
    block: UnitRef,
    cells: &[usize],
    removals: &[(usize, u8)],
) -> Vec<Step> {
    let targets: Vec<usize> = removals.iter().map(|&(i, _)| i).collect();
    vec![
        Step {
            cells: cells.to_vec(),
            units: vec![line],
            digits: vec![digit],
            text: format!(
                "In {}, only {} can hold the {digit} - and all of those \
                 cells lie inside {}.",
                line.label(),
                name_list(cells),
                block.label()
            ),
        },
        Step {
            cells: cells.to_vec(),
            units: vec![block],
            digits: vec![digit],
            text: format!(
                "The {digit} of {} must be in {}, so the {digit} is spoken \
                 for in that whole block.",
                line.label(),
                block.label()
            ),
        },
        Step {
            cells: targets,
            units: vec![block],
            digits: vec![digit],
            text: format!("The rest of {} can drop the {digit}.", block.label()),
        },
    ]
}
