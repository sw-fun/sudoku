use crate::grader::candidates::Candidates;
use crate::grader::effect::Effect;
use crate::grader::units::UNITS;

/// Naked subsets: k cells of a unit whose combined candidates are exactly k
/// digits let those digits be removed from every other cell of the unit.
pub fn naked_set(cands: &Candidates) -> Option<Effect> {
    for size in [2usize, 3] {
        for unit in &UNITS {
            let cells: Vec<usize> = unit
                .iter()
                .copied()
                .filter(|&idx| {
                    let count = cands.masks[idx].count_ones() as usize;
                    !cands.placed[idx] && count >= 2 && count <= size
                })
                .collect();
            for combo in combos(cells.len(), size) {
                let mut union = 0u16;
                for &i in &combo {
                    union |= cands.masks[cells[i]];
                }
                if union.count_ones() as usize != size {
                    continue;
                }
                let chosen: Vec<usize> = combo.iter().map(|&i| cells[i]).collect();
                let mut removals = Vec::new();
                for digit in 1u8..=9 {
                    let bit = 1 << (digit - 1);
                    if union & bit == 0 {
                        continue;
                    }
                    for &idx in unit {
                        if !chosen.contains(&idx)
                            && !cands.placed[idx]
                            && cands.masks[idx] & bit != 0
                        {
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

/// Hidden subsets: k digits of a unit confined to the same k cells let every
/// other candidate be removed from those cells. Every digit of the subset
/// must actually appear in the unit, otherwise the subset is degenerate.
pub fn hidden_set(cands: &Candidates) -> Option<Effect> {
    for size in [2usize, 3] {
        for unit in &UNITS {
            for digits in combos(9, size) {
                let mut union_mask = 0u16;
                for &d in &digits {
                    union_mask |= 1 << d;
                }
                let mut spots: Vec<usize> = Vec::new();
                let mut complete = true;
                for &d in &digits {
                    let bit = 1 << d;
                    let mut anywhere = false;
                    for &idx in unit {
                        if cands.placed[idx] {
                            continue;
                        }
                        if cands.masks[idx] & bit != 0 {
                            anywhere = true;
                        }
                    }
                    if !anywhere {
                        complete = false;
                        break;
                    }
                }
                if !complete {
                    continue;
                }
                for &idx in unit {
                    if cands.placed[idx] {
                        continue;
                    }
                    if cands.masks[idx] & union_mask != 0 {
                        spots.push(idx);
                    }
                }
                if spots.len() != size {
                    continue;
                }
                let mut removals = Vec::new();
                for &idx in &spots {
                    for digit in 1u8..=9 {
                        let bit = 1 << (digit - 1);
                        if union_mask & bit == 0 && cands.masks[idx] & bit != 0 {
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

/// Index combinations of `size` out of `n`, ascending.
fn combos(n: usize, size: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut current = Vec::new();
    build_combos(0, n, size, &mut current, &mut out);
    out
}

fn build_combos(
    start: usize,
    n: usize,
    size: usize,
    current: &mut Vec<usize>,
    out: &mut Vec<Vec<usize>>,
) {
    if current.len() == size {
        out.push(current.clone());
        return;
    }
    for i in start..n {
        current.push(i);
        build_combos(i + 1, n, size, current, out);
        current.pop();
    }
}
