pub mod candidates;
pub mod effect;
pub mod techniques;
mod trial;
pub mod units;

use crate::grid::Board;
use candidates::Candidates;
pub use techniques::Technique;

/// How many nested guesses the bounded-trial fallback may take after the
/// technique ladder stalls. Depth 8 completes AI Escargot and Golden Nugget
/// well inside the 2s grading bound.
const TRIAL_DEPTH: usize = 8;

/// Grading result: how often each technique fired (indexed by
/// `Technique as usize`), the hardest technique required, a score, and
/// whether the ladder (with its bounded-trial fallback) finished the grid.
///
/// The score is `weight * 100 + applications` where the weight belongs to
/// the hardest technique required, so scores stay inside hundred-wide bands:
/// 100-299 singles only, 300-399 locked candidates, 400-599 subsets,
/// 600-799 fish, 800+ requires trial and error.
pub struct Grade {
    pub counts: [usize; 8],
    pub hardest: Option<Technique>,
    pub score: usize,
    pub solved: bool,
}

/// Grades a puzzle by solving it with the cheapest human technique first.
pub fn grade(board: &Board) -> Grade {
    let mut cands = Candidates::from_board(board);
    let mut counts = [0usize; 8];
    let mut hardest: Option<Technique> = None;
    let solved;
    loop {
        if cands.placed.iter().all(|&done| done) {
            solved = true;
            break;
        }
        match techniques::try_all(&cands) {
            Some((technique, effect)) => {
                counts[technique as usize] += 1;
                if hardest.is_none_or(|current| technique.weight() > current.weight()) {
                    hardest = Some(technique);
                }
                effect::apply(&mut cands, effect);
            }
            None => {
                counts[Technique::Trial as usize] += 1;
                solved = trial::try_trial(&mut cands, TRIAL_DEPTH);
                if solved {
                    hardest = Some(Technique::Trial);
                }
                break;
            }
        }
    }
    let score = match hardest {
        None => 0,
        Some(technique) => technique.weight() * 100 + counts.iter().sum::<usize>(),
    };
    Grade {
        counts,
        hardest,
        score,
        solved,
    }
}
