//! Serde models mirroring the Wargaming/Wiki API shapes consumed by the
//! TypeScript app, with the same tolerant fallbacks as `SafeValue`/`Guard`.

use std::collections::HashMap;

use facet::Facet;
use serde::{Deserialize, Serialize};

/// Standard Wargaming API envelope: `{status, meta, data, error}`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub data: Option<T>,
    #[serde(default)]
    pub meta: Option<ApiMeta>,
    #[serde(default)]
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiMeta {
    #[serde(default)]
    pub page_total: Option<u64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiError {
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub code: Option<u64>,
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
}

/// One hit from `/wows/account/list/` (player search).
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountListEntry {
    pub account_id: u64,
    pub nickname: String,
}

/// Player profile from `/wows/account/info/`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlayerInfo {
    pub account_id: u64,
    pub nickname: String,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub hidden_profile: Option<bool>,
    #[serde(default)]
    pub statistics: Option<PlayerStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlayerStatistics {
    #[serde(default)]
    pub pvp: Option<PvpStats>,
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PvpStats {
    #[serde(default)]
    pub battles: i64,
    #[serde(default)]
    pub wins: i64,
    #[serde(default)]
    pub damage_dealt: i64,
    #[serde(default)]
    pub frags: i64,
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

/// Raw `/wows/encyclopedia/ships/` entry before `getWarship` post-processing.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RawEncyclopediaShip {
    pub ship_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nation: String,
    #[serde(default, rename = "type")]
    pub r#type: String,
    #[serde(default)]
    pub tier: i64,
    #[serde(default)]
    pub images: Option<Images>,
    #[serde(default)]
    pub is_premium: bool,
    #[serde(default)]
    pub is_special: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Images {
    #[serde(default)]
    pub small: String,
}

/// Post-processed ship entry matching the app's unique data format
/// (`icon`, `premium`, `new`, optional `model`).
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncyclopediaShip {
    pub ship_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nation: String,
    #[serde(default, rename = "type")]
    pub r#type: String,
    #[serde(default)]
    pub tier: i64,
    #[serde(default)]
    pub premium: bool,
    #[serde(default)]
    pub icon: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl From<RawEncyclopediaShip> for EncyclopediaShip {
    fn from(raw: RawEncyclopediaShip) -> Self {
        Self {
            ship_id: raw.ship_id,
            name: raw.name,
            nation: raw.nation,
            r#type: raw.r#type,
            tier: raw.tier,
            premium: raw.is_premium || raw.is_special,
            icon: raw.images.map(|i| i.small).unwrap_or_default(),
            new: None,
            model: None,
        }
    }
}

/// Encyclopedia metadata (`/wows/encyclopedia/info/` with `ship_nations`,
/// `ship_types`) used by the warship filter to map display names to IDs.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncyclopediaInfo {
    #[serde(default)]
    pub ship_nations: HashMap<String, String>,
    #[serde(default)]
    pub ship_types: HashMap<String, String>,
    #[serde(default)]
    pub ship_modules: serde_json::Value,
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
}

/// One row of the player's ship list with computed rating data.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShipStatLine {
    pub ship_id: u64,
    pub name: String,
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
    pub ap: f64,
}
