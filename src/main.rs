use console;
use std::cmp::{max, min};

// 2 possible states for cell: dead or alive
struct Cell {
    state: bool,
}

// store an encoding of cells for communication purposes
struct UpdatedCell {
    state: bool,
    row: usize,
    col: usize,
}

// define a grid of cells
struct ConwayGameGrid {
    grid: Vec<Cell>,

    // cell index = (row * self.grid_height) + col
    row: usize,
    col: usize,
    grid_width: usize,
    grid_height: usize,
}

// get the vector index from row and col numbers
fn compute_index(row: usize, col: usize, maxheight: usize) -> usize {
    (row * maxheight) + col
}

impl ConwayGameGrid {
    fn new(width: usize, height: usize) -> ConwayGameGrid {
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
    //
    // fn from(cells: Vec<Cell>) -> ConwayGameGrid {
    //     let gamestate = ConwayGameGrid {
    //         grid: cells,
    //         row: 0,
    //         col: 0,
    //     };
    //     gamestate
    // }

    // update grid based on updated cell vector
    fn update_cells(&mut self, cells: Vec<UpdatedCell>) {
        for cell in cells {
            self.grid[compute_index(cell.row, cell.col, self.grid_height)].state = cell.state;
        }
    }

    // get number of alive neighbours of current cell
    fn get_alive_neighbours_count(&self) -> u8 {
        let mut neighbours = 0;

        // the top row (row-1, col-1..=col+1)
        for col in max(self.col - 1, 0)..=min(self.col + 1, self.grid_width) {
            if self.grid[compute_index(min(self.row - 1, 0), col, self.grid_height)].state {
                neighbours += 1;
            }
        }

        // the bottom row (row+1, col-1..=col+1)
        for col in max(self.col - 1, 0)..=min(self.col + 1, self.grid_width) {
            if self.grid[compute_index(max(self.row + 1, self.grid_height), col, self.grid_height)]
                .state
            {
                neighbours += 1;
            }
        }

        // middle row (left and right)
        if self.grid[compute_index(self.row, max(self.col - 1, 0), self.grid_height)].state {
            neighbours += 1
        }
        if self.grid[compute_index(
            self.row,
            min(self.col + 1, self.grid_width),
            self.grid_height,
        )]
        .state
        {
            neighbours += 1
        }

        neighbours
    }

    // run for all the cells
    fn iterate(&mut self) {
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
                    if (alive > 3) && (alive < 2) {
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
    }
}

fn main() {
    println!("Hello, world!");
    let mut maingrid = ConwayGameGrid::new(50, 40);
    maingrid.iterate();
}
