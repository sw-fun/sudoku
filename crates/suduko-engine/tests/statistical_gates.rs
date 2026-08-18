use suduko_engine::difficulty::{self, Level};
use suduko_engine::grader::{self, Technique};
use suduko_engine::solver;

/// Sample count per level. Fixed seeds keep every assertion deterministic;
/// the gate proves band membership, uniqueness, and score separation at the
/// sampled operating point, not a distributional claim.
const SAMPLES: u64 = 5;
const BASE_SEED: u64 = 10_000;

fn levels() -> [(&'static str, Level); 5] {
    [
        ("easy", Level::Easy),
        ("medium", Level::Medium),
        ("hard", Level::Hard),
        ("harder", Level::Harder),
        ("hardest", Level::Hardest),
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

fn sample(level: Level) -> Vec<(suduko_engine::game::Puzzle, grader::Grade)> {
    let offset = match level {
        Level::Easy => 0,
        Level::Medium => 100,
        Level::Hard => 200,
        Level::Harder => 300,
        Level::Hardest => 400,
    };
    (0..SAMPLES)
        .map(|i| {
            let puzzle = difficulty::generate(level, BASE_SEED + offset + i)
                .unwrap_or_else(|e| panic!("{level:?} sample {i} failed: {e:?}"));
            let grade = grader::grade(puzzle.clues());
            (puzzle, grade)
        })
        .collect()
}

#[test]
fn every_sample_is_unique_and_inside_its_band() {
    for &(name, level) in &levels() {
        for (i, (puzzle, grade)) in sample(level).into_iter().enumerate() {
            assert!(grade.solved, "{name} sample {i} must be ladder-solvable");
            let hardest = grade
                .hardest
                .unwrap_or_else(|| panic!("{name} sample {i} has no hardest technique"));
            assert!(
                band_of(level).contains(&hardest),
                "{name} sample {i} hardest was {hardest:?}"
            );
            assert_eq!(
                solver::count_solutions(puzzle.clues(), 2),
                1,
                "{name} sample {i} must have exactly one solution"
            );
            let clues = puzzle.clue_count();
            assert!(
                difficulty::clue_range(level).contains(&clues),
                "{name} sample {i} clues {clues}"
            );
        }
    }
}

#[test]
fn easy_samples_are_singles_only() {
    for (i, (_, grade)) in sample(Level::Easy).into_iter().enumerate() {
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
                grade.counts[technique as usize], 0,
                "easy sample {i} used {technique:?}"
            );
        }
    }
}

#[test]
fn hardest_samples_are_never_singles_solvable() {
    for (i, (_, grade)) in sample(Level::Hardest).into_iter().enumerate() {
        assert!(
            grade.counts[Technique::Trial as usize] > 0,
            "hardest sample {i} never left the ladder"
        );
        let singles = grade.counts[Technique::NakedSingle as usize]
            + grade.counts[Technique::HiddenSingle as usize];
        assert!(
            singles < 60,
            "hardest sample {i} fell to singles almost immediately ({singles})"
        );
    }
}

#[test]
fn mean_scores_strictly_increase_across_levels() {
    let means: Vec<(&str, f64)> = levels()
        .iter()
        .map(|&(name, level)| {
            let scores: Vec<usize> = sample(level).into_iter().map(|(_, g)| g.score).collect();
            let mean = scores.iter().sum::<usize>() as f64 / scores.len() as f64;
            (name, mean)
        })
        .collect();
    for pair in means.windows(2) {
        assert!(
            pair[0].1 < pair[1].1,
            "mean score must increase: {} {:.1} !< {} {:.1}",
            pair[0].0,
            pair[0].1,
            pair[1].0,
            pair[1].1
        );
    }
}
