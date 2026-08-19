//! Locked candidates: pointing and claiming.

use crate::{Candidates, Effect};
use suduko_grid::{CELL_COUNT, block_of, col_of, row_of};

/// Pointing: a digit confined to one line inside a block is removed from
/// the rest of that line. Claiming: a digit confined to one block inside a
/// line is removed from the rest of that block.
#[must_use]
pub fn locked_candidates(cands: &Candidates) -> Option<Effect> {
    pointing(cands).or_else(|| claiming(cands))
}

fn pointing(cands: &Candidates) -> Option<Effect> {
    for block in 0..9usize {
        for digit in 1u8..=9 {
            let bit = 1 << (digit - 1);
            let cells: Vec<usize> = (0..CELL_COUNT)
                .filter(|&i| block_of(i) == block && cands.masks[i] & bit != 0)
                .collect();
            for same_row in [true, false] {
                if let Some(removals) = line_removals(cands, &cells, same_row, block, digit, bit) {
                    return Some(Effect::Eliminate { removals });
                }
            }
        }
    }
    None
}

/// When the block-confined cells of a digit share one line, remove the
/// digit from the rest of that line outside the block.
fn line_removals(
    cands: &Candidates,
    cells: &[usize],
    same_row: bool,
    block: usize,
    digit: u8,
    bit: u16,
) -> Option<Vec<(usize, u8)>> {
    let keys: Vec<usize> = cells
        .iter()
        .map(|&i| if same_row { row_of(i) } else { col_of(i) })
        .collect();
    if keys.len() < 2 || !keys.iter().all(|&k| k == keys[0]) {
        return None;
    }
    let line = keys[0];
    let hits: Vec<(usize, u8)> = (0..9)
        .map(|k| if same_row { line * 9 + k } else { k * 9 + line })
        .filter(|&i| block_of(i) != block && cands.masks[i] & bit != 0)
        .map(|i| (i, digit))
        .collect();
    (!hits.is_empty()).then_some(hits)
}

fn claiming(cands: &Candidates) -> Option<Effect> {
    for line in 0..9usize {
        for digit in 1u8..=9 {
            for horiz in [true, false] {
                if let Some(removals) = block_removals(cands, line, horiz, digit) {
                    return Some(Effect::Eliminate { removals });
                }
            }
        }
    }
    None
}

/// When the line-confined cells of a digit sit in one block, remove the
/// digit from the rest of that block outside the line.
fn block_removals(
    cands: &Candidates,
    line: usize,
    horiz: bool,
    digit: u8,
) -> Option<Vec<(usize, u8)>> {
    let bit = 1 << (digit - 1);
    let cells: Vec<usize> = (0..9)
        .map(|k| if horiz { line * 9 + k } else { k * 9 + line })
        .filter(|&i| cands.masks[i] & bit != 0)
        .collect();
    let blocks: Vec<usize> = cells.iter().map(|&i| block_of(i)).collect();
    if cells.len() < 2 || !blocks.iter().all(|&b| b == blocks[0]) {
        return None;
    }
    let hits: Vec<(usize, u8)> = (0..CELL_COUNT)
        .filter(|&i| {
            block_of(i) == blocks[0]
                && (if horiz { row_of(i) } else { col_of(i) }) != line
                && cands.masks[i] & bit != 0
        })
        .map(|i| (i, digit))
        .collect();
    (!hits.is_empty()).then_some(hits)
}
