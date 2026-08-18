use super::candidates::Candidates;

#[derive(Debug, PartialEq, Eq)]
pub enum Effect {
    Place { idx: usize, digit: u8 },
    Eliminate { removals: Vec<(usize, u8)> },
}

pub fn apply(cands: &mut Candidates, effect: Effect) {
    match effect {
        Effect::Place { idx, digit } => cands.place(idx, digit),
        Effect::Eliminate { removals } => {
            for &(idx, digit) in &removals {
                cands.eliminate(idx, digit);
            }
        }
    }
}
