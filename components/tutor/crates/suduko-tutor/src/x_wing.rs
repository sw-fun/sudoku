//! X-Wing: one digit, two parallel base lines, two crossing lines.

use crate::annotations::{Annotation, Effect, Step, Strategy, UnitRef};
use suduko_grid::CELL_COUNT;
use suduko_techniques::Candidates;

/// Every X-Wing (basic fish of size 2) on the board.
#[must_use]
pub fn x_wings(cands: &Candidates) -> Vec<Annotation> {
    let mut out = Vec::new();
    for digit in 1u8..=9 {
        let bit = 1 << (digit - 1);
        for base_are_rows in [true, false] {
            let positions = line_positions(cands, bit, base_are_rows);
            let eligible: Vec<usize> = (0..9).filter(|&l| positions[l].len() == 2).collect();
            for window in eligible.windows(2) {
                let (l1, l2) = (window[0], window[1]);
                if positions[l1] != positions[l2] {
                    continue;
                }
                let cross = positions[l1].clone();
                let mut corners: Vec<usize> = cross
                    .iter()
                    .flat_map(|&c| {
                        if base_are_rows {
                            [l1 * 9 + c, l2 * 9 + c]
                        } else {
                            [c * 9 + l1, c * 9 + l2]
                        }
                    })
                    .collect();
                corners.sort_unstable();
                let removals = x_removals(cands, digit, &corners, &cross, base_are_rows);
                if removals.is_empty() {
                    continue;
                }
                out.push(x_one(digit, &corners, base_are_rows, &removals));
            }
        }
    }
    out
}

fn line_positions(cands: &Candidates, bit: u16, rows: bool) -> Vec<Vec<usize>> {
    (0..9usize)
        .map(|line| {
            (0..CELL_COUNT)
                .filter(|&i| {
                    let on = if rows { i / 9 } else { i % 9 } == line;
                    on && !cands.placed[i] && cands.masks[i] & bit != 0
                })
                .map(|i| if rows { i % 9 } else { i / 9 })
                .collect()
        })
        .collect()
}

fn x_removals(
    cands: &Candidates,
    digit: u8,
    corners: &[usize],
    cross: &[usize],
    rows: bool,
) -> Vec<(usize, u8)> {
    (0..CELL_COUNT)
        .filter(|&i| {
            !corners.contains(&i)
                && !cands.placed[i]
                && cands.masks[i] & (1 << (digit - 1)) != 0
                && cross.contains(&(if rows { i % 9 } else { i / 9 }))
        })
        .map(|i| (i, digit))
        .collect()
}

fn x_one(
    digit: u8,
    corners: &[usize],
    base_are_rows: bool,
    removals: &[(usize, u8)],
) -> Annotation {
    // The four corners sit on exactly two base lines and two cross
    // lines; collect them distinctly regardless of corner order.
    let mut base_keys = [line_key(corners[0], base_are_rows), usize::MAX];
    let mut cross_keys = [line_key(corners[0], !base_are_rows), usize::MAX];
    for &c in &corners[1..] {
        let b = line_key(c, base_are_rows);
        if b != base_keys[0] {
            base_keys[1] = b;
        }
        let x = line_key(c, !base_are_rows);
        if x != cross_keys[0] {
            cross_keys[1] = x;
        }
    }
    let base = if base_are_rows {
        [UnitRef::Row(base_keys[0]), UnitRef::Row(base_keys[1])]
    } else {
        [UnitRef::Col(base_keys[0]), UnitRef::Col(base_keys[1])]
    };
    let cross = if base_are_rows {
        [UnitRef::Col(cross_keys[0]), UnitRef::Col(cross_keys[1])]
    } else {
        [UnitRef::Row(cross_keys[0]), UnitRef::Row(cross_keys[1])]
    };
    let base_text = base
        .iter()
        .map(|u| u.label())
        .collect::<Vec<_>>()
        .join(" and ");
    Annotation {
        strategy: Strategy::XWing,
        title: format!("X-Wing on {digit}s ({base_text})"),
        digits: vec![digit],
        pattern: corners.to_vec(),
        units: [base.to_vec(), cross.to_vec()].concat(),
        effect: Effect::Eliminate {
            removals: removals.to_vec(),
        },
        steps: x_steps(digit, corners, &base, &cross, base_are_rows, removals),
    }
}

fn line_key(idx: usize, rows: bool) -> usize {
    if rows { idx / 9 } else { idx % 9 }
}

fn x_steps(
    digit: u8,
    corners: &[usize],
    base: &[UnitRef],
    cross: &[UnitRef],
    rows: bool,
    removals: &[(usize, u8)],
) -> Vec<Step> {
    let cross_text = cross
        .iter()
        .map(|u| u.label())
        .collect::<Vec<_>>()
        .join(" and ");
    vec![
        Step {
            cells: corners[..2].to_vec(),
            units: vec![base[0]],
            digits: vec![digit],
            text: format!(
                "In {}, the {digit} has only two homes left.",
                base[0].label()
            ),
        },
        Step {
            cells: corners[2..].to_vec(),
            units: vec![base[1]],
            digits: vec![digit],
            text: format!(
                "In {}, the {digit} is confined to exactly the same {}.",
                base[1].label(),
                if rows { "columns" } else { "rows" }
            ),
        },
        Step {
            cells: corners.to_vec(),
            units: base.to_vec(),
            digits: vec![digit],
            text: format!(
                "The four corners form an X: whichever diagonal pair takes \
                 the {digit}, the {digit} is locked onto {cross_text}."
            ),
        },
        x_elim_step(digit, cross, &cross_text, removals),
    ]
}

fn x_elim_step(digit: u8, cross: &[UnitRef], cross_text: &str, removals: &[(usize, u8)]) -> Step {
    Step {
        cells: removals.iter().map(|&(i, _)| i).collect(),
        units: cross.to_vec(),
        digits: vec![digit],
        text: format!(
            "So every other cell of {cross_text} loses the {digit} - the \
             corners have it reserved."
        ),
    }
}
