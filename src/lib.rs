//! Conway's Game of Life — small Rust library
//!
//! This crate provides a simple, in-memory implementation of Conway's Game of
//! Life. The grid uses toroidal wrapping for neighbour calculations (edges
//! wrap around). The main types exported are:
//!
//! - `ConwayGameGrid` — the main grid type. Create with `ConwayGameGrid::new(width, height)`.
//! - `Cell` and `UpdatedCell` — primitives used to represent cell state and batched updates.
//!
//! Example:
//!
//! ```rust
//! use conway_gol_rs::{ConwayGameGrid, UpdatedCell};
//!
//! let mut g = ConwayGameGrid::new(10, 10);
//! g.set(5, 5, true);
//! g.iterate();
//! g.dump();
//! ```
mod cell;
mod grid;

pub use cell::{Cell, UpdatedCell};
pub use grid::ConwayGameGrid;
