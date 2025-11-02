use conway_gol_rs::{ConwayGameGrid, UpdatedCell};

// Helper to compute index in tests (row-major by width)
fn idx(row: usize, col: usize, width: usize) -> usize {
    row * width + col
}

#[test]
fn test_update_cells_indexes() {
    let mut g = ConwayGameGrid::new(3, 3);

    // set (1,2) alive using update_cells
    g.update_cells(vec![UpdatedCell::new(1, 2, true)]);

    assert!(
        g.grid[idx(1, 2, 3)].state,
        "cell (1,2) should be alive after update_cells"
    );
}

#[test]
fn test_alive_neighbours_wraps_toroidally() {
    let mut g = ConwayGameGrid::new(3, 3);

    // make a single alive cell at (0,0)
    g.grid[idx(0, 0, 3)].state = true;

    // under toroidal wrapping, the cell at (2,2) should see (0,0) as a neighbour
    let n = g.alive_neighbours_at(2, 2);
    assert_eq!(n, 1, "expected 1 neighbour via wrapping, got {}", n);
}

#[test]
fn test_blinker_oscillator() {
    let mut g = ConwayGameGrid::new(5, 5);
    let w = 5;

    // horizontal blinker in row 2, cols 1..=3
    g.grid[idx(2, 1, w)].state = true;
    g.grid[idx(2, 2, w)].state = true;
    g.grid[idx(2, 3, w)].state = true;

    g.iterate();

    // after one iteration it should be vertical in col=2 rows 1..=3
    assert!(g.grid[idx(1, 2, w)].state, "(1,2) should be alive");
    assert!(g.grid[idx(2, 2, w)].state, "(2,2) should be alive");
    assert!(g.grid[idx(3, 2, w)].state, "(3,2) should be alive");

    // the original horizontal endpoints should be dead
    assert!(!g.grid[idx(2, 1, w)].state, "(2,1) should be dead");
    assert!(!g.grid[idx(2, 3, w)].state, "(2,3) should be dead");
}
