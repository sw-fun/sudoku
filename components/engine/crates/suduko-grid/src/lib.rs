//! Core grid model: cells, boards, coordinates, and compact serialization.

pub mod board;
pub mod coords;
pub mod format;
pub mod puzzle;

pub use board::{Board, CELL_COUNT, Cell};
pub use coords::{block_of, col_of, peers_of, row_of};
pub use format::{ParseError, parse, to_string};
pub use puzzle::{Puzzle, PuzzleError, first_conflict};
