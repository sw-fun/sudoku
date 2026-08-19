//! Basic fish: X-wing (size 2) and swordfish (size 3).

use suduko_grid::{CELL_COUNT, col_of, row_of};
use suduko_techniques::{Candidates, Effect};

/// `size` parallel lines whose digit positions stay within `size`
/// cross-lines remove the digit from those cross-lines elsewhere.
#[must_use]
pub fn fish(cands: &Candidates, size: usize) -> Option<Effect> {
    for digit in 1u8..=9 {
        let bit = 1 << (digit - 1);
        for rows_first in [true, false] {
            if let Some(removals) = fish_one(cands, bit, digit, rows_first, size) {
                return Some(Effect::Eliminate { removals });
            }
        }
    }
    None
}

fn fish_one(
    cands: &Candidates,
    bit: u16,
    digit: u8,
    rows_first: bool,
    size: usize,
) -> Option<Vec<(usize, u8)>> {
    let positions = line_positions(cands, bit, rows_first);
    let eligible: Vec<usize> = (0..9)
        .filter(|&l| (2..=size).contains(&positions[l].len()))
        .collect();
    for mask in 0..(1usize << eligible.len()) {
        if mask.count_ones() as usize != size {
            continue;
        }
        let combo: Vec<usize> = (0..eligible.len())
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| eligible[i])
            .collect();
        let mut cross: Vec<usize> = combo.iter().flat_map(|&l| positions[l].clone()).collect();
        cross.sort_unstable();
        cross.dedup();
        if cross.len() != size {
            continue;
        }
        let hits: Vec<(usize, u8)> = (0..CELL_COUNT)
            .filter(|&i| outside_combo(cands, i, bit, &combo, &cross, rows_first))
            .map(|i| (i, digit))
            .collect();
        if !hits.is_empty() {
            return Some(hits);
        }
    }
    None
}

fn line_positions(cands: &Candidates, bit: u16, rows_first: bool) -> Vec<Vec<usize>> {
    (0..9usize)
        .map(|line| {
            (0..CELL_COUNT)
                .filter(|&i| {
                    let on = if rows_first { row_of(i) } else { col_of(i) } == line;
                    on && !cands.placed[i] && cands.masks[i] & bit != 0
                })
                .map(|i| if rows_first { col_of(i) } else { row_of(i) })
                .collect()
        })
        .collect()
}

fn outside_combo(
    cands: &Candidates,
    idx: usize,
    bit: u16,
    combo: &[usize],
    cross: &[usize],
    rows_first: bool,
) -> bool {
    let (line, cross_line) = if rows_first {
        (row_of(idx), col_of(idx))
    } else {
        (col_of(idx), row_of(idx))
    };
    !combo.contains(&line)
        && cross.contains(&cross_line)
        && !cands.placed[idx]
        && cands.masks[idx] & bit != 0
}
