# Conway's Game of Life

A Rust library implementation of Conway's Game of Life.

## Features

- Create a game grid of any size
- Iterate the game state
- Get and set individual cell states
- Visualize the game state as a string

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
conway_game_of_life = "0.1.0"
```

## Usage

Here's a quick example of how to use the library:

```rust
use conway_game_of_life::ConwayGameGrid;

fn main() {
    // Create a 10x10 grid
    let mut game = ConwayGameGrid::new(10, 10);
    
    // Set some initial live cells
    game.set_cell_state(1, 1, true);
    game.set_cell_state(1, 2, true);
    game.set_cell_state(1, 3, true);

    // Run a single iteration
    game.iterate();

    // Print the current state
    println!("{}", game.dump());
}
```

## API

### `ConwayGameGrid`

The main struct representing the game grid.

#### Methods

- `new(width: usize, height: usize) -> Self`: Create a new game grid.
- `iterate(&mut self)`: Advance the game state by one generation.
- `update_cells(&mut self, cells: &[UpdatedCell])`: Update multiple cells at once.
- `get_cell_state(&self, row: usize, col: usize) -> Option<bool>`: Get the state of a specific cell.
- `set_cell_state(&mut self, row: usize, col: usize, state: bool) -> bool`: Set the state of a specific cell.
- `dimensions(&self) -> (usize, usize)`: Get the dimensions of the grid.
- `dump(&self) -> String`: Get a string representation of the current grid state.

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
