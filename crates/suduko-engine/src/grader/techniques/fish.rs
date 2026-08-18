use crate::grader::candidates::Candidates;
use crate::grader::effect::Effect;
use crate::grid::CELL_COUNT;
use crate::grid::coords::{col_of, row_of};

pub fn x_wing(cands: &Candidates) -> Option<Effect> {
    fish(cands, 2)
}

pub fn swordfish(cands: &Candidates) -> Option<Effect> {
    fish(cands, 3)
}

/// Basic fish of the given size over rows (eliminating in columns) and then
/// over columns (eliminating in rows): `size` lines whose digit positions
/// stay within `size` cross-lines remove the digit from those cross-lines
/// elsewhere.
fn fish(cands: &Candidates, size: usize) -> Option<Effect> {
    for digit in 1u8..=9 {
        let bit = 1 << (digit - 1);
        for &rows_first in &[true, false] {
            let mut lines: Vec<Vec<usize>> = Vec::new();
            for line in 0..9usize {
                let positions: Vec<usize> = (0..CELL_COUNT)
                    .filter(|&idx| {
                        let on_line = if rows_first { row_of(idx) } else { col_of(idx) } == line;
                        on_line && !cands.placed[idx] && cands.masks[idx] & bit != 0
                    })
                    .map(|idx| if rows_first { col_of(idx) } else { row_of(idx) })
                    .collect();
                lines.push(positions);
            }
            let candidates: Vec<usize> = (0..9)
                .filter(|&l| (2..=size).contains(&lines[l].len()))
                .collect();
            if candidates.len() < size {
                continue;
            }
            let n = candidates.len();
            for mask in 0..(1usize << n) {
                if mask.count_ones() as usize != size {
                    continue;
                }
                let combo: Vec<usize> = (0..n)
                    .filter(|&i| mask & (1 << i) != 0)
                    .map(|i| candidates[i])
                    .collect();
                let mut cross = Vec::new();
                for &line in &combo {
                    for &c in &lines[line] {
                        if !cross.contains(&c) {
                            cross.push(c);
                        }
                    }
                }
                if cross.len() != size {
                    continue;
                }
                let mut removals = Vec::new();
                for &c in &cross {
                    for line in 0..9usize {
                        if combo.contains(&line) {
                            continue;
                        }
                        let idx = if rows_first {
                            line * 9 + c
                        } else {
                            c * 9 + line
                        };
                        if !cands.placed[idx] && cands.masks[idx] & bit != 0 {
                            removals.push((idx, digit));
                        }
                    }
                }
                if !removals.is_empty() {
                    return Some(Effect::Eliminate { removals });
                }
            }
        }
    }
    None
}
