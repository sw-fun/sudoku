use crate::grid::coords::peers_of;
use crate::grid::{Board, CELL_COUNT, Cell};

pub const ALL_DIGITS: u16 = 0x1FF;

/// Candidate masks for all 81 cells plus which cells are already placed.
/// Bit (digit - 1) set means the digit is still possible for the cell.
#[derive(Clone)]
pub struct Candidates {
    pub masks: [u16; CELL_COUNT],
    pub placed: [bool; CELL_COUNT],
}

impl Candidates {
    pub fn from_board(board: &Board) -> Self {
        let mut cands = Candidates {
            masks: [ALL_DIGITS; CELL_COUNT],
            placed: [false; CELL_COUNT],
        };
        for idx in 0..CELL_COUNT {
            if let Cell::Value(digit) = board.get(idx) {
                cands.place(idx, digit);
            }
        }
        cands
    }

    pub fn from_masks(masks: [u16; CELL_COUNT], placed: [bool; CELL_COUNT]) -> Self {
        Candidates { masks, placed }
    }

    pub fn place(&mut self, idx: usize, digit: u8) {
        let bit = 1 << (digit - 1);
        self.masks[idx] = bit;
        self.placed[idx] = true;
        for &peer in &peers_of(idx) {
            self.masks[peer] &= !bit;
        }
    }

    /// Removes `digit` from cell `idx` unless the cell is solved. Returns
    /// whether anything changed.
    pub fn eliminate(&mut self, idx: usize, digit: u8) -> bool {
        if self.placed[idx] || self.masks[idx].count_ones() <= 1 {
            return false;
        }
        let bit = 1 << (digit - 1);
        if self.masks[idx] & bit == 0 {
            return false;
        }
        self.masks[idx] &= !bit;
        true
    }
}
