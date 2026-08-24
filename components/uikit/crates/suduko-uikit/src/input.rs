//! Where the digit input surface renders.

use std::fmt;

/// Placement of the 1-9/Erase input surface. `Below` (the default)
/// and `Above` render one fixed pad row; `Popup` drops the fixed pad
/// and opens a mini keypad beside the tapped cell instead.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputMode {
    /// Fixed pad row above the board.
    Above,
    /// Fixed pad row below the board.
    #[default]
    Below,
    /// Per-cell popup keypad; no fixed pad.
    Popup,
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Above => "above",
            Self::Below => "below",
            Self::Popup => "popup",
        };
        f.write_str(text)
    }
}
