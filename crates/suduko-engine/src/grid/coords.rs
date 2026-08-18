const ROWS: usize = 9;
const COLS: usize = 9;

pub fn row_of(idx: usize) -> usize {
    idx / COLS
}

pub fn col_of(idx: usize) -> usize {
    idx % COLS
}

pub fn block_of(idx: usize) -> usize {
    (row_of(idx) / 3) * 3 + col_of(idx) / 3
}

pub fn peers_of(idx: usize) -> [usize; 20] {
    let (r, c) = (row_of(idx), col_of(idx));
    let mut peers = [0usize; 20];
    let mut n = 0;
    for i in 0..COLS {
        let row_cell = r * COLS + i;
        let col_cell = i * COLS + c;
        if row_cell != idx {
            peers[n] = row_cell;
            n += 1;
        }
        if col_cell != idx {
            peers[n] = col_cell;
            n += 1;
        }
    }
    let (block_row, block_col) = ((r / 3) * 3, (c / 3) * 3);
    for dr in 0..3 {
        for dc in 0..3 {
            let block_cell = (block_row + dr) * COLS + block_col + dc;
            if block_cell != idx && !peers[..n].contains(&block_cell) {
                peers[n] = block_cell;
                n += 1;
            }
        }
    }
    peers
}
