//! Mini keypad placement and button rules, computed from plain cell
//! facts so the crate stays decoupled from game state.

/// Horizontal placement of the keypad relative to the target cell.
#[derive(Debug, PartialEq, Eq)]
pub enum HAlign {
    /// Left edge at the cell; the keypad extends right.
    Left,
    /// Centered on the cell.
    Center,
    /// Right edge at the cell; the keypad extends left.
    Right,
}

/// Which side of the target cell the keypad renders on.
#[derive(Debug, PartialEq, Eq)]
pub struct Anchor {
    /// True renders below the cell, false above it.
    pub below: bool,
    /// Horizontal alignment keeping the keypad on the board.
    pub h: HAlign,
}

/// Anchor for the keypad opened on cell `idx` (row-major 0..80):
/// always toward the board center so the 3x4 keypad stays inside the
/// board and never covers the target cell. The middle row defaults
/// to below.
#[must_use]
pub fn anchor(idx: usize) -> Anchor {
    let (row, col) = (idx / 9, idx % 9);
    Anchor {
        below: row <= 4,
        h: if col <= 2 {
            HAlign::Left
        } else if col >= 6 {
            HAlign::Right
        } else {
            HAlign::Center
        },
    }
}

/// Inline CSS placing the keypad for cell `idx` inside a
/// `position: relative` board sized in `var(--cell)` units.
#[must_use]
pub fn anchor_style(idx: usize) -> String {
    use std::fmt::Write as _;
    let (row, col) = (idx / 9, idx % 9);
    let a = anchor(idx);
    let v = |n: usize| format!("calc({n} * var(--cell) + 3px)");
    let mut style = String::new();
    if a.below {
        let _ = write!(style, "top: {}; ", v(row + 1));
    } else {
        let _ = write!(style, "bottom: {}; ", v(9 - row));
    }
    match a.h {
        HAlign::Left => {
            let _ = write!(style, "left: {}; ", v(col));
        }
        HAlign::Right => {
            let _ = write!(style, "right: {}; ", v(8 - col));
        }
        HAlign::Center => {
            let _ = write!(
                style,
                "left: calc({col}.5 * var(--cell) + 3px); transform: translateX(-50%); "
            );
        }
    }
    style
}

/// Plain-data snapshot of the selected cell that drives keypad button
/// enablement. `complete_mask` bit `d - 1` marks digit `d` as fully
/// and correctly placed on the board.
#[derive(Clone, Copy, Debug, Default)]
pub struct CellInput {
    /// Bits 0..8 set for digits 1..9 already complete on the board.
    pub complete_mask: u16,
    /// The wrong (red) digit sitting in the selected cell, if any.
    pub wrong_digit: Option<u8>,
    /// The digit currently shown in the selected cell, if any.
    pub value: Option<u8>,
    /// True when the selected cell is an unerasable clue.
    pub given: bool,
}

impl CellInput {
    /// A keypad digit is tappable unless its nine placements are
    /// complete, or it is the red value already in the cell (any
    /// other digit replaces it directly).
    #[must_use]
    pub fn digit_enabled(&self, digit: u8) -> bool {
        self.complete_mask & (1 << (digit - 1)) == 0 && self.wrong_digit != Some(digit)
    }

    /// The erase button is tappable only for player-entered values.
    #[must_use]
    pub fn erase_enabled(&self) -> bool {
        self.value.is_some() && !self.given
    }
}
