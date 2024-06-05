pub mod conway;
use crate::conway::*;

fn main() {
    let mut maingrid = ConwayGameGrid::new(10, 10);
    let mut things: Vec<UpdatedCell> = Vec::new();
    things.push(UpdatedCell {
        state: true,
        row: 1,
        col: 3,
    });
    things.push(UpdatedCell {
        state: true,
        row: 2,
        col: 1,
    });
    things.push(UpdatedCell {
        state: true,
        row: 2,
        col: 3,
    });
    things.push(UpdatedCell {
        state: true,
        row: 3,
        col: 2,
    });
    things.push(UpdatedCell {
        state: true,
        row: 3,
        col: 3,
    });
    maingrid.update_cells(things);

    maingrid.iterate();
    maingrid.iterate();
    maingrid.iterate();
    maingrid.iterate();

    maingrid.iterate();
    maingrid.iterate();
    maingrid.iterate();
    maingrid.iterate();

    maingrid.iterate();
    maingrid.iterate();
    maingrid.iterate();
    maingrid.iterate();

    maingrid.iterate();
    maingrid.iterate();
    maingrid.iterate();
    maingrid.iterate();

    maingrid.dump();
}
