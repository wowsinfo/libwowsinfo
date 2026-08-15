//! Ship loadout views: consumables, commander skills, module upgrades, flags
//! and the combined modifier set applied to the ship's stats.
//! Split into domain submodules so every file stays small.

mod combine;
mod consumables;
mod flags;
mod next_ships;
mod skills;
mod summary;
mod tests;
mod upgrades;
mod views;

pub use combine::combined_modifiers;
pub use consumables::consumable_views;
pub(crate) use consumables::alter_views;
pub use flags::flag_views;
pub use next_ships::next_ship_views;
pub use skills::skill_views;
pub(crate) use skills::skill_summary;
pub use summary::{modifier_lines, modifier_summary, modifier_summary_any};
pub use upgrades::upgrade_views;
pub use views::{
    ConsumableView, FlagView, LocalBuildConfig, NextShip, SkillView, UpgradeView,
};

