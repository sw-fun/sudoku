//! The five published difficulty levels.

/// Ordered by the hardest technique their band accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Easy,
    Medium,
    Hard,
    Harder,
    Hardest,
}
