use crate::grid::coords::{block_of, col_of, row_of};
use crate::grid::{Board, Cell};

/// Returns the first cell (in row-major scan order) whose value duplicates a
/// value already seen in its row, column, or block, as `(index, value)`.
pub fn first_conflict(board: &Board) -> Option<(usize, u8)> {
    let mut row_seen = [[false; 9]; 9];
    let mut col_seen = [[false; 9]; 9];
    let mut block_seen = [[false; 9]; 9];
    for idx in 0..crate::CELL_COUNT {
        if let Cell::Value(v) = board.get(idx) {
            let (r, c, b) = (row_of(idx), col_of(idx), block_of(idx));
            let d = (v - 1) as usize;
            if row_seen[r][d] || col_seen[c][d] || block_seen[b][d] {
                return Some((idx, v));
            }
            row_seen[r][d] = true;
            col_seen[c][d] = true;
            block_seen[b][d] = true;
        }
    }
    None
}
