//! Row/column/block usage bitmasks shared by solver and generator.

use suduko_grid::{Board, CELL_COUNT, Cell, block_of, col_of, row_of};

pub enum Pick {
    Solved,
    DeadEnd,
    Cell(usize),
}

pub struct State {
    pub cells: [Cell; CELL_COUNT],
    rows: [u16; 9],
    cols: [u16; 9],
    blocks: [u16; 9],
}

impl State {
    #[must_use]
    pub fn from_board(board: &Board) -> Self {
        let mut state = State::empty();
        for idx in 0..CELL_COUNT {
            if let Cell::Value(v) = board.get(idx) {
                state.place(idx, v);
            }
        }
        state
    }

    #[must_use]
    pub fn empty() -> Self {
        State {
            cells: [Cell::Empty; CELL_COUNT],
            rows: [0; 9],
            cols: [0; 9],
            blocks: [0; 9],
        }
    }

    #[must_use]
    pub fn candidates(&self, idx: usize) -> u16 {
        0x1FF & !(self.rows[row_of(idx)] | self.cols[col_of(idx)] | self.blocks[block_of(idx)])
    }

    pub fn place(&mut self, idx: usize, digit: u8) {
        self.cells[idx] = Cell::Value(digit);
        let bit = 1 << (digit - 1);
        self.rows[row_of(idx)] |= bit;
        self.cols[col_of(idx)] |= bit;
        self.blocks[block_of(idx)] |= bit;
    }

    pub fn unplace(&mut self, idx: usize, digit: u8) {
        self.cells[idx] = Cell::Empty;
        let bit = 1 << (digit - 1);
        self.rows[row_of(idx)] &= !bit;
        self.cols[col_of(idx)] &= !bit;
        self.blocks[block_of(idx)] &= !bit;
    }
}

/// Most-constrained-cell selection: fewest candidates first.
#[must_use]
pub fn pick_cell(state: &State) -> Pick {
    let mut best: Option<(usize, u32)> = None;
    for idx in 0..CELL_COUNT {
        if state.cells[idx] != Cell::Empty {
            continue;
        }
        let count = state.candidates(idx).count_ones();
        if count == 0 {
            return Pick::DeadEnd;
        }
        if best.is_none_or(|(_, best_count)| count < best_count) {
            best = Some((idx, count));
        }
    }
    match best {
        None => Pick::Solved,
        Some((idx, _)) => Pick::Cell(idx),
    }
}
