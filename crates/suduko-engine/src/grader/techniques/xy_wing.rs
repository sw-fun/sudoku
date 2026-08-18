use crate::grader::candidates::Candidates;
use crate::grader::effect::Effect;
use crate::grid::CELL_COUNT;
use crate::grid::coords::peers_of;

/// XY-wing: a pivot with exactly two candidates {x,y} and two pincer peers
/// holding {x,z} and {y,z} let z be removed from every cell seeing both
/// pincers. Only the (few) two-candidate cells can participate.
pub fn xy_wing(cands: &Candidates) -> Option<Effect> {
    let bivalue: Vec<usize> = (0..CELL_COUNT)
        .filter(|&idx| !cands.placed[idx] && cands.masks[idx].count_ones() == 2)
        .collect();
    for &pivot in &bivalue {
        let pmask = cands.masks[pivot];
        let pivot_peers = peers_of(pivot);
        for &pincer_a in &bivalue {
            if pincer_a == pivot
                || !pivot_peers.contains(&pincer_a)
                || cands.masks[pincer_a] & !pmask == 0
            {
                continue;
            }
            let x = cands.masks[pincer_a] & pmask;
            let z = cands.masks[pincer_a] & !pmask;
            let wanted_b = (pmask & !x) | z;
            for &pincer_b in &bivalue {
                if pincer_b == pivot
                    || pincer_b == pincer_a
                    || !pivot_peers.contains(&pincer_b)
                    || cands.masks[pincer_b] != wanted_b
                {
                    continue;
                }
                let removals = eliminations(cands, pincer_a, pincer_b, z);
                if !removals.is_empty() {
                    return Some(Effect::Eliminate { removals });
                }
            }
        }
    }
    None
}

fn eliminations(cands: &Candidates, a: usize, b: usize, z: u16) -> Vec<(usize, u8)> {
    let digit = z.trailing_zeros() as u8 + 1;
    let peers_a = peers_of(a);
    let peers_b = peers_of(b);
    let mut removals = Vec::new();
    for target in 0..CELL_COUNT {
        if target != a
            && target != b
            && !cands.placed[target]
            && peers_a.contains(&target)
            && peers_b.contains(&target)
            && cands.masks[target] & z != 0
        {
            removals.push((target, digit));
        }
    }
    removals
}
