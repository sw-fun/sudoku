use super::cell::Cell;

pub const CELL_COUNT: usize = 81;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    cells: [Cell; CELL_COUNT],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [Cell::Empty; CELL_COUNT],
        }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.iter().all(Cell::is_empty)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
