use suduko_engine::format;
use suduko_engine::generator::{self, DigParams};
use suduko_engine::grader::{
    self, Technique, candidates::Candidates, effect, effect::Effect, techniques,
};
use suduko_engine::grid::{CELL_COUNT, Cell};
use suduko_engine::solver;

const WIKIPEDIA_PUZZLE: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const AI_ESCARGOT: &str =
    "1....7.9..3..2...8..96..5....53..9...1..8...26....4...3......1..4......7..7...3..";

#[test]
fn singles_solvable_puzzle_needs_only_singles() {
    let puzzle = format::parse(WIKIPEDIA_PUZZLE).unwrap();
    let grade = grader::grade(&puzzle);
    assert!(grade.solved, "the ladder must solve a singles puzzle");
    for technique in [
        Technique::LockedCandidates,
        Technique::NakedSet,
        Technique::HiddenSet,
        Technique::XWing,
        Technique::Swordfish,
        Technique::Trial,
    ] {
        assert_eq!(
            grade.counts[technique as usize], 0,
            "{technique:?} must never fire on a singles-solvable puzzle"
        );
    }
    assert!(
        grade.hardest <= Some(Technique::HiddenSingle),
        "hardest technique was {:?}",
        grade.hardest
    );
    assert!(
        grade.score < 300,
        "score stays in the singles band: {}",
        grade.score
    );
}

#[test]
fn locked_candidates_fixture() {
    let puzzle = generator::generate_puzzle(
        15,
        DigParams {
            target_clues: 28,
            symmetric: false,
        },
    );
    assert_eq!(puzzle.clue_count(), 28);
    let grade = grader::grade(puzzle.clues());
    assert_eq!(grade.hardest, Some(Technique::LockedCandidates));
    assert!(grade.solved);
}

#[test]
fn naked_set_fixture() {
    let puzzle = generator::generate_puzzle(
        24,
        DigParams {
            target_clues: 26,
            symmetric: false,
        },
    );
    let grade = grader::grade(puzzle.clues());
    assert_eq!(grade.hardest, Some(Technique::NakedSet));
    assert!(grade.solved);
}

#[test]
fn hidden_subsets_are_shadowed_by_complementary_naked_sets() {
    // A hidden k-subset in a unit implies a naked (m-k)-subset on the
    // complement, and the ladder tries naked sets first. Across 800 seeded
    // generations at dig depths 22-28 no puzzle required HiddenSet, so the
    // technique is pinned by the crafted-mask tests in grader_techniques.rs
    // (correct firing and degenerate non-firing) rather than by a generated
    // fixture. This test pins the shadowing property itself: for the subset
    // fixtures above, HiddenSet never fires.
    for (seed, target) in [(24u64, 26usize), (15, 28)] {
        let puzzle = generator::generate_puzzle(
            seed,
            DigParams {
                target_clues: target,
                symmetric: false,
            },
        );
        let grade = grader::grade(puzzle.clues());
        assert_eq!(grade.counts[Technique::HiddenSet as usize], 0);
    }
}

#[test]
fn trial_fixture_grades_beyond_the_ladder() {
    let escargot = format::parse(AI_ESCARGOT).unwrap();
    let grade = grader::grade(&escargot);
    assert!(grade.solved, "bounded trial must complete the grid");
    assert_eq!(grade.hardest, Some(Technique::Trial));
    assert!(grade.score >= 800, "trial band: {}", grade.score);
}

#[test]
fn trial_fixture_grades_within_time_bound() {
    let escargot = format::parse(AI_ESCARGOT).unwrap();
    let start = std::time::Instant::now();
    grader::grade(&escargot);
    assert!(start.elapsed().as_millis() < 2_000);
}

#[test]
fn grading_is_deterministic() {
    let puzzle = format::parse(WIKIPEDIA_PUZZLE).unwrap();
    assert_eq!(grader::grade(&puzzle).score, grader::grade(&puzzle).score);
}

#[test]
fn ladder_eliminations_never_remove_the_truth() {
    // Walk the ladder over AI Escargot and verify no elimination ever removes
    // the actual solution digit of a cell. Regression for a hidden-subset
    // bug that eliminated a true candidate when a subset digit was missing.
    let escargot = format::parse(AI_ESCARGOT).unwrap();
    let truth = solver::solve(&escargot).expect("escargot is solvable");
    let mut cands = Candidates::from_board(&escargot);
    loop {
        if cands.placed.iter().all(|&done| done) {
            break;
        }
        match techniques::try_all(&cands) {
            Some((_, eff)) => {
                if let Effect::Eliminate { removals } = &eff {
                    for &(idx, digit) in removals {
                        if let Cell::Value(truth_v) = truth.get(idx) {
                            assert_ne!(
                                digit, truth_v,
                                "eliminated TRUE digit {digit} at cell {idx}"
                            );
                        }
                    }
                }
                effect::apply(&mut cands, eff);
            }
            None => {
                for idx in 0..CELL_COUNT {
                    if !cands.placed[idx] {
                        if let Cell::Value(truth_v) = truth.get(idx) {
                            assert!(
                                cands.masks[idx] & (1 << (truth_v - 1)) != 0,
                                "cell {idx} lost true digit {truth_v}"
                            );
                        }
                    }
                }
                break;
            }
        }
    }
}
