use std::time::Instant;

use suduko_grid::parse;
use suduko_solver::{count_solutions, solve};

const WIKIPEDIA_PUZZLE: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const WIKIPEDIA_SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
const AI_ESCARGOT: &str =
    "1....7.9..3..2...8..96..5....53..9...1..8...26....4...3......1..4......7..7...3..";

#[test]
fn solve_finds_the_known_solution_and_is_deterministic() {
    let puzzle = parse(WIKIPEDIA_PUZZLE).expect("fixture parses");
    let solved = solve(&puzzle).expect("puzzle is solvable");
    assert_eq!(suduko_grid::to_string(&solved), WIKIPEDIA_SOLUTION);
    assert_eq!(solve(&puzzle), solve(&puzzle));
}

#[test]
fn solve_handles_degenerate_boards() {
    let mut board = suduko_grid::Board::new();
    board.set(0, suduko_grid::Cell::Value(5));
    board.set(1, suduko_grid::Cell::Value(5));
    assert_eq!(solve(&board), None);
    let complete = parse(WIKIPEDIA_SOLUTION).unwrap();
    assert_eq!(solve(&complete), Some(complete));
}

#[test]
fn uniqueness_counting_decides_unique_and_ambiguous() {
    let puzzle = parse(WIKIPEDIA_PUZZLE).unwrap();
    assert_eq!(count_solutions(&puzzle, 2), 1);
    let escargot = parse(AI_ESCARGOT).unwrap();
    assert_eq!(count_solutions(&escargot, 2), 1);
    assert_eq!(count_solutions(&suduko_grid::Board::new(), 2), 2);
    let mut single = suduko_grid::Board::new();
    single.set(40, suduko_grid::Cell::Value(5));
    assert_eq!(count_solutions(&single, 2), 2);
    assert_eq!(count_solutions(&puzzle, 0), 0);
}

#[test]
fn solver_handles_ai_escargot_within_time_bound() {
    let escargot = parse(AI_ESCARGOT).unwrap();
    let start = Instant::now();
    let solution = solve(&escargot).expect("AI Escargot is solvable");
    assert_eq!(count_solutions(&solution, 2), 1);
    assert!(
        start.elapsed().as_millis() < 2_000,
        "solve took {:?}, expected well under 2s",
        start.elapsed()
    );
}
