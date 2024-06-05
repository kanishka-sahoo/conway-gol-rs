use std::cmp::{max, min};

// 2 possible states for cell: dead or alive
#[derive(Debug)]
pub struct Cell {
    pub state: bool,
}

// store an encoding of cells for communication purposes
pub struct UpdatedCell {
    pub state: bool,
    pub row: usize,
    pub col: usize,
}

// define a grid of cells
pub struct ConwayGameGrid {
    pub grid: Vec<Cell>,

    // cell index = (row * self.grid_height) + col
    pub row: usize,
    pub col: usize,
    pub grid_width: usize,
    pub grid_height: usize,
}

// get the vector index from row and col numbers
fn compute_index(row: usize, col: usize, maxheight: usize) -> usize {
    (row * maxheight) + col
}

impl ConwayGameGrid {
    pub fn new(width: usize, height: usize) -> ConwayGameGrid {
        let mut grid = Vec::new();
        for _ in 0..width * height {
            let cell = Cell { state: false };
            grid.push(cell);
        }

        let gamestate = ConwayGameGrid {
            grid,
            row: 0,
            col: 0,
            grid_width: width,
            grid_height: height,
        };
        gamestate
    }

    // update grid based on updated cell vector
    pub fn update_cells(&mut self, cells: Vec<UpdatedCell>) {
        for cell in cells {
            self.grid[compute_index(cell.row, cell.col, self.grid_height)].state = cell.state;
        }
    }

    // get number of alive neighbours of current cell
    fn get_alive_neighbours_count(&self) -> u8 {
        let mut neighbours = 0;

        let lcol: usize;
        let lrow: usize;
        let grow: usize;
        let gcol: usize;

        if self.col == 0 {
            lcol = 0
        } else {
            lcol = self.col - 1;
        }

        if self.row == 0 {
            lrow = 0
        } else {
            lrow = self.row - 1;
        }

        if self.col >= self.grid_width - 1 {
            gcol = 0
        } else {
            gcol = self.col + 1;
        }

        if self.row >= self.grid_height - 1 {
            grow = 0
        } else {
            grow = self.row + 1;
        }

        // the top row (row-1, col-1..=col+1)
        for col in max(lcol, 0)..=min(gcol, self.grid_width - 1) {
            if self.grid[compute_index(lrow, col, self.grid_height)].state {
                neighbours += 1;
            }
        }

        // the bottom row (row+1, col-1..=col+1)
        for col in max(lcol, 0)..=min(gcol, self.grid_width - 1) {
            if self.grid[compute_index(grow, col, self.grid_height)].state {
                neighbours += 1;
            }
        }

        // middle row (left and right)
        if self.grid[compute_index(self.row, lcol, self.grid_height)].state {
            neighbours += 1
        }
        if self.grid[compute_index(self.row, gcol, self.grid_height)].state {
            neighbours += 1
        }

        neighbours
    }

    // run for all the cells
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
                if self.grid[compute_index(row, col, self.grid_height)].state {
                    // if live cell has more than 3 or less than 2 live neighbours, it dies
                    if (alive > 3) || (alive < 2) {
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

    // debug print to screen
    pub fn dump(&self) {
        for row in 0..self.grid_height {
            for col in 0..self.grid_width {
                if self.grid[compute_index(row, col, self.grid_height)].state {
                    print!("[]");
                } else {
                    print!("oo");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}
