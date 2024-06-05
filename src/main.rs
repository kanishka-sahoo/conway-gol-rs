pub mod conway;
use crate::conway::*;

fn main() {
    println!("Hello, world!");
    let mut maingrid = ConwayGameGrid::new(50, 40);
    maingrid.iterate();
}
