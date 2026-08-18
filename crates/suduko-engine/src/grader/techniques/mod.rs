pub mod fish;
pub mod intersections;
pub mod singles;
pub mod subsets;

use super::candidates::Candidates;
use super::effect::Effect;

pub use fish::{swordfish, x_wing};
pub use intersections::locked_candidates;
pub use singles::{hidden_single, naked_single};
pub use subsets::{hidden_set, naked_set};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Technique {
    NakedSingle = 0,
    HiddenSingle = 1,
    LockedCandidates = 2,
    NakedSet = 3,
    HiddenSet = 4,
    XWing = 5,
    Swordfish = 6,
    Trial = 7,
}

impl Technique {
    pub fn weight(self) -> usize {
        self as usize + 1
    }
}

/// Tries the ladder cheapest technique first and returns the first effect.
pub fn try_all(cands: &Candidates) -> Option<(Technique, Effect)> {
    if let Some(e) = naked_single(cands) {
        return Some((Technique::NakedSingle, e));
    }
    if let Some(e) = hidden_single(cands) {
        return Some((Technique::HiddenSingle, e));
    }
    if let Some(e) = locked_candidates(cands) {
        return Some((Technique::LockedCandidates, e));
    }
    if let Some(e) = naked_set(cands) {
        return Some((Technique::NakedSet, e));
    }
    if let Some(e) = hidden_set(cands) {
        return Some((Technique::HiddenSet, e));
    }
    if let Some(e) = x_wing(cands) {
        return Some((Technique::XWing, e));
    }
    if let Some(e) = swordfish(cands) {
        return Some((Technique::Swordfish, e));
    }
    None
}
