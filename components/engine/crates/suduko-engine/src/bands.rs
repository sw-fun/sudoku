//! Band configuration and the accept/reject generation loop.

use crate::Level;
use suduko_generator::{DigParams, Rng, dig, generate_full};
use suduko_grader::{Technique, grade};
use suduko_grid::Puzzle;

/// A level's acceptance configuration: which hardest techniques count,
/// how deep to dig, and the clue-count window.
pub(crate) struct Band {
    techniques: &'static [Technique],
    dig_target: usize,
    clue_min: usize,
    clue_max: usize,
}

const EASY: Band = Band {
    techniques: &[Technique::NakedSingle, Technique::HiddenSingle],
    dig_target: 44,
    clue_min: 38,
    clue_max: 55,
};
const MEDIUM: Band = Band {
    techniques: &[Technique::LockedCandidates],
    dig_target: 28,
    clue_min: 26,
    clue_max: 40,
};
const HARD: Band = Band {
    techniques: &[Technique::NakedSet, Technique::HiddenSet],
    dig_target: 26,
    clue_min: 24,
    clue_max: 32,
};
const HARDER: Band = Band {
    techniques: &[Technique::XYWing, Technique::XWing, Technique::Swordfish],
    dig_target: 26,
    clue_min: 22,
    clue_max: 31,
};
const HARDEST: Band = Band {
    techniques: &[Technique::Trial],
    dig_target: 24,
    clue_min: 22,
    clue_max: 30,
};

fn band(level: Level) -> &'static Band {
    match level {
        Level::Easy => &EASY,
        Level::Medium => &MEDIUM,
        Level::Hard => &HARD,
        Level::Harder => &HARDER,
        Level::Hardest => &HARDEST,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LevelError {
    /// No accepted puzzle after `attempts` graded digs.
    Exhausted { attempts: usize },
}

/// Clue-count window accepted for `level` (secondary guard; the primary
/// gate is the technique band).
#[must_use]
pub fn clue_range(level: Level) -> std::ops::RangeInclusive<usize> {
    let b = band(level);
    b.clue_min..=b.clue_max
}

/// Generates a unique puzzle graded inside `level`'s technique band.
/// Same seed, same puzzle.
///
/// # Errors
///
/// Returns `LevelError::Exhausted` when no accepted puzzle is found within
/// the default attempt budget (24 digs x 3 grids).
#[must_use = "generation is deterministic and expensive; use the result"]
pub fn generate(level: Level, seed: u64) -> Result<Puzzle, LevelError> {
    generate_bounded(level, seed, 24, 3)
}

/// The acceptance loop: for up to `grid_attempts` fresh grids, draw up to
/// `dig_attempts` independent digs each; accept the first in-band puzzle
/// with a clue count inside the level window.
///
/// # Errors
///
/// Returns `LevelError::Exhausted` with the number of graded digs after
/// the budget is spent.
///
/// # Panics
///
/// Never: dug clues always agree with the grid they came from, so the
/// `Puzzle` construction expect is unreachable.
pub fn generate_bounded(
    level: Level,
    seed: u64,
    dig_attempts: usize,
    grid_attempts: usize,
) -> Result<Puzzle, LevelError> {
    let b = band(level);
    let params = DigParams {
        target_clues: b.dig_target,
        symmetric: false,
    };
    let mut master = Rng::new(seed);
    let mut attempts = 0;
    for _ in 0..grid_attempts {
        let full = generate_full(master.next_u64());
        for _ in 0..dig_attempts {
            let clues = dig(&full, params, &mut Rng::new(master.next_u64()));
            attempts += 1;
            let puzzle = Puzzle::new(clues, full.clone()).expect("dug clues agree");
            if accepts(&puzzle, b) {
                return Ok(puzzle);
            }
        }
    }
    Err(LevelError::Exhausted { attempts })
}

fn accepts(puzzle: &Puzzle, b: &Band) -> bool {
    let g = grade(puzzle.clues());
    let clues = puzzle.clue_count();
    g.solved
        && g.hardest
            .is_some_and(|hardest| b.techniques.contains(&hardest))
        && b.clue_min <= clues
        && clues <= b.clue_max
}
