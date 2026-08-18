use std::time::Instant;

use suduko_engine::format;
use suduko_engine::grid::{Board, Cell};
use suduko_engine::solver;

const WIKIPEDIA_PUZZLE: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const WIKIPEDIA_SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
const AI_ESCARGOT: &str =
    "1....7.9..3..2...8..96..5....53..9...1..8...26....4...3......1..4......7..7...3..";

#[test]
fn solve_finds_the_known_solution() {
    let puzzle = format::parse(WIKIPEDIA_PUZZLE).expect("fixture parses");
    let solved = solver::solve(&puzzle).expect("puzzle is solvable");
    assert_eq!(format::to_string(&solved), WIKIPEDIA_SOLUTION);
}

#[test]
fn solve_is_deterministic() {
    let puzzle = format::parse(WIKIPEDIA_PUZZLE).unwrap();
    assert_eq!(solver::solve(&puzzle), solver::solve(&puzzle));
}

#[test]
fn solve_returns_none_on_contradictory_board() {
    let mut board = Board::new();
    board.set(0, Cell::Value(5));
    board.set(1, Cell::Value(5));
    assert_eq!(solver::solve(&board), None);
}

#[test]
fn solve_of_complete_board_returns_itself() {
    let complete = format::parse(WIKIPEDIA_SOLUTION).unwrap();
    assert_eq!(solver::solve(&complete), Some(complete));
}

#[test]
fn unique_puzzle_counts_exactly_one() {
    let puzzle = format::parse(WIKIPEDIA_PUZZLE).unwrap();
    assert_eq!(solver::count_solutions(&puzzle, 2), 1);
    let escargot = format::parse(AI_ESCARGOT).unwrap();
    assert_eq!(solver::count_solutions(&escargot, 2), 1);
}

#[test]
fn empty_board_is_not_unique() {
    assert_eq!(solver::count_solutions(&Board::new(), 2), 2);
}

#[test]
fn single_clue_board_is_not_unique() {
    let mut board = Board::new();
    board.set(40, Cell::Value(5));
    assert_eq!(solver::count_solutions(&board, 2), 2);
}

#[test]
fn count_solutions_respects_cap_zero() {
    let puzzle = format::parse(WIKIPEDIA_PUZZLE).unwrap();
    assert_eq!(solver::count_solutions(&puzzle, 0), 0);
}

#[test]
fn is_solved_recognizes_complete_consistent_boards() {
    let complete = format::parse(WIKIPEDIA_SOLUTION).unwrap();
    assert!(solver::is_solved(&complete));

    let mut conflicting = complete.clone();
    conflicting.set(1, Cell::Value(1));
    assert!(!solver::is_solved(&conflicting));

    let partial = format::parse(WIKIPEDIA_PUZZLE).unwrap();
    assert!(!solver::is_solved(&partial));
    assert!(!solver::is_solved(&Board::new()));
}

#[test]
fn solver_handles_ai_escargot_within_time_bound() {
    let escargot = format::parse(AI_ESCARGOT).unwrap();
    let start = Instant::now();
    let solution = solver::solve(&escargot).expect("AI Escargot is solvable");
    let elapsed = start.elapsed();
    assert!(solver::is_solved(&solution));
    assert!(
        elapsed.as_millis() < 2_000,
        "solve took {elapsed:?}, expected well under 2s"
    );
}
