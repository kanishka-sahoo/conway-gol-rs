/// A single cell of the Conway's Game of Life grid.
///
/// `state` is true when the cell is alive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cell {
    pub state: bool,
}

/// Represents an updated cell position and the state it should have.
///
/// This type is used by `ConwayGameGrid::update_cells` to apply batched
/// updates to the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpdatedCell {
    pub state: bool,
    pub row: usize,
    pub col: usize,
}

impl UpdatedCell {
    /// Create a new UpdatedCell for (row, col) with given state.
    pub fn new(row: usize, col: usize, state: bool) -> Self {
        Self { state, row, col }
    }
}

impl Cell {
    /// Create a new Cell with the given state.
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}
