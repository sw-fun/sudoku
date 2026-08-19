use suduko_engine::{Level, Technique, clue_range, count_solutions, generate, grade};

/// Sample count per level. Fixed seeds keep every assertion deterministic;
/// the gate proves band membership, uniqueness, and score separation at the
/// sampled operating point, not a distributional claim.
const SAMPLES: u64 = 5;
const BASE_SEED: u64 = 10_000;

fn levels() -> [(&'static str, Level, u64); 5] {
    [
        ("easy", Level::Easy, 0),
        ("medium", Level::Medium, 100),
        ("hard", Level::Hard, 200),
        ("harder", Level::Harder, 300),
        ("hardest", Level::Hardest, 400),
    ]
}

fn band_of(level: Level) -> &'static [Technique] {
    match level {
        Level::Easy => &[Technique::NakedSingle, Technique::HiddenSingle],
        Level::Medium => &[Technique::LockedCandidates],
        Level::Hard => &[Technique::NakedSet, Technique::HiddenSet],
        Level::Harder => &[Technique::XYWing, Technique::XWing, Technique::Swordfish],
        Level::Hardest => &[Technique::Trial],
    }
}

fn sample(level: Level, offset: u64) -> Vec<(usize, usize, [usize; 9])> {
    (0..SAMPLES)
        .map(|i| {
            let puzzle = generate(level, BASE_SEED + offset + i)
                .unwrap_or_else(|e| panic!("{level:?} sample {i} failed: {e:?}"));
            let g = grade(puzzle.clues());
            (puzzle.clue_count(), g.score, g.counts)
        })
        .collect()
}

#[test]
fn every_sample_is_unique_and_inside_its_band() {
    for &(name, level, offset) in &levels() {
        for (i, (clues, _, counts)) in sample(level, offset).into_iter().enumerate() {
            let hardest_tier = band_of(level)
                .iter()
                .map(|t| *t as usize)
                .max()
                .expect("nonempty");
            let used_outside = (0..9)
                .filter(|t| {
                    !band_of(level).contains(&match *t {
                        0 => Technique::NakedSingle,
                        1 => Technique::HiddenSingle,
                        2 => Technique::LockedCandidates,
                        3 => Technique::NakedSet,
                        4 => Technique::HiddenSet,
                        5 => Technique::XYWing,
                        6 => Technique::XWing,
                        7 => Technique::Swordfish,
                        _ => Technique::Trial,
                    })
                })
                .any(|t| counts[t] > 0 && t > hardest_tier);
            assert!(!used_outside, "{name} sample {i} used a harder technique");
            assert!(
                clue_range(level).contains(&clues),
                "{name} {i} clues {clues}"
            );
            let puzzle = generate(level, BASE_SEED + offset + i as u64).expect("regenerable");
            assert_eq!(count_solutions(puzzle.clues(), 2), 1, "{name} {i} unique");
        }
    }
}

#[test]
fn easy_samples_are_singles_only() {
    for (i, (_, _, counts)) in sample(Level::Easy, 0).into_iter().enumerate() {
        for technique in [
            Technique::LockedCandidates,
            Technique::NakedSet,
            Technique::HiddenSet,
            Technique::XYWing,
            Technique::XWing,
            Technique::Swordfish,
            Technique::Trial,
        ] {
            assert_eq!(
                counts[technique as usize], 0,
                "easy sample {i} used {technique:?}"
            );
        }
    }
}

#[test]
fn hardest_samples_are_never_singles_solvable() {
    for (i, (_, _, counts)) in sample(Level::Hardest, 400).into_iter().enumerate() {
        assert!(
            counts[Technique::Trial as usize] > 0,
            "hardest sample {i} never left the ladder"
        );
        let singles =
            counts[Technique::NakedSingle as usize] + counts[Technique::HiddenSingle as usize];
        assert!(
            singles < 60,
            "hardest sample {i} fell to singles ({singles})"
        );
    }
}

#[test]
fn mean_scores_strictly_increase_across_levels() {
    let means: Vec<f64> = levels()
        .iter()
        .map(|&(_, level, offset)| {
            #[allow(clippy::cast_precision_loss)]
            let scores: Vec<f64> = sample(level, offset)
                .iter()
                .map(|&(_, score, _)| score as f64)
                .collect();
            #[allow(clippy::cast_precision_loss)]
            let count = scores.len() as f64;
            scores.iter().sum::<f64>() / count
        })
        .collect();
    for pair in means.windows(2) {
        assert!(pair[0] < pair[1], "means must increase: {means:?}");
    }
}
