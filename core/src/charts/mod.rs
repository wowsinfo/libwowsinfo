//! Chart data computed from a player's ship list.
//!
//! Pure calculations for the player-info charts: the stats-vs-average radar,
//! per-class averages, the top ships list and the game-mode distribution.

mod averages;
mod modes;
mod radar;
#[cfg(test)]
mod tests;
mod top;

pub use averages::{per_class_averages, ClassAverage};
pub use modes::{mode_distribution, ModeBattles};
pub use radar::{stats_vs_average, RadarValues};
pub use top::{top_ships, TopShip};
