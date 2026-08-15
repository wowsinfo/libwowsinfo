//! Ship comparison table.
//!
//! Ports the Flutter two `compare_ship_page` and the wows-toolkit
//! `ComparisonShip` concept: pick up to a handful of ships and line up their
//! headline stats in a table. All values are computed from the same
//! `wowsinfo.json` build used by the ship detail screen.
//! Split into domain submodules so every file stays small.

mod build;
mod rows;
mod similar;
#[cfg(test)]
mod tests;
mod types;

pub use build::build_local_compare;
pub use similar::similar_ships;
pub use types::{CompareRow, CompareShipHeader, LocalCompare, SimilarShip};

