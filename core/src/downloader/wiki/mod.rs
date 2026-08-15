//! Wiki parsing (`/wows/encyclopedia/*`).
//! Split into domain submodules so every file stays small.

mod helper;
mod lists;
mod ship;
mod tests;

pub use lists::{parse_collection_cards, parse_collections, parse_commander_skills, parse_consumables, parse_maps};
pub use ship::parse_ship_wiki;

