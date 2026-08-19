//! XY-wing: pivot {x,y} with pincers {x,z} and {y,z} eliminates z from
//! every cell seeing both pincers. Only two-candidate cells participate.

use suduko_grid::{CELL_COUNT, peers_of};
use suduko_techniques::{Candidates, Effect};

#[must_use]
pub fn xy_wing(cands: &Candidates) -> Option<Effect> {
    let bivalue: Vec<usize> = (0..CELL_COUNT)
        .filter(|&i| !cands.placed[i] && cands.masks[i].count_ones() == 2)
        .collect();
    for &pivot in &bivalue {
        if let Some(removals) = wings_from(cands, &bivalue, pivot) {
            return Some(Effect::Eliminate { removals });
        }
    }
    None
}

fn wings_from(cands: &Candidates, bivalue: &[usize], pivot: usize) -> Option<Vec<(usize, u8)>> {
    let pmask = cands.masks[pivot];
    let pivot_peers = peers_of(pivot);
    for &a in bivalue {
        if a == pivot || !pivot_peers.contains(&a) || cands.masks[a] & !pmask == 0 {
            continue;
        }
        let x = cands.masks[a] & pmask;
        let z = cands.masks[a] & !pmask;
        let wanted = (pmask & !x) | z;
        for &b in bivalue {
            if b == pivot || b == a || !pivot_peers.contains(&b) || cands.masks[b] != wanted {
                continue;
            }
            let hits = z_removals(cands, a, b, z);
            if !hits.is_empty() {
                return Some(hits);
            }
        }
    }
    None
}

fn z_removals(cands: &Candidates, a: usize, b: usize, z: u16) -> Vec<(usize, u8)> {
    let digit = u8::try_from(z.trailing_zeros() + 1).expect("single bit: 1..=9");
    let (peers_a, peers_b) = (peers_of(a), peers_of(b));
    (0..CELL_COUNT)
        .filter(|&t| {
            t != a
                && t != b
                && !cands.placed[t]
                && peers_a.contains(&t)
                && peers_b.contains(&t)
                && cands.masks[t] & z != 0
        })
        .map(|t| (t, digit))
        .collect()
}
