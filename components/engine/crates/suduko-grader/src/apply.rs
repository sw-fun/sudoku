//! Applying technique effects to candidate state.

use suduko_techniques::{Candidates, Effect};

/// A placement resolves the cell and clears peers; an elimination removes a
/// candidate unless the cell is already solved or would drop to zero
/// candidates.
pub fn apply(cands: &mut Candidates, effect: Effect) {
    match effect {
        Effect::Place { idx, digit } => cands.place(idx, digit),
        Effect::Eliminate { removals } => {
            for &(idx, digit) in &removals {
                if !cands.placed[idx] && cands.masks[idx].count_ones() > 1 {
                    cands.masks[idx] &= !(1 << (digit - 1));
                }
            }
        }
    }
}
