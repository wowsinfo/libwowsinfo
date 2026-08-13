//! Serde models mirroring the Wargaming/Wiki API shapes consumed by the
//! TypeScript app, with the same tolerant fallbacks as `SafeValue`/`Guard`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod clan;
mod player;
mod rank;
mod wiki;

pub use clan::{ClanInfo, ClanMember, ClanSearchResult};
pub use player::{
    Achievement, EncyclopediaAchievement, PlayerInfo, PlayerStatistics, PlayerView, PrEntry,
    PvpStats, RecentDay, RecentOverview, ShipStatLine, ShipStats, WeaponStats,
};
pub use rank::{RankInfo, RankPlayerInfo, RankSeason, RankShipStat};
pub use wiki::{
    AntiAircraftProfile, AntiAircraftSlot, ArtilleryProfile, CollectionCard, CommanderSkill,
    Consumable, ConsumableProfile, EncyclopediaShip, EngineProfile, GunSlot, HullProfile, Images,
    MinMax, Perk, RawEncyclopediaShip, ShellInfo, ShipArmour, ShipConcealment, ShipMobility,
    ShipProfile, ShipWeaponry, ShipWiki, TorpedoProfile, WikiCollection,
};

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
