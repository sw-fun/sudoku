use suduko_engine::Technique;
use suduko_engine::{Level, LevelError, clue_range, generate, generate_bounded};
use suduko_engine::{count_solutions, grade};

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
    for (level, seed) in [
        (Level::Easy, 1u64),
        (Level::Medium, 2),
        (Level::Hard, 3),
        (Level::Harder, 4),
        (Level::Hardest, 5),
    ] {
        let puzzle = generate(level, seed).expect("level is reachable");
        let g = grade(puzzle.clues());
        assert!(g.solved, "{level:?} must be ladder-solvable");
        assert!(
            band_of(level).contains(&g.hardest.expect("graded")),
            "{level:?} hardest was {:?}",
            g.hardest
        );
        assert_eq!(count_solutions(puzzle.clues(), 2), 1);
        assert!(clue_range(level).contains(&puzzle.clue_count()));
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
        let a = generate(level, 77).expect("reachable");
        let b = generate(level, 77).expect("reachable");
        assert_eq!(
            suduko_engine::to_string(a.clues()),
            suduko_engine::to_string(b.clues()),
            "{level:?} deterministic"
        );
    }
}

#[test]
fn exhausted_budgets_fail_closed() {
    assert!(matches!(
        generate_bounded(Level::Easy, 9, 0, 0),
        Err(LevelError::Exhausted { attempts: 0 })
    ));
    let start = std::time::Instant::now();
    assert!(generate_bounded(Level::Hardest, 123, 0, 2).is_err());
    assert!(start.elapsed().as_secs() < 2);
}
