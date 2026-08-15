//! Typed ship component parsers (`ships.<id>.components`).
//!
//! Every component id in the game data maps to one of the shapes below;
//! following the wows-toolkit convention each shape is one typed struct.
//! Split into domain submodules so every file stays small.

mod aa;
mod airstrike;
mod depth_charge;
mod guns;
mod helpers;
mod hull;
mod hull_parse;
mod pinger;
mod special;
mod torpedoes;

pub(crate) use aa::parse_air_defense;
pub use aa::{AirDefenseStats, AuraInfo};
pub use airstrike::{AirSupportStats, AirstrikeStats};
pub use depth_charge::{DepthChargeLauncherStats, DepthChargePackStats, DepthChargeStats};
pub(crate) use guns::{parse_guns, parse_weapon};
pub use guns::{BurstInfo, DispersionStats, GunStats, WeaponInfo};
#[cfg(test)]
pub use guns::BatteryStats;
pub(crate) use hull_parse::parse_hull;
pub use hull::{EngineStats, FireControlStats, HullStats, MobilityStats, SubmarineBatteryStats, VisibilityStats};
pub use pinger::PingerStats;
pub(crate) use special::parse_special;
pub use special::SpecialStats;
pub use torpedoes::TorpedoStats;

#[cfg(test)]
mod tests;

