use crate::grid::coords::{block_of, col_of, row_of};
use crate::grid::{Board, CELL_COUNT, Cell};

const ALL_DIGITS: u16 = 0x1FF;

pub struct State {
    pub cells: [Cell; CELL_COUNT],
    pub rows: [u16; 9],
    pub cols: [u16; 9],
    pub blocks: [u16; 9],
}

impl State {
    pub fn from_board(board: &Board) -> Self {
        let mut state = State {
            cells: [Cell::Empty; CELL_COUNT],
            rows: [0; 9],
            cols: [0; 9],
            blocks: [0; 9],
        };
        for idx in 0..CELL_COUNT {
            if let Cell::Value(v) = board.get(idx) {
                state.place(idx, v);
            }
        }
        state
    }

    pub fn candidates(&self, idx: usize) -> u16 {
        ALL_DIGITS & !(self.rows[row_of(idx)] | self.cols[col_of(idx)] | self.blocks[block_of(idx)])
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
