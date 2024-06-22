use crate::cell::{Cell, UpdatedCell};
use std::cmp::{max, min};

pub struct ConwayGameGrid {
    grid: Vec<Cell>,
    grid_width: usize,
    grid_height: usize,
}

impl ConwayGameGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![Cell { state: false }; width * height];
        Self {
            grid,
            grid_width: width,
            grid_height: height,
        }
    }

    pub fn update_cells(&mut self, cells: &[UpdatedCell]) {
        for &cell in cells {
            let index = self.compute_index(cell.row, cell.col);
            self.grid[index].state = cell.state;
        }
    }

    fn compute_index(&self, row: usize, col: usize) -> usize {
        row * self.grid_width + col
    }

    fn get_alive_neighbours_count(&self, row: usize, col: usize) -> u8 {
        let mut neighbours = 0;
        for i in max(1, row + 1) - 1..=min(row + 1, self.grid_height - 1) {
            for j in max(1, col + 1) - 1..=min(col + 1, self.grid_width - 1) {
                if (i != row || j != col) && self.grid[self.compute_index(i, j)].state {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    pub fn iterate(&mut self) {
        let mut updated_cells = Vec::new();

        for row in 0..self.grid_height {
            for col in 0..self.grid_width {
                let alive = self.get_alive_neighbours_count(row, col);
                let current_state = self.grid[self.compute_index(row, col)].state;
                let new_state = match (current_state, alive) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                if new_state != current_state {
                    updated_cells.push(UpdatedCell::new(row, col, new_state));
                }
            }
        }

        self.update_cells(&updated_cells);
    }

    pub fn dump(&self) -> String {
        let mut output = String::new();
        for row in 0..self.grid_height {
            for col in 0..self.grid_width {
                output.push_str(if self.grid[self.compute_index(row, col)].state {
                    "[]"
                } else {
                    "oo"
                });
            }
            output.push('\n');
        }
        output.push('\n');
        output
    }

    pub fn get_cell_state(&self, row: usize, col: usize) -> Option<bool> {
        if row < self.grid_height && col < self.grid_width {
            Some(self.grid[self.compute_index(row, col)].state)
        } else {
            None
        }
    }

    pub fn set_cell_state(&mut self, row: usize, col: usize, state: bool) -> bool {
        if row < self.grid_height && col < self.grid_width {
            let index = self.compute_index(row, col);
            self.grid[index].state = state;
            true
        } else {
            false
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.grid_width, self.grid_height)
    }
}
