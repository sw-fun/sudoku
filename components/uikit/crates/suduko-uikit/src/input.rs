//! Where the digit input surface lives, and how pencil notes
//! behave.

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

/// Pencil-note behavior. The header Notes button cycles the three.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NotesMode {
    /// No marks; digit entry places values.
    #[default]
    Off,
    /// Player-entered marks only (start empty); digits toggle marks.
    User,
    /// App-filled computed candidates; digits strike them out.
    Auto,
}

impl NotesMode {
    /// Header button label for the mode.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Notes",
            Self::User => "Notes: mine",
            Self::Auto => "Notes: auto",
        }
    }

    /// The next mode when the header button cycles: off -> user ->
    /// auto -> off.
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Off => Self::User,
            Self::User => Self::Auto,
            Self::Auto => Self::Off,
        }
    }

    /// Save-slot encoding of the mode, or `None` for a bad index.
    #[must_use]
    pub fn from_index(idx: u8) -> Option<Self> {
        match idx {
            0 => Some(Self::Off),
            1 => Some(Self::User),
            2 => Some(Self::Auto),
            _ => None,
        }
    }
}
