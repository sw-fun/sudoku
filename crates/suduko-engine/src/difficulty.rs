use crate::game::Puzzle;
use crate::generator::{self, DigParams};
use crate::grader::{self, Technique};
use crate::rng::Rng;

/// The five published difficulty levels, ordered by the hardest technique
/// their band accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Easy,
    Medium,
    Hard,
    Harder,
    Hardest,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LevelError {
    /// No accepted puzzle after `attempts` graded digs.
    Exhausted { attempts: usize },
}

struct Band {
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

/// Clue-count window accepted for `level` (givens stay a secondary guard;
/// the primary gate is the technique band).
pub fn clue_range(level: Level) -> std::ops::RangeInclusive<usize> {
    let band = band_of(level);
    band.clue_min..=band.clue_max
}

/// Generates a unique puzzle graded inside `level`'s technique band, drawing
/// digs from a deterministic seed. Same seed, same puzzle.
pub fn generate(level: Level, seed: u64) -> Result<Puzzle, LevelError> {
    generate_bounded(level, seed, 24, 3)
}

/// The acceptance loop: for up to `grid_attempts` fresh grids, draw up to
/// `dig_attempts` independent digs each; accept the first puzzle whose
/// grade lands in the band with a clue count inside the level window.
pub fn generate_bounded(
    level: Level,
    seed: u64,
    dig_attempts: usize,
    grid_attempts: usize,
) -> Result<Puzzle, LevelError> {
    let band = band_of(level);
    let mut master = Rng::new(seed);
    let mut attempts = 0;
    for _ in 0..grid_attempts {
        let full = generator::generate_full(master.next_u64());
        for _ in 0..dig_attempts {
            let params = DigParams {
                target_clues: band.dig_target,
                symmetric: false,
            };
            let puzzle = generator::dig_with(&full, params, master.next_u64());
            attempts += 1;
            let grade = grader::grade(puzzle.clues());
            let clues = puzzle.clue_count();
            if grade.solved
                && grade
                    .hardest
                    .is_some_and(|hardest| band.techniques.contains(&hardest))
                && band.clue_min <= clues
                && clues <= band.clue_max
            {
                return Ok(puzzle);
            }
        }
    }
    Err(LevelError::Exhausted { attempts })
}

fn band_of(level: Level) -> &'static Band {
    match level {
        Level::Easy => &EASY,
        Level::Medium => &MEDIUM,
        Level::Hard => &HARD,
        Level::Harder => &HARDER,
        Level::Hardest => &HARDEST,
    }
}
