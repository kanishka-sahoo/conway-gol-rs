use std::cmp::{max, min};

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 60;

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

    // cell index = (row * GRID_HEIGHT) + col
    row: usize,
    col: usize,
}

// get the vector index from row and col numbers
fn compute_index(row: usize, col: usize) -> usize {
    (row * GRID_HEIGHT) + col
}

impl ConwayGameGrid {
    fn new() -> ConwayGameGrid {
        let mut grid = Vec::new();
        for _ in 0..GRID_WIDTH * GRID_HEIGHT {
            let cell = Cell { state: false };
            grid.push(cell);
        }

        let gamestate = ConwayGameGrid {
            grid,
            row: 0,
            col: 0,
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
            self.grid[compute_index(cell.row, cell.col)].state = cell.state;
        }
    }

    // get number of alive neighbours of current cell
    fn get_alive_neighbours_count(&self) -> u8 {
        let mut neighbours = 0;

        // the top row (row-1, col-1..=col+1)
        for col in max(self.col - 1, 0)..=min(self.col + 1, GRID_WIDTH) {
            if self.grid[compute_index(min(self.row - 1, 0), col)].state {
                neighbours += 1;
            }
        }

        // the bottom row (row+1, col-1..=col+1)
        for col in max(self.col - 1, 0)..=min(self.col + 1, GRID_WIDTH) {
            if self.grid[compute_index(max(self.row + 1, GRID_HEIGHT), col)].state {
                neighbours += 1;
            }
        }

        // middle row (left and right)
        if self.grid[compute_index(self.row, max(self.col - 1, 0))].state {
            neighbours += 1
        }
        if self.grid[compute_index(self.row, min(self.col + 1, GRID_WIDTH))].state {
            neighbours += 1
        }

        neighbours
    }

    // run for all the cells
    fn iterate(&mut self) {
        let mut updatedcells = Vec::new();

        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                self.row = row;
                self.col = col;
                let alive = self.get_alive_neighbours_count();
                let mut newcell = UpdatedCell {
                    state: false,
                    row,
                    col,
                };
                if self.grid[compute_index(row, col)].state {
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
    let mut maingrid = ConwayGameGrid::new();
    maingrid.iterate();
}
