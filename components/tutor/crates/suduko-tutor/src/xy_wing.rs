//! XY-Wing: pivot {x,y} with pincers {x,z} and {y,z}.

use crate::annotations::cell_name;
use crate::annotations::{Annotation, Effect, Step, Strategy};
use suduko_grid::{CELL_COUNT, peers_of};
use suduko_techniques::Candidates;

/// Every XY-Wing on the board.
#[must_use]
pub fn xy_wings(cands: &Candidates) -> Vec<Annotation> {
    let bivalue: Vec<usize> = (0..CELL_COUNT)
        .filter(|&i| !cands.placed[i] && cands.masks[i].count_ones() == 2)
        .collect();
    let mut out = Vec::new();
    for &pivot in &bivalue {
        for &pa in &bivalue {
            if pa == pivot || !sees(pa, pivot) {
                continue;
            }
            for &pb in &bivalue {
                if pb == pivot || pb == pa || !sees(pb, pivot) || !sees(pa, pb) {
                    continue;
                }
                let Some((dx, dy, dz)) = wing_digits(cands, pivot, pa, pb) else {
                    continue;
                };
                let removals = z_removals(cands, pa, pb, dz);
                if removals.is_empty() {
                    continue;
                }
                out.push(xy_one(pivot, pa, pb, dx, dy, dz, &removals));
            }
        }
    }
    out
}

fn wing_digits(cands: &Candidates, pivot: usize, pa: usize, pb: usize) -> Option<(u8, u8, u8)> {
    let (pm, am, bm) = (cands.masks[pivot], cands.masks[pa], cands.masks[pb]);
    let shared_a = pm & am;
    let shared_b = pm & bm;
    if shared_a.count_ones() != 1
        || shared_b.count_ones() != 1
        || shared_a == shared_b
        || (am & bm).count_ones() != 1
        || am & bm & pm != 0
    {
        return None;
    }
    let bit = |m: u16| u8::try_from(m.trailing_zeros() + 1).expect("bit 1..=9");
    Some((bit(shared_a), bit(shared_b), bit(am & bm)))
}

fn sees(a: usize, b: usize) -> bool {
    peers_of(a).contains(&b)
}

fn z_removals(cands: &Candidates, pa: usize, pb: usize, dz: u8) -> Vec<(usize, u8)> {
    let bit = 1 << (dz - 1);
    (0..CELL_COUNT)
        .filter(|&t| {
            t != pa
                && t != pb
                && !cands.placed[t]
                && cands.masks[t] & bit != 0
                && sees(t, pa)
                && sees(t, pb)
        })
        .map(|t| (t, dz))
        .collect()
}

fn xy_one(
    pivot: usize,
    pa: usize,
    pb: usize,
    dx: u8,
    dy: u8,
    dz: u8,
    removals: &[(usize, u8)],
) -> Annotation {
    let mut digits = vec![dx, dy, dz];
    digits.sort_unstable();
    let mut pattern = vec![pivot, pa, pb];
    pattern.sort_unstable();
    Annotation {
        strategy: Strategy::XYWing,
        title: format!(
            "XY-Wing: pivot {} with pincers {} and {}",
            cell_name(pivot),
            cell_name(pa),
            cell_name(pb)
        ),
        digits,
        pattern,
        units: vec![],
        effect: Effect::Eliminate {
            removals: removals.to_vec(),
        },
        steps: xy_steps(pivot, pa, pb, dx, dy, dz, removals),
    }
}

#[allow(clippy::too_many_arguments)]
fn xy_steps(
    pivot: usize,
    pa: usize,
    pb: usize,
    dx: u8,
    dy: u8,
    dz: u8,
    removals: &[(usize, u8)],
) -> Vec<Step> {
    let targets: Vec<usize> = removals.iter().map(|&(i, _)| i).collect();
    vec![
        Step {
            cells: vec![pivot],
            units: vec![],
            digits: vec![dx, dy],
            text: format!(
                "The pivot {} holds exactly two candidates: {dx} or {dy}.",
                cell_name(pivot)
            ),
        },
        Step {
            cells: vec![pa, pb],
            units: vec![],
            digits: vec![dx, dy, dz],
            text: format!(
                "Two pincers see the pivot: {} takes the leftover of {dx} \
                 (it holds {dx} or {dz}) and {} takes the leftover of {dy} \
                 (it holds {dy} or {dz}).",
                cell_name(pa),
                cell_name(pb)
            ),
        },
        Step {
            cells: vec![pivot, pa, pb],
            units: vec![],
            digits: vec![dz],
            text: format!(
                "Either way the pivot resolves, one pincer becomes the \
                 {dz} - so any cell seeing both pincers is never {dz}."
            ),
        },
        Step {
            cells: targets,
            units: vec![],
            digits: vec![dz],
            text: format!("These cells see both pincers; the {dz} can be erased."),
        },
    ]
}
