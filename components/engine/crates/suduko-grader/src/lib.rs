//! The grading engine: ladder driver and bounded-trial fallback.

mod apply;
pub mod ladder;
mod trial;

pub use apply::apply;
pub use ladder::{Grade, Technique, grade, try_all};

/// How many nested guesses the fallback may take after the ladder stalls.
/// Depth 8 completes AI Escargot and Golden Nugget well inside the 2s
/// grading bound.
pub const TRIAL_DEPTH: usize = 8;
