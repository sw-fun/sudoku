//! Shared annotation types: what a strategy found and how to teach it.

use suduko_techniques::Effect as RawEffect;

/// A taught strategy, ordered by solving-ladder rung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Strategy {
    NakedSingle,
    HiddenSingle,
    Pointing,
    Claiming,
    NakedPair,
    HiddenPair,
    XWing,
    XYWing,
    Trial,
}

impl Strategy {
    /// Ladder rung: `find_all` orders ascending by this.
    #[must_use]
    pub fn rung(self) -> u32 {
        match self {
            Strategy::NakedSingle => 1,
            Strategy::HiddenSingle => 2,
            Strategy::Pointing => 3,
            Strategy::Claiming => 4,
            Strategy::NakedPair => 5,
            Strategy::HiddenPair => 6,
            Strategy::XWing => 7,
            Strategy::XYWing => 8,
            Strategy::Trial => 9,
        }
    }

    /// Human name shown in the strategy picker.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Strategy::NakedSingle => "Naked Single",
            Strategy::HiddenSingle => "Hidden Single",
            Strategy::Pointing => "Pointing",
            Strategy::Claiming => "Claiming",
            Strategy::NakedPair => "Naked Pair",
            Strategy::HiddenPair => "Hidden Pair",
            Strategy::XWing => "X-Wing",
            Strategy::XYWing => "XY-Wing",
            Strategy::Trial => "Trial",
        }
    }
}

/// A row, column, or block; indices are 0-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitRef {
    Row(usize),
    Col(usize),
    Block(usize),
}

impl UnitRef {
    /// The nine cell indices of the unit, ascending.
    #[must_use]
    pub fn cells(self) -> [usize; 9] {
        match self {
            UnitRef::Row(r) => core::array::from_fn(|k| r * 9 + k),
            UnitRef::Col(c) => core::array::from_fn(|k| k * 9 + c),
            UnitRef::Block(b) => core::array::from_fn(|k| {
                let top = (b / 3) * 27 + (b % 3) * 3;
                top + (k / 3) * 9 + k % 3
            }),
        }
    }

    /// Short label for teaching text ("row 3", "block 7").
    #[must_use]
    pub fn label(self) -> String {
        match self {
            UnitRef::Row(r) => format!("row {}", r + 1),
            UnitRef::Col(c) => format!("column {}", c + 1),
            UnitRef::Block(b) => format!("block {}", b + 1),
        }
    }
}

/// What the learner may conclude: one placement or a batch of removals.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Effect {
    Place { idx: usize, digit: u8 },
    Eliminate { removals: Vec<(usize, u8)> },
}

impl From<&RawEffect> for Effect {
    fn from(raw: &RawEffect) -> Self {
        match raw {
            RawEffect::Place { idx, digit } => Effect::Place {
                idx: *idx,
                digit: *digit,
            },
            RawEffect::Eliminate { removals } => Effect::Eliminate {
                removals: removals.clone(),
            },
        }
    }
}

/// One teaching beat: what to highlight and what to say.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    /// Cells to spotlight in this beat.
    pub cells: Vec<usize>,
    /// Units to tint in this beat.
    pub units: Vec<UnitRef>,
    /// Digits to emphasize in pencil marks.
    pub digits: Vec<u8>,
    /// The explanation shown under the board.
    pub text: String,
}

/// A strategy occurrence on the current board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub strategy: Strategy,
    /// Picker label, e.g. "X-Wing on 2s (columns 4 and 6)".
    pub title: String,
    /// The digits the strategy is about, ascending.
    pub digits: Vec<u8>,
    /// The pattern cells (pair corners, wing cells, ...), ascending.
    pub pattern: Vec<usize>,
    /// The units the pattern lives in.
    pub units: Vec<UnitRef>,
    /// What can be safely inferred.
    pub effect: Effect,
    /// Ordered teaching beats.
    pub steps: Vec<Step>,
}

/// "r3c5" style cell name for teaching text.
#[must_use]
pub fn cell_name(idx: usize) -> String {
    format!("r{}c{}", idx / 9 + 1, idx % 9 + 1)
}

/// Comma-separated cell names.
#[must_use]
pub fn name_list(cells: &[usize]) -> String {
    cells
        .iter()
        .map(|&i| cell_name(i))
        .collect::<Vec<_>>()
        .join(", ")
}
