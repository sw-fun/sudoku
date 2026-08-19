//! The technique ladder (chain of responsibility) and the grade entry.

use crate::TRIAL_DEPTH;
use crate::apply::apply;
use crate::trial::try_trial;
use suduko_grid::{Board, first_conflict};
use suduko_techniques::{
    Candidates, hidden_set, hidden_single, locked_candidates, naked_set, naked_single,
};
use suduko_techniques_advanced::{fish, xy_wing};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Technique {
    NakedSingle = 0,
    HiddenSingle = 1,
    LockedCandidates = 2,
    NakedSet = 3,
    HiddenSet = 4,
    XYWing = 5,
    XWing = 6,
    Swordfish = 7,
    Trial = 8,
}

/// Grading result: per-technique counts (indexed by `Technique as usize`),
/// the hardest technique required, a score, and whether the ladder (with
/// its bounded-trial fallback) finished the grid.
///
/// The score is `weight * 100 + applications` with the weight of the
/// hardest technique, so scores stay in hundred-wide bands: 100-299
/// singles only, 300-399 locked candidates, 400-599 subsets, 600-699
/// XY-wing, 700-799 basic fish, 800+ trial and error.
pub struct Grade {
    pub counts: [usize; 9],
    pub hardest: Option<Technique>,
    pub score: usize,
    pub solved: bool,
}

/// Grades a puzzle by solving it with the cheapest human technique first.
/// The board must be clue-consistent (no repeated digit in a unit); an
/// inconsistent board fails fast with `solved: false` and a zero score.
/// Grading near-empty boards is unsupported: the trial fallback would
/// search an astronomical solution space.
#[must_use]
pub fn grade(board: &Board) -> Grade {
    if first_conflict(board).is_some() {
        return Grade {
            counts: [0; 9],
            hardest: None,
            score: 0,
            solved: false,
        };
    }
    let mut cands = Candidates::from_board(board);
    let mut counts = [0usize; 9];
    let mut hardest: Option<Technique> = None;
    let solved;
    loop {
        if cands.placed.iter().all(|&done| done) {
            solved = true;
            break;
        }
        if let Some((technique, effect)) = try_all(&cands) {
            counts[technique as usize] += 1;
            if hardest.is_none_or(|current| technique > current) {
                hardest = Some(technique);
            }
            apply(&mut cands, effect);
        } else {
            counts[Technique::Trial as usize] += 1;
            solved = try_trial(&mut cands, TRIAL_DEPTH);
            if solved {
                hardest = Some(Technique::Trial);
            }
            break;
        }
    }
    let score = hardest.map_or(0, |t| (t as usize + 1) * 100 + counts.iter().sum::<usize>());
    Grade {
        counts,
        hardest,
        score,
        solved,
    }
}

/// Tries the ladder cheapest technique first and returns the first effect.
#[must_use]
pub fn try_all(cands: &Candidates) -> Option<(Technique, suduko_techniques::Effect)> {
    naked_single(cands)
        .map(|e| (Technique::NakedSingle, e))
        .or_else(|| hidden_single(cands).map(|e| (Technique::HiddenSingle, e)))
        .or_else(|| locked_candidates(cands).map(|e| (Technique::LockedCandidates, e)))
        .or_else(|| naked_set(cands).map(|e| (Technique::NakedSet, e)))
        .or_else(|| hidden_set(cands).map(|e| (Technique::HiddenSet, e)))
        .or_else(|| xy_wing(cands).map(|e| (Technique::XYWing, e)))
        .or_else(|| fish(cands, 2).map(|e| (Technique::XWing, e)))
        .or_else(|| fish(cands, 3).map(|e| (Technique::Swordfish, e)))
}
