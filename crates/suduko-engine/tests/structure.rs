use suduko_engine::{Board, Cell};

#[test]
fn engine_exposes_grid_model_types() {
    let board = Board::new();
    assert_eq!(board.len(), 81, "a board holds 81 cells");
}

#[test]
fn engine_cell_covers_empty_and_digits() {
    assert!(Cell::Empty.is_empty());
    for digit in 1..=9 {
        let cell = Cell::Value(digit);
        assert_eq!(cell.value(), Some(digit));
    }
}
