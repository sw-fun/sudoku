//! Engine facade: difficulty levels, band config, and generation dispatch.
//! Downstream consumers (the UI) depend only on this crate plus grid types.

mod bands;
mod level;

pub use bands::{LevelError, clue_range, generate, generate_bounded};
pub use level::Level;
pub use suduko_generator::generate_full;
pub use suduko_grader::{Grade, Technique, grade};
pub use suduko_grid::{Board, CELL_COUNT, Cell, Puzzle, parse, to_string};
pub use suduko_grid::{block_of, col_of, first_conflict, peers_of, row_of};
pub use suduko_solver::{count_solutions, solve};
