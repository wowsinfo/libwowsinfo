//! Player, ship and rating models for the search -> stats flow.

use facet::Facet;
use serde::{Deserialize, Serialize};

/// Player profile from `/wows/account/info/`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlayerInfo {
    pub account_id: u64,
    pub nickname: String,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub last_battle_time: Option<i64>,
    #[serde(default)]
    pub leveling_tier: Option<i64>,
    #[serde(default)]
    pub hidden_profile: Option<bool>,
    #[serde(default)]
    pub logout_at: Option<i64>,
    #[serde(default)]
    pub statistics: Option<PlayerStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct PlayerStatistics {
    #[serde(default)]
    pub battles: i64,
    #[serde(default)]
    pub distance: i64,
    #[serde(default)]
    pub pvp: Option<PvpStats>,
    #[serde(default, rename = "pvp_solo")]
    pub solo: Option<PvpStats>,
    #[serde(default, rename = "pvp_div2")]
    pub div2: Option<PvpStats>,
    #[serde(default, rename = "pvp_div3")]
    pub div3: Option<PvpStats>,
    #[serde(default)]
    pub pve: Option<PvpStats>,
    #[serde(default, rename = "rank_solo")]
    pub rank_solo: Option<PvpStats>,
}

/// Per-ship stats from `/wows/ships/stats/`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShipStats {
    pub ship_id: u64,
    #[serde(default)]
    pub battles: i64,
    #[serde(default)]
    pub wins: i64,
    #[serde(default)]
    pub damage_dealt: i64,
    #[serde(default)]
    pub frags: i64,
    #[serde(default)]
    pub pvp: Option<PvpStats>,
    #[serde(default, rename = "pvp_solo")]
    pub solo: Option<PvpStats>,
    #[serde(default, rename = "pvp_div2")]
    pub div2: Option<PvpStats>,
    #[serde(default, rename = "pvp_div3")]
    pub div3: Option<PvpStats>,
    #[serde(default)]
    pub pve: Option<PvpStats>,
    #[serde(default, rename = "rank_solo")]
    pub rank_solo: Option<PvpStats>,
    #[serde(default)]
    pub last_battle_time: i64,
    // Computed by `getOverallRating` (written back onto the stats).
    #[serde(default, skip)]
    pub rating: f64,
    #[serde(default, skip)]
    pub ap: f64,
    #[serde(default, skip)]
    pub avg_dmg: f64,
    #[serde(default, skip)]
    pub avg_winrate: f64,
    #[serde(default, skip)]
    pub avg_frags: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct PvpStats {
    #[serde(default)]
    pub battles: i64,
    #[serde(default)]
    pub wins: i64,
    #[serde(default)]
    pub damage_dealt: i64,
    #[serde(default)]
    pub frags: i64,
    #[serde(default)]
    pub losses: i64,
    #[serde(default)]
    pub draws: i64,
    #[serde(default)]
    pub xp: i64,
    #[serde(default)]
    pub survived_battles: i64,
    #[serde(default)]
    pub survived_wins: i64,
    #[serde(default)]
    pub planes_killed: i64,
    #[serde(default)]
    pub ships_spotted: i64,
    #[serde(default)]
    pub max_damage_dealt: i64,
    #[serde(default)]
    pub max_frags_battle: i64,
    #[serde(default)]
    pub max_xp: i64,
    #[serde(default)]
    pub art_agro: i64,
    #[serde(default)]
    pub torpedo_agro: i64,
    #[serde(default)]
    pub capture_points: i64,
    #[serde(default)]
    pub dropped_capture_points: i64,
    #[serde(default)]
    pub team_capture_points: i64,
    #[serde(default)]
    pub team_dropped_capture_points: i64,
    #[serde(default)]
    pub max_planes_killed: i64,
    #[serde(default)]
    pub max_ships_spotted: i64,
    #[serde(default)]
    pub max_total_agro: i64,
    #[serde(default)]
    pub max_damage_scouting: i64,
    #[serde(default)]
    pub max_damage_dealt_to_buildings: i64,
    #[serde(default)]
    pub max_suppressions_count: i64,
    #[serde(default)]
    pub main_battery: Option<WeaponStats>,
    #[serde(default)]
    pub second_battery: Option<WeaponStats>,
    #[serde(default)]
    pub torpedoes: Option<WeaponStats>,
    #[serde(default)]
    pub aircraft: Option<WeaponStats>,
    #[serde(default)]
    pub ramming: Option<WeaponStats>,
}

/// Hit ratio data for one weapon group (main battery, torpedoes, ...).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct WeaponStats {
    #[serde(default)]
    pub shots: i64,
    #[serde(default)]
    pub hits: i64,
    #[serde(default)]
    pub frags: i64,
    #[serde(default)]
    pub max_frags_battle: i64,
}

/// One unlocked achievement for a player (`achievement_id` -> count).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct Achievement {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub count: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub icon: String,
}

/// One day of a player's recent stats (per-day delta from `statsbydate`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct RecentDay {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub battles: i64,
    #[serde(default)]
    pub winrate: f64,
    #[serde(default)]
    pub avg_damage: f64,
}

/// 10-day overview shown by the recent charts.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct RecentOverview {
    #[serde(default)]
    pub days: Vec<RecentDay>,
    #[serde(default)]
    pub total_battles: i64,
    #[serde(default)]
    pub avg_winrate: f64,
    #[serde(default)]
    pub avg_damage: f64,
}

/// Wiki entry for an achievement (name/icon), used to enrich the player's
/// unlocked list. Cached in key-value storage.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncyclopediaAchievement {
    #[serde(default, rename = "achievement_id")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "image")]
    pub icon: String,
}

/// One expected-value entry from `personal_rating.json` (`data.<ship_id>`).
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct PrEntry {
    #[serde(default)]
    pub average_damage_dealt: f64,
    #[serde(default)]
    pub average_frags: f64,
    #[serde(default)]
    pub win_rate: f64,
}

/// A full player view assembled by the core for the search -> stats flow.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct PlayerView {
    pub account_id: u64,
    pub nickname: String,
    pub server: String,
    pub rating: f64,
    pub rating_colour: String,
    pub rating_comment: String,
    pub ap: f64,
    #[serde(default)]
    pub hidden_profile: bool,
    #[serde(default)]
    pub ships: Vec<ShipStatLine>,
    #[serde(default)]
    pub statistics: PlayerStatistics,
    #[serde(default)]
    pub achievements: Vec<Achievement>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub last_battle_time: Option<i64>,
    #[serde(default)]
    pub leveling_tier: Option<i64>,
    #[serde(default)]
    pub logout_at: Option<i64>,
    #[serde(default)]
    pub clan_tag: String,
    #[serde(default)]
    pub recent: Option<RecentOverview>,
    #[serde(default)]
    pub rank: Option<super::rank::RankPlayerInfo>,
    #[serde(default)]
    pub rank_ships: Vec<super::rank::RankShipStat>,
    #[serde(default)]
    pub clan: Option<super::clan::ClanInfo>,
}

/// One row of the player's ship list with computed rating data.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShipStatLine {
    pub ship_id: u64,
    pub name: String,
    /// Local encyclopedia index (e.g. `PASB510`) for the bundled ship icon.
    #[serde(default)]
    pub index: String,
    pub tier: i64,
    pub r#type: String,
    pub nation: String,
    pub icon: String,
    pub premium: bool,
    pub battles: i64,
    pub avg_dmg: f64,
    pub avg_winrate: f64,
    pub avg_frags: f64,
    pub rating: f64,
    pub rating_colour: String,
    pub rating_comment: String,
    pub ap: f64,
    #[serde(default)]
    pub statistics: PlayerStatistics,
    #[serde(default)]
    pub expected_dmg: f64,
    #[serde(default)]
    pub expected_winrate: f64,
    #[serde(default)]
    pub expected_frags: f64,
    /// Last battle time for this ship (sort key in the player ship list).
    #[serde(default)]
    pub last_battle_time: i64,
}
