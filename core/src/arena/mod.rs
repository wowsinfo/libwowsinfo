//! Live-battle arena models and parsing.
//!
//! The WoWs-RS desktop companion reads `replays/tempArenaInfo.json` (the file
//! the game writes for the current match) and serves it over LAN. This module
//! parses that file into typed models and derives the team split and the
//! player lookups the app needs, mirroring the app's RS client.

use serde::{Deserialize, Serialize};

mod parser;
#[cfg(test)]
mod tests;

pub use parser::{is_bot, parse_arena, stat_lookups, team_of, teams};

/// Which team a player is on. The arena file's `relation` is 0 or 1 for the
/// friendly side and 2 for the enemy side (the app treats `< 2` as friends).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Team {
    Ally,
    Enemy,
}

/// One entry from `tempArenaInfo.json` `vehicles[]`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct ArenaVehicle {
    #[serde(default, rename = "shipId")]
    pub ship_id: u64,
    #[serde(default)]
    pub relation: u8,
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
}

/// A parsed `tempArenaInfo.json` document.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct ArenaInfo {
    #[serde(default, rename = "gameMode")]
    pub game_mode: i64,
    #[serde(default, rename = "mapName")]
    pub map_name: String,
    #[serde(default, rename = "mapDisplayName")]
    pub map_display_name: String,
    #[serde(default, rename = "playersPerTeam")]
    pub players_per_team: u64,
    #[serde(default, rename = "teamsCount")]
    pub teams_count: u64,
    #[serde(default)]
    pub duration: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub scenario: String,
    #[serde(default, rename = "gameType")]
    pub game_type: String,
    #[serde(default, rename = "dateTime")]
    pub date_time: String,
    #[serde(default, rename = "playerID")]
    pub player_id: u64,
    #[serde(default, rename = "playerName")]
    pub player_name: String,
    #[serde(default, rename = "playerVehicle")]
    pub player_vehicle: String,
    #[serde(default)]
    pub vehicles: Vec<ArenaVehicle>,
}
