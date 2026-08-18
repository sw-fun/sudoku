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

    /// Returns the cell at `idx` (0..81), row-major. Panics if out of range.
    pub fn get(&self, idx: usize) -> Cell {
        self.cells[idx]
    }

    /// Sets the cell at `idx` (0..81), row-major. Panics if out of range.
    pub fn set(&mut self, idx: usize, cell: Cell) {
        self.cells[idx] = cell;
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
