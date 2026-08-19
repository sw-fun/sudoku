use suduko_engine::{Level, generate};

/// Seeds that exhausted the 24x3 caps in the acceptance 100-seed sweep
/// (docs/engine-acceptance.md). They must accept after hardening.
#[test]
fn recorded_exhausting_seeds_now_accept() {
    for &(level, seed) in &[
        (Level::Medium, 51_063u64),
        (Level::Hard, 52_007),
        (Level::Hard, 52_021),
        (Level::Hard, 52_033),
        (Level::Hard, 52_042),
        (Level::Hard, 52_073),
        (Level::Hard, 52_086),
        (Level::Harder, 53_001),
        (Level::Harder, 53_013),
        (Level::Harder, 53_042),
        (Level::Harder, 53_055),
        (Level::Harder, 53_056),
        (Level::Harder, 53_060),
        (Level::Harder, 53_096),
    ] {
        let start = std::time::Instant::now();
        let puzzle = generate(level, seed)
            .unwrap_or_else(|e| panic!("{level:?} seed {seed} still exhausts: {e:?}"));
        assert!(
            start.elapsed().as_secs() < 5,
            "{level:?} seed {seed} took {:?}, expected under 5s",
            start.elapsed()
        );
        assert!(suduko_engine::count_solutions(puzzle.clues(), 2) == 1);
    }
}
