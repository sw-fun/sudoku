//! Cell and board types.

/// A board cell: empty or a value in 1..=9 (documented invariant).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Value(u8),
}

impl Cell {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    #[must_use]
    pub fn value(&self) -> Option<u8> {
        match self {
            Cell::Empty => None,
            Cell::Value(v) => Some(*v),
        }
    }
}

pub const CELL_COUNT: usize = 81;

/// Row-major 81-cell board. Values stay in 1..=9 or `Empty`; `get`/`set`
/// panic on out-of-range indices by design (indices are always engine-made).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    cells: [Cell; CELL_COUNT],
}

impl Board {
    #[must_use]
    pub fn new() -> Self {
        Board {
            cells: [Cell::Empty; CELL_COUNT],
        }
    }

    #[must_use]
    pub fn from_cells(cells: [Cell; CELL_COUNT]) -> Self {
        Board { cells }
    }

    #[must_use]
    pub fn get(&self, idx: usize) -> Cell {
        self.cells[idx]
    }

    pub fn set(&mut self, idx: usize, cell: Cell) {
        self.cells[idx] = cell;
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
