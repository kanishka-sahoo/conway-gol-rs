pub mod conway;
use crate::conway::*;

fn main() {
    let mut maingrid = ConwayGameGrid::new(10, 10);
    let mut things: Vec<UpdatedCell> = Vec::new();
    things.push(UpdatedCell::new(2, 3, true));
    things.push(UpdatedCell::new(2, 4, true));
    things.push(UpdatedCell::new(2, 5, true));
    maingrid.update_cells(things);
    maingrid.dump();
    for _ in 0..10 {
        maingrid.iterate();
        maingrid.dump();
    }
}
