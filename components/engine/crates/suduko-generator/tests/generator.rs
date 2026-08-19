use std::time::Instant;

use suduko_generator::{DigParams, Rng, dig, generate_full};
use suduko_grid::first_conflict;
use suduko_grid::{CELL_COUNT, Cell};
use suduko_solver::{count_solutions, solve};

fn params(target_clues: usize, symmetric: bool) -> DigParams {
    DigParams {
        target_clues,
        symmetric,
    }
}

#[test]
fn rng_golden_stream_is_pinned() {
    let mut rng = Rng::new(0);
    assert_eq!(
        rng.next_u64(),
        0xE220_A839_7B1D_CDAF,
        "SplitMix64 golden for seed 0"
    );
    assert_eq!(rng.next_u64(), 0x6E78_9E6A_A1B9_65F4);
}

#[test]
fn full_grid_is_complete_valid_and_deterministic() {
    let full = generate_full(42);
    assert_eq!(count_solutions(&full, 1), 1, "must be a valid solution");
    assert_eq!(generate_full(42), generate_full(42));
    assert_ne!(generate_full(1), generate_full(2));
}

#[test]
fn dug_puzzle_keeps_exactly_one_solution_at_lenient_targets() {
    let full = generate_full(42);
    let mut rng = Rng::new(9);
    for target in [55, 50, 40] {
        let clues = dig(&full, params(target, false), &mut rng);
        assert_eq!(count_solutions(&clues, 2), 1);
        assert!(first_conflict(&clues).is_none());
        let given: usize = (0..CELL_COUNT)
            .filter(|&i| clues.get(i) != Cell::Empty)
            .count();
        assert_eq!(given, target, "digging should reach {target}");
    }
}

#[test]
fn symmetric_digging_is_point_symmetric_and_unique() {
    let full = generate_full(5);
    let clues = dig(&full, params(45, true), &mut Rng::new(7));
    for idx in 0..CELL_COUNT {
        assert_eq!(
            clues.get(idx).is_empty(),
            clues.get(CELL_COUNT - 1 - idx).is_empty(),
            "cell {idx} must mirror its partner"
        );
    }
    assert_eq!(count_solutions(&clues, 2), 1);
}

#[test]
fn unreachable_target_still_terminates_with_valid_puzzle() {
    let full = generate_full(11);
    let start = Instant::now();
    let clues = dig(&full, params(17, false), &mut Rng::new(3));
    let given: usize = (0..CELL_COUNT)
        .filter(|&i| clues.get(i) != Cell::Empty)
        .count();
    assert!(
        (17..=35).contains(&given),
        "stopped when load-bearing: {given}"
    );
    assert_eq!(count_solutions(&clues, 2), 1);
    assert!(
        start.elapsed().as_secs() < 5,
        "generation must stay bounded"
    );
    let _ = solve(&clues);
}
