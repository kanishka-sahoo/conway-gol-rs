# Conway's Game of Life

A Rust library implementation of Conway's Game of Life.

## Features

- Create a game grid of any size
- Iterate the game state
- Get and set individual cell states
- Visualize the game state as a string


## Usage

Here's a quick example of how to use the library:

```rust
use conway_gol_rs::ConwayGameGrid;
use conway_gol_rs::UpdatedCell as Cell;

fn main() {
    // Create a 10x10 grid
    let mut game = ConwayGameGrid::new(10, 10);

    // Set some initial live cells using UpdatedCell::new
    let cells: Vec<Cell> = vec![
        Cell::new(5, 5, true),
        Cell::new(5, 6, true),
        Cell::new(6, 5, true),
        Cell::new(6, 6, true),
    ];

    // Apply the batched updates
    game.update_cells(cells);

    // Run a single iteration
    game.iterate();

    // Dump the grid to stdout (debug print)
    game.dump();
}
```

### Helpers: get/set and grid size

This crate provides small, safe helpers on `ConwayGameGrid` to access and
modify individual cells without reaching into the `grid` vector directly:

- `get(&self, row, col) -> Option<bool>`: returns the cell state or `None` when out of bounds.
- `set(&mut self, row, col, state) -> bool`: sets a cell and returns `true` on success.
- `width()` / `height()` accessors return the grid dimensions.

Note on neighbour behavior: the library uses toroidal wrapping for neighbour
calculations (edges wrap around like a torus). Use `alive_neighbours_at(row, col)` to
query neighbour counts at arbitrary positions.

## API

### `ConwayGameGrid`

The main struct representing the game grid.

#### Methods

- `new(width: usize, height: usize) -> Self`: Create a new game grid.
- `iterate(&mut self)`: Advance the game state by one generation.
- `update_cells(&mut self, cells: Vec<UpdatedCell>)`: Update multiple cells at once (apply batched updates).
    Use `UpdatedCell::new(row, col, state)` to construct updates.
- `get(&self, row, col) -> Option<bool>`: Query a cell's state safely.
- `set(&mut self, row, col, state) -> bool`: Set a cell's state (returns false if out of bounds).
- `alive_neighbours_at(&self, row, col) -> u8`: Returns the number of alive neighbours at a position (toroidal wrapping).
- `get_alive_neighbours_count(&self) -> u8`: Query neighbours for the grid's internal cursor (keeps compatibility with earlier code).
- `dump(&self)`: Prints a simple textual representation of the grid to stdout (debug helper).

### `UpdatedCell`

A struct representing a cell update.

#### Fields

- `row: usize`: The row of the cell.
- `col: usize`: The column of the cell.
- `state: bool`: The new state of the cell.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
