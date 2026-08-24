//! The five published difficulty levels.

/// Ordered by the hardest technique their band accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Easy,
    Medium,
    Hard,
    Harder,
    Hardest,
}

impl Level {
    /// Display label shown in the game header and menus.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
            Self::Harder => "Harder",
            Self::Hardest => "Hardest",
        }
    }
}
