//! What a technique produces: a placement or a batch of eliminations.

#[derive(Debug, PartialEq, Eq)]
pub enum Effect {
    Place { idx: usize, digit: u8 },
    Eliminate { removals: Vec<(usize, u8)> },
}
