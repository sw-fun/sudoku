pub mod fish;
pub mod intersections;
pub mod singles;
pub mod subsets;
pub mod xy_wing;

use super::candidates::Candidates;
use super::effect::Effect;

pub use fish::{swordfish, x_wing};
pub use intersections::locked_candidates;
pub use singles::{hidden_single, naked_single};
pub use subsets::{hidden_set, naked_set};
pub use xy_wing::xy_wing;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Technique {
    NakedSingle = 0,
    HiddenSingle = 1,
    LockedCandidates = 2,
    NakedSet = 3,
    HiddenSet = 4,
    XYWing = 5,
    XWing = 6,
    Swordfish = 7,
    Trial = 8,
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
    if let Some(e) = xy_wing(cands) {
        return Some((Technique::XYWing, e));
    }
    if let Some(e) = x_wing(cands) {
        return Some((Technique::XWing, e));
    }
    if let Some(e) = swordfish(cands) {
        return Some((Technique::Swordfish, e));
    }
    None
}
