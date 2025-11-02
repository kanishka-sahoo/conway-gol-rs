use conway_gol_rs::ConwayGameGrid;

#[test]
fn one_by_one_grid_neighbors() {
    let mut g = ConwayGameGrid::new(1, 1);
    // single cell alive
    g.set(0, 0, true);
    // distinct neighbor count should be 0 (no other distinct positions)
    assert_eq!(g.alive_neighbours_at(0, 0), 0);
}

#[test]
fn one_by_three_unique_neighbors() {
    let mut g = ConwayGameGrid::new(3, 1);
    // set both neighbours of middle cell
    g.set(0, 0, true);
    g.set(0, 2, true);

    // middle cell should see 2 neighbours
    assert_eq!(g.alive_neighbours_at(0, 1), 2);
}

#[test]
fn two_by_two_all_alive_neighbors() {
    let mut g = ConwayGameGrid::new(2, 2);
    // set all 4 cells alive
    for r in 0..2 {
        for c in 0..2 {
            g.set(r, c, true);
        }
    }

    // each cell should see the other 3 as neighbours (distinct positions)
    for r in 0..2 {
        for c in 0..2 {
            assert_eq!(
                g.alive_neighbours_at(r, c),
                3,
                "cell ({},{}) should have 3 neighbours",
                r,
                c
            );
        }
    }
}

#[test]
fn narrow_grid_column_neighbors() {
    // 1 column, 4 rows
    let mut g = ConwayGameGrid::new(1, 4);
    // set neighbors above and below row 1 (rows 0 and 2)
    g.set(0, 0, true);
    g.set(2, 0, true);
    assert_eq!(g.alive_neighbours_at(1, 0), 2);
}
