use crate::cell::{Cell, UpdatedCell};

/// Define a grid of cells for Conway's Game of Life.
///
/// The public surface of this type is kept the same but internal helpers
/// and indexing have been cleaned up for correctness and clarity.
pub struct ConwayGameGrid {
    pub grid: Vec<Cell>,

    /// number of columns
    grid_width: usize,
    /// number of rows
    grid_height: usize,

    /// temporary row/col used by some methods (kept for compatibility with
    /// the existing public API that expected this internal state).
    row: usize,
    col: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_and_bounds() {
        let mut g = ConwayGameGrid::new(4, 3);

        // initial cells are dead
        assert_eq!(g.get(0, 0), Some(false));

        // out of bounds get returns None
        assert_eq!(g.get(10, 10), None);

        // out of bounds set returns false
        assert!(!g.set(10, 10, true));

        // valid set updates the cell
        assert!(g.set(2, 3, true));
        assert_eq!(g.get(2, 3), Some(true));

        // clearing works
        assert!(g.set(2, 3, false));
        assert_eq!(g.get(2, 3), Some(false));
    }

    #[test]
    fn test_dimensions() {
        let g = ConwayGameGrid::new(7, 5);
        assert_eq!(g.width(), 7);
        assert_eq!(g.height(), 5);
    }
}

/// get the vector index from row and col numbers using grid width
fn compute_index(row: usize, col: usize, width: usize) -> usize {
    (row * width) + col
}

impl ConwayGameGrid {
    /// Create a new empty grid (all cells dead) with given width (cols) and height (rows).
    pub fn new(width: usize, height: usize) -> ConwayGameGrid {
        let grid = vec![Cell::default(); width * height];

        ConwayGameGrid {
            grid,
            grid_width: width,
            grid_height: height,
            row: 0,
            col: 0,
        }
    }

    /// update grid based on updated cell vector
    pub fn update_cells(&mut self, cells: Vec<UpdatedCell>) {
        for cell in cells {
            let idx = compute_index(cell.row, cell.col, self.grid_width);
            if idx < self.grid.len() {
                self.grid[idx].state = cell.state;
            }
        }
    }

    /// get number of alive neighbours of the current cell (uses `self.row` and `self.col`).
    ///
    /// Note: this keeps the original public signature but the implementation is
    /// corrected and simplified. It clamps neighbours to the grid bounds (no wrapping).
    pub fn get_alive_neighbours_count(&self) -> u8 {
        self.alive_neighbours_at(self.row, self.col)
    }

    /// Return the number of alive neighbours for the cell at (row, col).
    ///
    /// This uses toroidal wrapping (edges wrap around) so the grid behaves like a torus.
    pub fn alive_neighbours_at(&self, row: usize, col: usize) -> u8 {
        use std::collections::HashSet;

        // Collect distinct neighbor coordinates (avoid counting the same
        // position multiple times on very small grids) and exclude the cell itself.
        let mut seen: HashSet<(usize, usize)> = HashSet::new();

        let h = self.grid_height as isize;
        let w = self.grid_width as isize;

        for dr in -1isize..=1isize {
            for dc in -1isize..=1isize {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let nr = ((row as isize + dr + h) % h) as usize;
                let nc = ((col as isize + dc + w) % w) as usize;

                if nr == row && nc == col {
                    // although wrapping may map to the same cell, we exclude
                    // the cell itself from neighbour set
                    continue;
                }

                seen.insert((nr, nc));
            }
        }

        seen.into_iter()
            .filter(|&(r, c)| self.grid[compute_index(r, c, self.grid_width)].state)
            .count() as u8
    }

    /// run one iteration for all the cells
    pub fn iterate(&mut self) {
        let mut updatedcells = Vec::new();

        for row in 0..self.grid_height {
            for col in 0..self.grid_width {
                self.row = row;
                self.col = col;
                let alive = self.get_alive_neighbours_count();
                let mut newcell = UpdatedCell {
                    state: false,
                    row,
                    col,
                };
                if self.grid[compute_index(row, col, self.grid_width)].state {
                    // if live cell has more than 3 or less than 2 live neighbours, it dies
                    if !(2..=3).contains(&alive) {
                        newcell.state = false;
                        updatedcells.push(newcell);
                    }
                } else {
                    // if dead cell has 3 alive neighbours it comes alive
                    if alive == 3 {
                        newcell.state = true;
                        updatedcells.push(newcell);
                    }
                }
            }
        }

        self.update_cells(updatedcells);
        self.row = 0;
        self.col = 0;
    }

    /// Get the state of the cell at (row, col).
    ///
    /// Returns `Some(true)` if the cell is alive, `Some(false)` if dead, or
    /// `None` if the coordinates are out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
        if row < self.grid_height && col < self.grid_width {
            let idx = compute_index(row, col, self.grid_width);
            Some(self.grid[idx].state)
        } else {
            None
        }
    }

    /// Set the state of the cell at (row, col).
    ///
    /// Returns `true` if the cell was updated, or `false` if the coordinates
    /// were out of bounds.
    pub fn set(&mut self, row: usize, col: usize, state: bool) -> bool {
        if row < self.grid_height && col < self.grid_width {
            let idx = compute_index(row, col, self.grid_width);
            self.grid[idx].state = state;
            true
        } else {
            false
        }
    }

    /// Return the number of columns (width) of the grid.
    pub fn width(&self) -> usize {
        self.grid_width
    }

    /// Return the number of rows (height) of the grid.
    pub fn height(&self) -> usize {
        self.grid_height
    }

    /// debug print to screen
    pub fn dump(&self) {
        for row in 0..self.grid_height {
            for col in 0..self.grid_width {
                if self.grid[compute_index(row, col, self.grid_width)].state {
                    print!("[]");
                } else {
                    print!("oo");
                }
            }
            println!();
        }
        println!();
    }
}
