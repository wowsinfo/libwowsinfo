//! Ranked-season models (`/wows/seasons/accountinfo/` and `shipstats/`).

use std::collections::HashMap;

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::player::PvpStats;

/// Ranked-season data for a player (`/wows/seasons/accountinfo/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct RankPlayerInfo {
    #[serde(default)]
    pub account_id: u64,
    #[serde(default)]
    pub seasons: HashMap<String, RankSeason>,
}

/// One ranked season: stats per rank bucket (`ranks`, keyed by the API's
/// rank key such as `-1`) plus the season's progress from the account-level
/// `rank_info` block.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct RankSeason {
    #[serde(default)]
    pub ranks: HashMap<String, RankSeasonMode>,
    #[serde(default)]
    pub rank_info: Option<RankInfo>,
}

/// The ranked stats earned at one rank key (`-1` aggregates all ranks).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct RankSeasonMode {
    #[serde(default)]
    pub rank_solo: Option<PvpStats>,
    #[serde(default)]
    pub rank_div2: Option<PvpStats>,
    #[serde(default)]
    pub rank_div3: Option<PvpStats>,
}

/// Progress within a ranked season.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct RankInfo {
    #[serde(default)]
    pub max_rank: i64,
    #[serde(default)]
    pub start_rank: i64,
    #[serde(default)]
    pub stars: i64,
    #[serde(default)]
    pub rank: i64,
    #[serde(default)]
    pub stage: i64,
}

/// Per-ship ranked stats (`/wows/seasons/shipstats/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct RankShipStat {
    #[serde(default)]
    pub ship_id: u64,
    #[serde(default)]
    pub seasons: HashMap<String, RankSeason>,
}
