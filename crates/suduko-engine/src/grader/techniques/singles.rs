use crate::grader::candidates::Candidates;
use crate::grader::effect::Effect;
use crate::grader::units::UNITS;

pub fn naked_single(cands: &Candidates) -> Option<Effect> {
    for (idx, &mask) in cands.masks.iter().enumerate() {
        if !cands.placed[idx] && mask.count_ones() == 1 {
            return Some(Effect::Place {
                idx,
                digit: mask.trailing_zeros() as u8 + 1,
            });
        }
    }
    None
}

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
            if count == 1 && !cands.placed[spot.expect("count implies spot")] {
                return Some(Effect::Place {
                    idx: spot.expect("count implies spot"),
                    digit,
                });
            }
        }
    }
    None
}
