//! Compact 81-character board serialization.

use crate::board::{Board, CELL_COUNT, Cell};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    WrongLength,
    InvalidCharacter { pos: usize },
}

#[must_use]
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

/// Parses an 81-character board string ('.' or '0' empty, '1'-'9' value).
///
/// # Errors
///
/// Returns `ParseError::WrongLength` for any length other than 81 and
/// `ParseError::InvalidCharacter` for characters outside the accepted set.
pub fn parse(text: &str) -> Result<Board, ParseError> {
    if text.len() != CELL_COUNT {
        return Err(ParseError::WrongLength);
    }
    let mut board = Board::new();
    for (pos, ch) in text.chars().enumerate() {
        board.set(pos, parse_char(ch, pos)?);
    }
    Ok(board)
}

fn parse_char(ch: char, pos: usize) -> Result<Cell, ParseError> {
    match ch {
        '.' | '0' => Ok(Cell::Empty),
        '1'..='9' => Ok(Cell::Value(ch as u8 - b'0')),
        _ => Err(ParseError::InvalidCharacter { pos }),
    }
}
