use suduko_grader::{Technique, grade};
use suduko_grid::{Board, CELL_COUNT, Cell, parse};
use suduko_solver::solve;
use suduko_techniques::Candidates;
use suduko_techniques::Effect;

const WIKIPEDIA_PUZZLE: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const AI_ESCARGOT: &str =
    "1....7.9..3..2...8..96..5....53..9...1..8...26....4...3......1..4......7..7...3..";

fn ladder_techniques() -> [Technique; 6] {
    [
        Technique::LockedCandidates,
        Technique::NakedSet,
        Technique::HiddenSet,
        Technique::XYWing,
        Technique::XWing,
        Technique::Swordfish,
    ]
}

#[test]
fn singles_solvable_puzzle_needs_only_singles() {
    let puzzle = parse(WIKIPEDIA_PUZZLE).unwrap();
    let g = grade(&puzzle);
    assert!(g.solved);
    for technique in ladder_techniques() {
        assert_eq!(g.counts[technique as usize], 0, "{technique:?} fired");
    }
    assert!(g.hardest <= Some(Technique::HiddenSingle));
    assert!(
        g.score < 300,
        "score stays in the singles band: {}",
        g.score
    );
}

#[test]
fn grading_is_deterministic() {
    let puzzle = parse(WIKIPEDIA_PUZZLE).unwrap();
    assert_eq!(grade(&puzzle).score, grade(&puzzle).score);
}

#[test]
fn ladder_eliminations_never_remove_the_truth() {
    let escargot = parse(AI_ESCARGOT).unwrap();
    let truth = solve(&escargot).expect("escargot is solvable");
    let mut cands = Candidates::from_board(&escargot);
    let stall_check = |cands: &Candidates| {
        for idx in 0..CELL_COUNT {
            if !cands.placed[idx]
                && let Cell::Value(truth_v) = truth.get(idx)
            {
                assert!(
                    cands.masks[idx] & (1 << (truth_v - 1)) != 0,
                    "cell {idx} lost true digit {truth_v}"
                );
            }
        }
    };
    loop {
        if cands.placed.iter().all(|&done| done) {
            break;
        }
        if let Some((_, eff)) = suduko_grader::try_all(&cands) {
            if let Effect::Eliminate { removals } = &eff {
                for &(idx, digit) in removals {
                    if let Cell::Value(truth_v) = truth.get(idx) {
                        assert_ne!(digit, truth_v, "eliminated TRUE {digit} at {idx}");
                    }
                }
            }
            suduko_grader::apply(&mut cands, eff);
        } else {
            stall_check(&cands);
            break;
        }
    }
}

#[test]
fn trial_fixture_grades_beyond_the_ladder_within_time_bound() {
    let escargot = parse(AI_ESCARGOT).unwrap();
    let start = std::time::Instant::now();
    let g = grade(&escargot);
    assert!(g.solved, "bounded trial must complete the grid");
    assert_eq!(g.hardest, Some(Technique::Trial));
    assert!(g.score >= 800);
    assert!(start.elapsed().as_millis() < 2_000);
}

#[test]
fn contradictory_board_fails_fast_without_crashing() {
    let mut bad = Board::new();
    bad.set(0, Cell::Value(1));
    bad.set(1, Cell::Value(1));
    let start = std::time::Instant::now();
    let g = grade(&bad);
    assert!(!g.solved, "a contradictory board has no solution");
    assert!(
        start.elapsed().as_millis() < 500,
        "contradiction should be detected quickly"
    );
}
