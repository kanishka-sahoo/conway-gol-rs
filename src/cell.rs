#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub state: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct UpdatedCell {
    pub state: bool,
    pub row: usize,
    pub col: usize,
}

impl UpdatedCell {
    pub fn new(row: usize, col: usize, state: bool) -> Self {
        Self { state, row, col }
    }
}
