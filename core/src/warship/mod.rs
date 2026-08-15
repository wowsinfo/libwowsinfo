//! Warship filtering (RN `WarshipFilter` parity).
//! Split into domain submodules so every file stays small.

mod core;
#[cfg(test)]
mod tests;

pub use core::{filter_ships, filter_warships, get_colour_with_range, get_key_by_value, get_tier_label, normalise, NormalisedFilter, ShipFilter};

