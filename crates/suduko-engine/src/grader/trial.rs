use super::candidates::Candidates;
use super::effect;
use super::techniques;

/// Runs the technique ladder, then branches on the most constrained cell when
/// the ladder stalls, up to `depth` nested guesses. On success `cands` holds
/// the completed grid.
pub fn try_trial(cands: &mut Candidates, depth: usize) -> bool {
    let mut work = cands.clone();
    loop {
        if is_solved(&work) {
            *cands = work;
            return true;
        }
        match techniques::try_all(&work) {
            Some((_, effect)) => effect::apply(&mut work, effect),
            None => break,
        }
    }
    if depth == 0 {
        return false;
    }
    let (idx, mask) = match most_constrained(&work) {
        Some(found) => found,
        None => return false,
    };
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

fn most_constrained(cands: &Candidates) -> Option<(usize, u16)> {
    let mut best: Option<(usize, u16)> = None;
    for (idx, &mask) in cands.masks.iter().enumerate() {
        if cands.placed[idx] {
            continue;
        }
        if best.is_none_or(|(_, best_mask)| mask.count_ones() < best_mask.count_ones()) {
            best = Some((idx, mask));
        }
    }
    best
}

fn is_solved(cands: &Candidates) -> bool {
    cands.placed.iter().all(|&done| done)
}
