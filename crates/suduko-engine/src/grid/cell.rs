#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Value(u8),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    pub fn value(&self) -> Option<u8> {
        match self {
            Cell::Empty => None,
            Cell::Value(v) => Some(*v),
        }
    }
}
