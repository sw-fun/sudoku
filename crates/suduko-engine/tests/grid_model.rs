use suduko_engine::game::Puzzle;
use suduko_engine::grid::{Board, CELL_COUNT, Cell};
use suduko_engine::{block_of, col_of, format, peers_of, row_of, validate};

fn board_with(entries: &[(usize, u8)]) -> Board {
    let mut board = Board::new();
    for &(idx, v) in entries {
        board.set(idx, Cell::Value(v));
    }
    board
}

#[test]
fn index_helpers_map_corner_center_and_last_cells() {
    assert_eq!(row_of(0), 0);
    assert_eq!(col_of(0), 0);
    assert_eq!(block_of(0), 0);
    assert_eq!((row_of(8), col_of(8), block_of(8)), (0, 8, 2));
    assert_eq!((row_of(9), col_of(9), block_of(9)), (1, 0, 0));
    assert_eq!((row_of(80), col_of(80), block_of(80)), (8, 8, 8));
    assert_eq!((row_of(40), col_of(40), block_of(40)), (4, 4, 4));
}

#[test]
fn peers_of_corner_has_twenty_symmetric_members() {
    let peers = peers_of(0);
    assert_eq!(peers.len(), 20);
    assert!(!peers.contains(&0));
    for &p in &peers {
        assert!(p < CELL_COUNT);
        assert!(peers_of(p).contains(&0), "peer relation is symmetric");
    }
    for i in 0..9 {
        if i != 0 {
            assert!(peers.contains(&i), "same row: {i}");
        }
    }
    assert!(peers.contains(&9));
    assert!(peers.contains(&72));
    assert!(peers.contains(&10), "same block: 10");
    assert!(peers.contains(&19), "same block: 19");
    assert!(!peers.contains(&12), "r1c3 shares nothing with r0c0");
    assert!(!peers.contains(&28), "r3c1 shares nothing with r0c0");
}

#[test]
fn peers_of_center_covers_row_column_and_block() {
    let peers = peers_of(40); // r4,c4,b4
    for i in 0..9 {
        let row_cell = 4 * 9 + i;
        let col_cell = i * 9 + 4;
        if row_cell != 40 {
            assert!(peers.contains(&row_cell));
        }
        if col_cell != 40 {
            assert!(peers.contains(&col_cell));
        }
    }
    for &b in &[30, 31, 32, 39, 41, 48, 49, 50] {
        assert!(peers.contains(&b), "block peer {b}");
    }
    assert_eq!(peers.len(), 20);
}

#[test]
fn row_duplicate_is_reported() {
    let board = board_with(&[(0, 5), (8, 5)]);
    assert_eq!(validate::first_conflict(&board), Some((8, 5)));
}

#[test]
fn column_duplicate_is_reported() {
    let board = board_with(&[(0, 3), (72, 3)]);
    assert!(validate::first_conflict(&board).is_some());
}

#[test]
fn block_duplicate_is_reported() {
    let board = board_with(&[(0, 7), (10, 7)]);
    assert!(validate::first_conflict(&board).is_some());
}

#[test]
fn consistent_partial_and_full_boards_pass() {
    let partial = board_with(&[
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 8),
        (8, 9),
    ]);
    assert_eq!(validate::first_conflict(&partial), None);

    let full = format::parse(
        "123456789456789123789123456214365897365897214897214365531642978642978531978531642",
    )
    .expect("valid full board string");
    assert_eq!(validate::first_conflict(&full), None);
}

#[test]
fn serialization_round_trips_empty_and_mixed_boards() {
    let empty = Board::new();
    assert_eq!(format::to_string(&empty), ".".repeat(81));
    assert_eq!(format::parse(&format::to_string(&empty)).unwrap(), empty);

    let mixed = board_with(&[(0, 9), (40, 1), (80, 5)]);
    assert_eq!(format::parse(&format::to_string(&mixed)).unwrap(), mixed);
}

#[test]
fn parse_accepts_zero_as_empty_marker() {
    let board = format::parse(&format!("0{}", ".".repeat(80))).unwrap();
    assert_eq!(board.get(0), Cell::Empty);
}

#[test]
fn parse_rejects_malformed_strings() {
    assert!(format::parse("").is_err());
    assert!(format::parse(&"1".repeat(80)).is_err());
    assert!(format::parse(&"1".repeat(82)).is_err());
    assert!(format::parse(&"x".repeat(81)).is_err());
    assert!(format::parse(&".".repeat(80)).is_err());
}

const VALID_SOLUTION: &str =
    "123456789456789123789123456214365897365897214897214365531642978642978531978531642";

#[test]
fn puzzle_accepts_matching_clues_and_complete_solution() {
    let solution = format::parse(VALID_SOLUTION).unwrap();
    let mut clues = solution.clone();
    for idx in [0, 1, 2, 9, 10, 20, 40, 60, 80] {
        clues.set(idx, Cell::Empty);
    }
    let puzzle = Puzzle::new(clues, solution.clone()).expect("valid puzzle");
    assert_eq!(puzzle.solution(), &solution);
    assert_eq!(puzzle.clue_count(), 72);
}

#[test]
fn puzzle_rejects_incomplete_solution() {
    let mut solution = format::parse(VALID_SOLUTION).unwrap();
    solution.set(0, Cell::Empty);
    let clues = Board::new();
    assert!(Puzzle::new(clues, solution).is_err());
}

#[test]
fn puzzle_rejects_clue_mismatch() {
    let solution = format::parse(VALID_SOLUTION).unwrap();
    let mut clues = solution.clone();
    clues.set(0, Cell::Value(9));
    assert!(Puzzle::new(clues, solution).is_err());
}

#[test]
fn puzzle_rejects_inconsistent_solution() {
    let mut solution = format::parse(VALID_SOLUTION).unwrap();
    solution.set(1, Cell::Value(1));
    let clues = Board::new();
    assert!(Puzzle::new(clues, solution).is_err());
}
