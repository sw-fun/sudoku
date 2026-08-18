use crate::grader::candidates::Candidates;
use crate::grader::effect::Effect;
use crate::grid::CELL_COUNT;
use crate::grid::coords::{block_of, col_of, row_of};

/// Locked candidates: pointing (digit confined to one line inside a block,
/// removed from the rest of the line) and claiming (digit confined to one
/// block inside a line, removed from the rest of the block).
pub fn locked_candidates(cands: &Candidates) -> Option<Effect> {
    for block in 0..9usize {
        for digit in 1u8..=9 {
            let bit = 1 << (digit - 1);
            let cells: Vec<usize> = (0..CELL_COUNT)
                .filter(|&idx| block_of(idx) == block && cands.masks[idx] & bit != 0)
                .collect();
            if cells.len() < 2 {
                continue;
            }
            let rows: Vec<usize> = cells.iter().map(|&idx| row_of(idx)).collect();
            let cols: Vec<usize> = cells.iter().map(|&idx| col_of(idx)).collect();
            if rows.iter().all(|&r| r == rows[0]) {
                let removals = line_removals(cands, rows[0], digit, |idx| block_of(idx) != block);
                if !removals.is_empty() {
                    return Some(Effect::Eliminate { removals });
                }
            }
            if cols.iter().all(|&c| c == cols[0]) {
                let removals = col_removals(cands, cols[0], digit, |idx| block_of(idx) != block);
                if !removals.is_empty() {
                    return Some(Effect::Eliminate { removals });
                }
            }
        }
    }
    for line in 0..9usize {
        for digit in 1u8..=9 {
            let bit = 1 << (digit - 1);
            let row_cells: Vec<usize> = (0..9)
                .map(|col| line * 9 + col)
                .filter(|&idx| cands.masks[idx] & bit != 0)
                .collect();
            let blocks: Vec<usize> = row_cells.iter().map(|&idx| block_of(idx)).collect();
            if row_cells.len() >= 2 && blocks.iter().all(|&b| b == blocks[0]) {
                let removals = block_removals(cands, blocks[0], digit, |idx| row_of(idx) != line);
                if !removals.is_empty() {
                    return Some(Effect::Eliminate { removals });
                }
            }
            let col_cells: Vec<usize> = (0..9)
                .map(|row| row * 9 + line)
                .filter(|&idx| cands.masks[idx] & bit != 0)
                .collect();
            let cblocks: Vec<usize> = col_cells.iter().map(|&idx| block_of(idx)).collect();
            if col_cells.len() >= 2 && cblocks.iter().all(|&b| b == cblocks[0]) {
                let removals = block_removals(cands, cblocks[0], digit, |idx| col_of(idx) != line);
                if !removals.is_empty() {
                    return Some(Effect::Eliminate { removals });
                }
            }
        }
    }
    None
}

fn line_removals(
    cands: &Candidates,
    row: usize,
    digit: u8,
    outside: impl Fn(usize) -> bool,
) -> Vec<(usize, u8)> {
    (0..9)
        .map(|col| row * 9 + col)
        .filter(|&idx| {
            !cands.placed[idx] && outside(idx) && cands.masks[idx] & (1 << (digit - 1)) != 0
        })
        .map(|idx| (idx, digit))
        .collect()
}

fn col_removals(
    cands: &Candidates,
    col: usize,
    digit: u8,
    outside: impl Fn(usize) -> bool,
) -> Vec<(usize, u8)> {
    (0..9)
        .map(|row| row * 9 + col)
        .filter(|&idx| {
            !cands.placed[idx] && outside(idx) && cands.masks[idx] & (1 << (digit - 1)) != 0
        })
        .map(|idx| (idx, digit))
        .collect()
}

fn block_removals(
    cands: &Candidates,
    block: usize,
    digit: u8,
    outside: impl Fn(usize) -> bool,
) -> Vec<(usize, u8)> {
    (0..CELL_COUNT)
        .filter(|&idx| {
            block_of(idx) == block
                && !cands.placed[idx]
                && outside(idx)
                && cands.masks[idx] & (1 << (digit - 1)) != 0
        })
        .map(|idx| (idx, digit))
        .collect()
}
