//! Naked and hidden subsets (pairs and triples).

use crate::{Candidates, Effect, UNITS};

/// Naked subsets: k cells of a unit whose combined candidates are exactly k
/// digits let those digits be removed from every other cell of the unit.
#[must_use]
pub fn naked_set(cands: &Candidates) -> Option<Effect> {
    for size in [2usize, 3] {
        for unit in &UNITS {
            if let Some(removals) = naked_in_unit(cands, unit, size) {
                return Some(Effect::Eliminate { removals });
            }
        }
    }
    None
}

fn naked_in_unit(cands: &Candidates, unit: &[usize; 9], size: usize) -> Option<Vec<(usize, u8)>> {
    let cells: Vec<usize> = unit
        .iter()
        .copied()
        .filter(|&idx| {
            !cands.placed[idx] && (2..=size).contains(&(cands.masks[idx].count_ones() as usize))
        })
        .collect();
    for mask in 0..(1usize << cells.len()) {
        if mask.count_ones() as usize != size {
            continue;
        }
        let chosen: Vec<usize> = (0..cells.len())
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| cells[i])
            .collect();
        let union: u16 = chosen.iter().map(|&c| cands.masks[c]).fold(0, |a, m| a | m);
        if union.count_ones() as usize != size {
            continue;
        }
        let removals: Vec<(usize, u8)> = unit
            .iter()
            .copied()
            .filter(|&i| !chosen.contains(&i) && !cands.placed[i] && cands.masks[i] & union != 0)
            .flat_map(|i| {
                (1..=9u8)
                    .filter(move |d| cands.masks[i] & (1 << (d - 1)) & union != 0)
                    .map(move |d| (i, d))
            })
            .collect();
        if !removals.is_empty() {
            return Some(removals);
        }
    }
    None
}

/// Hidden subsets: k digits of a unit confined to the same k cells let every
/// other candidate be removed from those cells. Every digit of the subset
/// must actually appear in the unit, otherwise the subset is degenerate.
#[must_use]
pub fn hidden_set(cands: &Candidates) -> Option<Effect> {
    for size in [2usize, 3] {
        for unit in &UNITS {
            if let Some(removals) = hidden_in_unit(cands, unit, size) {
                return Some(Effect::Eliminate { removals });
            }
        }
    }
    None
}

fn hidden_in_unit(cands: &Candidates, unit: &[usize; 9], size: usize) -> Option<Vec<(usize, u8)>> {
    // Only digits with at least one open position in the unit can form a
    // hidden subset; enumerating combinations of those keeps this cheap.
    let present: Vec<usize> = (0..9usize)
        .filter(|&k| {
            unit.iter()
                .any(|&i| !cands.placed[i] && cands.masks[i] & (1 << k) != 0)
        })
        .collect();
    for combo in combos(&present, size) {
        let dmask: u16 = combo.iter().fold(0, |acc, &k| acc | (1 << k));
        let spots: Vec<usize> = unit
            .iter()
            .copied()
            .filter(|&i| !cands.placed[i] && cands.masks[i] & dmask != 0)
            .collect();
        if spots.len() != size {
            continue;
        }
        let removals: Vec<(usize, u8)> = spots
            .iter()
            .copied()
            .flat_map(|i| {
                (1..=9u8)
                    .filter(move |d| {
                        dmask & (1 << (d - 1)) == 0 && cands.masks[i] & (1 << (d - 1)) != 0
                    })
                    .map(move |d| (i, d))
            })
            .collect();
        if !removals.is_empty() {
            return Some(removals);
        }
    }
    None
}

/// Index combinations of `size` out of `items`, ascending.
fn combos(items: &[usize], size: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut current = Vec::new();
    build_combos(items, 0, size, &mut current, &mut out);
    out
}

fn build_combos(
    items: &[usize],
    start: usize,
    size: usize,
    current: &mut Vec<usize>,
    out: &mut Vec<Vec<usize>>,
) {
    if current.len() == size {
        out.push(current.clone());
        return;
    }
    for i in start..items.len() {
        current.push(items[i]);
        build_combos(items, i + 1, size, current, out);
        current.pop();
    }
}
