//! Deterministic backtracking solver with a capped solution counter.

mod count;
mod search;
pub mod state;

pub use count::{count_solutions, solve};
pub use state::{Pick, State, pick_cell};
