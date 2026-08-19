//! Candidate masks: the shared state every technique operates on.

use suduko_grid::{Board, CELL_COUNT, Cell, peers_of};

/// Bit (digit - 1) set means the digit is still possible for the cell.
#[derive(Clone)]
pub struct Candidates {
    pub masks: [u16; CELL_COUNT],
    pub placed: [bool; CELL_COUNT],
}

impl Candidates {
    #[must_use]
    pub fn from_board(board: &Board) -> Self {
        let mut cands = Candidates::fresh();
        for idx in 0..CELL_COUNT {
            if let Cell::Value(digit) = board.get(idx) {
                cands.place(idx, digit);
            }
        }
        cands
    }

    #[must_use]
    pub fn fresh() -> Self {
        Candidates {
            masks: [0x1FF; CELL_COUNT],
            placed: [false; CELL_COUNT],
        }
    }

    pub fn place(&mut self, idx: usize, digit: u8) {
        let bit = 1 << (digit - 1);
        self.masks[idx] = bit;
        self.placed[idx] = true;
        for &peer in &peers_of(idx) {
            self.masks[peer] &= !bit;
        }
    }
}
