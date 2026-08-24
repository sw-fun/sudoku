//! Pure UI kit for the Sudoku app. Everything here is plain data in,
//! plain data out: no browser types, no game state. The Yew layer
//! stays thin by delegating placement decisions and button rules to
//! this crate.

pub mod input;
pub mod keypad;
pub mod time;

pub use input::{InputMode, NotesMode};
pub use keypad::{Anchor, CellInput, HAlign, anchor, anchor_style};
pub use time::mmss;
