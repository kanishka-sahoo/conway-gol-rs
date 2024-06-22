use conway_game_of_life::ConwayGameGrid;

#[test]
fn test_new_grid() {
    let grid = ConwayGameGrid::new(5, 5);
    assert_eq!(grid.dimensions(), (5, 5));
}

#[test]
fn test_set_and_get_cell_state() {
    let mut grid = ConwayGameGrid::new(3, 3);

    assert!(grid.set_cell_state(1, 1, true));
    assert_eq!(grid.get_cell_state(1, 1), Some(true));

    assert_eq!(grid.get_cell_state(0, 0), Some(false));

    assert!(!grid.set_cell_state(3, 3, true));
    assert_eq!(grid.get_cell_state(3, 3), None);
}

#[test]
fn test_simple_iteration() {
    let mut grid = ConwayGameGrid::new(3, 3);

    // Set up a simple blinker pattern
    grid.set_cell_state(0, 1, true);
    grid.set_cell_state(1, 1, true);
    grid.set_cell_state(2, 1, true);

    grid.iterate();

    assert_eq!(grid.get_cell_state(1, 0), Some(true));
    assert_eq!(grid.get_cell_state(1, 1), Some(true));
    assert_eq!(grid.get_cell_state(1, 2), Some(true));
    assert_eq!(grid.get_cell_state(0, 1), Some(false));
    assert_eq!(grid.get_cell_state(2, 1), Some(false));
}

#[test]
fn test_stable_pattern() {
    let mut grid = ConwayGameGrid::new(4, 4);

    // Set up a stable square pattern
    grid.set_cell_state(1, 1, true);
    grid.set_cell_state(1, 2, true);
    grid.set_cell_state(2, 1, true);
    grid.set_cell_state(2, 2, true);

    let initial_state = grid.dump();
    grid.iterate();
    let final_state = grid.dump();

    assert_eq!(initial_state, final_state);
}

#[test]
fn test_out_of_bounds() {
    let grid = ConwayGameGrid::new(3, 3);

    assert_eq!(grid.get_cell_state(3, 3), None);
    assert_eq!(grid.get_cell_state(0, 3), None);
    assert_eq!(grid.get_cell_state(3, 0), None);
}

#[test]
fn test_update_cells() {
    use conway_game_of_life::UpdatedCell;

    let mut grid = ConwayGameGrid::new(3, 3);

    let updates = vec![
        UpdatedCell::new(0, 0, true),
        UpdatedCell::new(1, 1, true),
        UpdatedCell::new(2, 2, true),
    ];

    grid.update_cells(&updates);

    assert_eq!(grid.get_cell_state(0, 0), Some(true));
    assert_eq!(grid.get_cell_state(1, 1), Some(true));
    assert_eq!(grid.get_cell_state(2, 2), Some(true));
    assert_eq!(grid.get_cell_state(0, 1), Some(false));
}

#[test]
fn test_dump() {
    let mut grid = ConwayGameGrid::new(2, 2);
    grid.set_cell_state(0, 0, true);
    grid.set_cell_state(1, 1, true);

    let expected = "[]oo\noo[]\n\n";
    assert_eq!(grid.dump(), expected);
}
