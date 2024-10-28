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
conway_game_of_life = "0.1.3"
```

## Usage

Here's a quick example of how to use the library:

```rust
use conway_gol_rs::ConwayGameGrid;
use conway_gol_rs::UpdatedCell as Cell;

fn main() {
    // Create a 10x10 grid
    let mut game = ConwayGameGrid::new(10, 10);

    // Set some initial live cells
    let cells: Vec<Cell> = vec![Cell::new(5, 5, true), Cell::new(5, 6, true), Cell::new(6, 5, true), Cell::new(6, 6, true)];

    game.update_cells(cells);

    // Run a single iteration
    game.iterate();

    // Print the current state
    println!("{:?}", game.dump());
}
```

## API

### `ConwayGameGrid`

The main struct representing the game grid.

#### Methods

- `new(width: usize, height: usize) -> Self`: Create a new game grid.
- `iterate(&mut self)`: Advance the game state by one generation.
- `update_cells(&mut self, cells: &[UpdatedCell])`: Update multiple cells at once.
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
