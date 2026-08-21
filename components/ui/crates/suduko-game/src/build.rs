//! Game construction from engine puzzles and serialized strings.

use super::{BuildError, Game};
use suduko_grid::{Board, CELL_COUNT, Cell, Puzzle};

/// Builds a game from 81-char clue/solution strings ('.' or '0' empty).
///
/// # Errors
///
/// Returns `BuildError::BadStrings` when the strings do not parse or do
/// not form a valid puzzle; the UI only feeds it engine output.
pub fn from_strings(clues: &str, solution: &str) -> Result<Game, BuildError> {
    let (clues, solution) = (
        parse(clues).ok_or(BuildError::BadStrings)?,
        parse(solution).ok_or(BuildError::BadStrings)?,
    );
    let puzzle = Puzzle::new(clues, solution).map_err(|_| BuildError::BadStrings)?;
    Ok(from_puzzle(&puzzle))
}

/// Builds a fresh, empty game from a validated engine puzzle.
pub fn from_puzzle(puzzle: &Puzzle) -> Game {
    Game {
        clues: digits(puzzle.clues()),
        solution: digits(puzzle.solution()),
        user: [0; CELL_COUNT],
        selected: None,
        bad_inputs: 0,
        won: false,
        elapsed_secs: 0,
        teaching: super::Teaching::default(),
    }
}

fn digits(board: &Board) -> [u8; CELL_COUNT] {
    let mut out = [0u8; CELL_COUNT];
    for (idx, slot) in out.iter_mut().enumerate() {
        if let Cell::Value(v) = board.get(idx) {
            *slot = v;
        }
    }
    out
}

fn parse(text: &str) -> Option<Board> {
    suduko_grid::parse(text).ok()
}
