//! Local wiki data: game constants and the bundled `wowsinfo.json` game data.
//!
//! This is the "obtain the json and use it locally" mode: `wows-constants`
//! publishes a single `latest.json` of game constants, and the Flutter app
//! bundles `wowsinfo.json` from WoWs-Game-Data. Both are parsed here without
//! any API calls, mirroring `local_pr()`.

mod constants;
mod gamedata;
#[cfg(test)]
mod tests;

pub use constants::{
    parse_constants, BattleType, DeathReason, GameConstants, GameVersion,
};
pub use gamedata::{
    parse_game_data, AbilityInfo, AchievementInfo, CommanderSkill, ConsumableInfo, GameData,
    ShipInfo,
};
