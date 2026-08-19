//! Seeded generation: RNG, random full grids, uniqueness-preserving digs.

mod dig;
mod full;
mod rng;

pub use dig::{DigParams, dig};
pub use full::generate_full;
pub use rng::Rng;
