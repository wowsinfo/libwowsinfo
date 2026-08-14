//! Local wiki data: game constants and the bundled `wowsinfo.json` game data.
//!
//! This is the "obtain the json and use it locally" mode: `wows-constants`
//! publishes a single `latest.json` of game constants, and the Flutter app
//! bundles `wowsinfo.json` from WoWs-Game-Data. Both are parsed here without
//! any API calls, mirroring `local_pr()`.

use std::io::Read;

mod aircraft;
mod aircraft_views;
mod compare;
mod constants;
mod components;
mod gamedata;
#[cfg(test)]
mod tests;
mod lang;
mod local_ship;
#[cfg(test)]
mod local_ship_tests;
mod loadouts;
mod modifiers;
mod penetration;
mod projectile;
mod ship_builder;
mod wiki_lists;
pub use constants::{
    parse_constants, BattleType, DeathReason, GameConstants, GameVersion,
};
pub use aircraft::{parse_aircrafts, AircraftInfo};
pub use aircraft_views::{
    air_support_plane, aircraft_slot_views, AircraftDetail, AircraftOptionView, AircraftSlotView,
};
pub use compare::{
    build_local_compare, similar_ships, CompareRow, CompareShipHeader, LocalCompare, SimilarShip,
};
pub use gamedata::{
    parse_game_data, AbilityInfo, AchievementInfo, CommanderSkill, ConsumableInfo, GameData,
    ShipInfo,
};
pub use components::{
    AirDefenseStats, AirSupportStats, AuraInfo, BurstInfo, DepthChargeStats, EngineStats,
    FireControlStats, GunStats, HullStats, MobilityStats, PingerStats, SpecialStats,
    SubmarineBatteryStats, TorpedoStats, VisibilityStats, WeaponInfo,
};
pub use lang::{parse_lang, LangMap};
pub use local_ship::{
    build_local_ship_wiki, LocalShipWiki, MainBatteryView, ModuleOptionView, ModuleSlotView,
    PenCurveView, ShellView, TorpedoView,
};
pub use loadouts::{
    combined_modifiers, consumable_views, flag_views, modifier_summary, next_ship_views,
    skill_views, upgrade_views, ConsumableView, FlagView, LocalBuildConfig, NextShip, SkillView,
    UpgradeView,
};
pub use modifiers::{
    apply_modifiers, parse_modifiers, AdjustedStats, ModifierSet, ModifierValue,
};
pub use penetration::{
    overmatch_mm, penetration_curve, solve_for_range, BallisticShell, PenetrationPoint,
};
pub use projectile::{parse_projectiles, ApInfo, ProjectileInfo};
pub use wiki_lists::{all_consumable_views, all_skill_views, LocalSkillWikiEntry};
pub use ship_builder::{
    build_ship_build, module_slots, ModuleOption, ModuleSelection, ShipBuild,
};

/// Decompress a zstd frame in memory into a UTF-8 string.
///
/// The bundled assets are `wowsinfo.zst` / `lang.zst` (zstd level 19);
/// decompression happens on the Rust side so the app only has to ship and
/// pass the compressed bytes.
pub fn decompress_zstd(bytes: &[u8]) -> std::io::Result<String> {
    let mut decoder = zstd::stream::read::Decoder::new(bytes)?;
    let mut out = String::new();
    decoder.read_to_string(&mut out)?;
    Ok(out)
}
