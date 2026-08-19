//! Bounded-trial fallback for puzzles beyond the technique ladder.

use crate::apply::apply;
use crate::ladder::try_all;
use suduko_grid::CELL_COUNT;
use suduko_techniques::Candidates;

/// Runs the ladder, then branches on the most constrained cell up to
/// `depth` nested guesses. On success `cands` holds the completed grid.
pub fn try_trial(cands: &mut Candidates, depth: usize) -> bool {
    let work = run_ladder(cands.clone());
    if work.placed.iter().all(|&done| done) {
        *cands = work;
        return true;
    }
    let Some(idx) = most_constrained(&work) else {
        return false;
    };
    if depth == 0 {
        return false;
    }
    let mask = work.masks[idx];
    for digit in 1u8..=9 {
        let bit = 1 << (digit - 1);
        if mask & bit != 0 {
            let mut branch = work.clone();
            branch.place(idx, digit);
            if try_trial(&mut branch, depth - 1) {
                *cands = branch;
                return true;
            }
        }
    }
    false
}

fn run_ladder(mut work: Candidates) -> Candidates {
    loop {
        if work.placed.iter().all(|&done| done) {
            return work;
        }
        match try_all(&work) {
            Some((_, effect)) => apply(&mut work, effect),
            None => return work,
        }
    }
}

fn most_constrained(work: &Candidates) -> Option<usize> {
    (0..CELL_COUNT)
        .filter(|&i| !work.placed[i])
        .min_by_key(|&i| work.masks[i].count_ones())
}
