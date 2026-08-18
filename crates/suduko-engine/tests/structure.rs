use suduko_engine::grid::{Board, CELL_COUNT, Cell};

#[test]
fn engine_exposes_grid_model_types() {
    let board = Board::new();
    assert_eq!(CELL_COUNT, 81, "a board holds 81 cells");
    assert_eq!(board.get(0), Cell::Empty);
}

#[test]
fn engine_cell_covers_empty_and_digits() {
    assert!(Cell::Empty.is_empty());
    for digit in 1..=9 {
        let cell = Cell::Value(digit);
        assert_eq!(cell.value(), Some(digit));
    }
}
