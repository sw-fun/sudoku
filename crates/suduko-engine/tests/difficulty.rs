use suduko_engine::difficulty::{self, Level};
use suduko_engine::grader::{self, Technique};
use suduko_engine::solver;

fn band_of(level: Level) -> &'static [Technique] {
    match level {
        Level::Easy => &[Technique::NakedSingle, Technique::HiddenSingle],
        Level::Medium => &[Technique::LockedCandidates],
        Level::Hard => &[Technique::NakedSet, Technique::HiddenSet],
        Level::Harder => &[Technique::XYWing, Technique::XWing, Technique::Swordfish],
        Level::Hardest => &[Technique::Trial],
    }
}

#[test]
fn every_level_returns_a_unique_puzzle_inside_its_band() {
    let cases = [
        (Level::Easy, 1u64),
        (Level::Medium, 2),
        (Level::Hard, 3),
        (Level::Harder, 4),
        (Level::Hardest, 5),
    ];
    for &(level, seed) in &cases {
        let puzzle = difficulty::generate(level, seed).expect("level is reachable");
        let grade = grader::grade(puzzle.clues());
        assert!(
            grade.solved,
            "{level:?} puzzle must be solvable by the ladder"
        );
        assert!(
            band_of(level).contains(&grade.hardest.expect("graded")),
            "{level:?} hardest was {:?}",
            grade.hardest
        );
        assert_eq!(
            solver::count_solutions(puzzle.clues(), 2),
            1,
            "{level:?} puzzle must stay unique"
        );
        let clues = puzzle.clue_count();
        assert!(
            difficulty::clue_range(level).contains(&clues),
            "{level:?} clues {clues} outside {:?}",
            difficulty::clue_range(level)
        );
    }
}

#[test]
fn generation_is_deterministic_per_seed() {
    for level in [
        Level::Easy,
        Level::Medium,
        Level::Hard,
        Level::Harder,
        Level::Hardest,
    ] {
        let a = difficulty::generate(level, 77).expect("reachable");
        let b = difficulty::generate(level, 77).expect("reachable");
        assert_eq!(
            suduko_engine::format::to_string(a.clues()),
            suduko_engine::format::to_string(b.clues()),
            "{level:?} must be deterministic"
        );
    }
}

#[test]
fn zero_attempts_fail_closed_without_panicking() {
    let result = difficulty::generate_bounded(Level::Easy, 9, 0, 0);
    assert!(matches!(
        result,
        Err(difficulty::LevelError::Exhausted { attempts: 0 })
    ));
}

#[test]
fn no_dig_attempts_means_untouched_grid_is_rejected() {
    // One full grid, zero dig passes: the 81-clue board cannot be in any
    // band, so the loop must exhaust gracefully and quickly.
    let start = std::time::Instant::now();
    let result = difficulty::generate_bounded(Level::Hardest, 123, 0, 2);
    assert!(result.is_err());
    assert!(start.elapsed().as_secs() < 2);
}

#[test]
fn levels_are_strictly_separated_by_band() {
    // Spot-check ordering: the max weight of each band grows with the level.
    let weights: Vec<usize> = [
        Level::Easy,
        Level::Medium,
        Level::Hard,
        Level::Harder,
        Level::Hardest,
    ]
    .iter()
    .map(|&level| band_of(level).iter().map(|t| t.weight()).max().unwrap())
    .collect();
    let mut sorted = weights.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(weights, sorted, "band maxima must strictly increase");
}
