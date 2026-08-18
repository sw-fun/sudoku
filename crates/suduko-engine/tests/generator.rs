use std::time::Instant;

use suduko_engine::generator::{self, DigParams};
use suduko_engine::grid::{Board, CELL_COUNT, Cell};
use suduko_engine::solver;
use suduko_engine::validate;

fn params(target_clues: usize, symmetric: bool) -> DigParams {
    DigParams {
        target_clues,
        symmetric,
    }
}

#[test]
fn full_grid_is_complete_and_valid() {
    let full = generator::generate_full(42);
    assert!(
        solver::is_solved(&full),
        "generated grid must be a valid solution"
    );
}

#[test]
fn full_grid_generation_is_deterministic_per_seed() {
    assert_eq!(generator::generate_full(42), generator::generate_full(42));
    assert_eq!(generator::generate_full(7), generator::generate_full(7));
    assert_ne!(
        generator::generate_full(1),
        generator::generate_full(2),
        "different seeds should produce different grids"
    );
}

#[test]
fn dug_puzzle_keeps_exactly_one_solution() {
    let puzzle = generator::generate_puzzle(42, params(40, false));
    assert_eq!(solver::count_solutions(puzzle.clues(), 2), 1);
    assert!(validate::first_conflict(puzzle.clues()).is_none());
}

#[test]
fn target_clues_is_honored_for_lenient_targets() {
    for target in [55, 50, 40] {
        let puzzle = generator::generate_puzzle(99, params(target, false));
        assert_eq!(puzzle.clue_count(), target, "digging should reach {target}");
    }
}

#[test]
fn symmetric_digging_is_point_symmetric() {
    let puzzle = generator::generate_puzzle(5, params(45, true));
    let clues = puzzle.clues();
    for idx in 0..CELL_COUNT {
        assert_eq!(
            clues.get(idx).is_empty(),
            clues.get(CELL_COUNT - 1 - idx).is_empty(),
            "cell {idx} must mirror its point-symmetric partner"
        );
    }
    assert_eq!(solver::count_solutions(clues, 2), 1);
}

#[test]
fn unreachable_target_still_terminates_with_valid_puzzle() {
    let start = Instant::now();
    let puzzle = generator::generate_puzzle(11, params(17, false));
    let elapsed = start.elapsed();
    let clues = puzzle.clue_count();
    assert!(
        (17..=35).contains(&clues),
        "digging stops when no removable clue remains, got {clues}"
    );
    assert_eq!(solver::count_solutions(puzzle.clues(), 2), 1);
    assert!(
        elapsed.as_secs() < 5,
        "generation must stay bounded, took {elapsed:?}"
    );
}

#[test]
fn puzzle_clues_agree_with_stored_solution() {
    let puzzle = generator::generate_puzzle(1234, params(40, false));
    for idx in 0..CELL_COUNT {
        match puzzle.clues().get(idx) {
            Cell::Empty => {}
            Cell::Value(v) => {
                assert_eq!(puzzle.solution().get(idx), Cell::Value(v));
            }
        }
    }
}

#[test]
fn puzzle_generation_is_deterministic_per_seed() {
    let a = generator::generate_puzzle(77, params(40, false));
    let b = generator::generate_puzzle(77, params(40, false));
    assert_eq!(format::to_string(a.clues()), format::to_string(b.clues()));
    assert_eq!(
        format::to_string(a.solution()),
        format::to_string(b.solution())
    );
}

#[test]
fn generation_completes_quickly_at_playable_depths() {
    let start = Instant::now();
    generator::generate_puzzle(2026, params(35, false));
    assert!(
        start.elapsed().as_secs() < 5,
        "typical generation should be fast"
    );
}

use suduko_engine::format;
#[test]
fn rng_golden_stream_is_pinned() {
    let mut rng = suduko_engine::rng::Rng::new(0);
    let first = rng.next_u64();
    let second = rng.next_u64();
    assert_eq!(
        first, 0xE220_A839_7B1D_CDAF,
        "SplitMix64 golden value for seed 0"
    );
    assert_eq!(second, 0x6E78_9E6A_A1B9_65F4);
}
