//! Game construction from engine puzzles and serialized strings, plus
//! the versioned save codec (one slot: level, stats, optional
//! in-progress game).

use super::{BuildError, Game};
use suduko_grid::{Board, CELL_COUNT, Cell, Puzzle};
use suduko_tutor::{Annotation, Candidates, Effect, Step, Strategy, cell_name};

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
        show_me: false,
        show_me_auto: false,
        show_me_delay_ticks: 0,
        show_me_wait: 0,
        eliminated: Vec::new(),
        pencil: false,
        user_marks: [0; CELL_COUNT],
        keypad_open: false,
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

/// A Trial annotation for the most constrained empty cell, or None
/// when the board is finished. The digit comes from the stored unique
/// solution, so the placement is guaranteed correct.
#[must_use]
pub fn trial_annotation(game: &Game, cands: &Candidates) -> Option<Annotation> {
    let idx = (0..CELL_COUNT)
        .filter(|&i| !cands.placed[i] && cands.masks[i].count_ones() >= 1)
        .min_by_key(|&i| cands.masks[i].count_ones())?;
    let digit = game.solution[idx];
    let options: Vec<u8> = (1..=9u8)
        .filter(|d| cands.masks[idx] & (1 << (d - 1)) != 0)
        .collect();
    Some(Annotation {
        strategy: Strategy::Trial,
        title: format!("Trial: place {digit} in {}", cell_name(idx)),
        digits: vec![digit],
        pattern: vec![idx],
        units: vec![],
        effect: Effect::Place { idx, digit },
        steps: trial_steps(idx, digit, &options),
    })
}

fn trial_steps(idx: usize, digit: u8, options: &[u8]) -> Vec<Step> {
    let option_text = options
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" or ");
    vec![
        Step {
            cells: vec![idx],
            units: vec![],
            digits: options.to_vec(),
            text: "The taught strategies are exhausted on this board - \
                       harder patterns (swordfish, chains) are beyond this \
                       tutor, so we reason it out instead."
                .to_string(),
        },
        Step {
            cells: vec![idx],
            units: vec![],
            digits: options.to_vec(),
            text: format!(
                "The most constrained cell is {}: only {option_text} \
                     remain possible there.",
                cell_name(idx)
            ),
        },
        Step {
            cells: vec![idx],
            units: vec![],
            digits: vec![digit],
            text: format!(
                "We place {digit} - the puzzle's unique solution \
                     confirms it - and new strategies open up from there."
            ),
        },
    ]
}
