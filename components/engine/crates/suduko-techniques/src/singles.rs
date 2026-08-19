//! Singles: the two cheapest ladder rungs.

use crate::{Candidates, Effect, UNITS};

/// Returns the first cell reduced to a single candidate.
///
/// # Panics
///
/// Never panics: a one-bit mask's trailing zero index is always 0..8, so
/// the digit reconstruction stays in 1..=9.
#[must_use]
pub fn naked_single(cands: &Candidates) -> Option<Effect> {
    for (idx, &mask) in cands.masks.iter().enumerate() {
        if !cands.placed[idx] && mask.is_power_of_two() {
            let digit = u8::try_from(mask.trailing_zeros() + 1).expect("single bit: 1..=9");
            return Some(Effect::Place { idx, digit });
        }
    }
    None
}

/// Returns the first digit confined to one cell within a unit.
///
/// # Panics
///
/// Only via the unreachable `expect("count implies spot")`.
#[must_use]
pub fn hidden_single(cands: &Candidates) -> Option<Effect> {
    for unit in &UNITS {
        for digit in 1u8..=9 {
            let bit = 1 << (digit - 1);
            let mut spot = None;
            let mut count = 0;
            for &idx in unit {
                if cands.masks[idx] & bit != 0 {
                    count += 1;
                    spot = Some(idx);
                }
            }
            if count == 1 && spot.is_some_and(|idx| !cands.placed[idx]) {
                return Some(Effect::Place {
                    idx: spot.expect("count implies spot"),
                    digit,
                });
            }
        }
    }
    None
}
