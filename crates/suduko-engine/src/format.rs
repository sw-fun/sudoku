use crate::grid::{Board, CELL_COUNT, Cell};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    WrongLength,
    InvalidCharacter { pos: usize },
}

pub fn to_string(board: &Board) -> String {
    let mut out = String::with_capacity(CELL_COUNT);
    for idx in 0..CELL_COUNT {
        match board.get(idx) {
            Cell::Empty => out.push('.'),
            Cell::Value(v) => out.push((b'0' + v) as char),
        }
    }
    out
}

pub fn parse(text: &str) -> Result<Board, ParseError> {
    if text.len() != CELL_COUNT {
        return Err(ParseError::WrongLength);
    }
    let mut board = Board::new();
    for (pos, ch) in text.chars().enumerate() {
        let cell = match ch {
            '.' | '0' => Cell::Empty,
            '1'..='9' => Cell::Value(ch as u8 - b'0'),
            _ => return Err(ParseError::InvalidCharacter { pos }),
        };
        board.set(pos, cell);
    }
    Ok(board)
}
