//! Clan models (`/wows/clans/list/` and `/wows/clans/info/`).

use facet::Facet;
use serde::{Deserialize, Serialize};

/// One clan search hit (`/wows/clans/list/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ClanSearchResult {
    #[serde(default)]
    pub clan_id: u64,
    #[serde(default)]
    pub tag: String,
}

/// Full clan info with members (`/wows/clans/info/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ClanInfo {
    #[serde(default)]
    pub clan_id: u64,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub members_count: i64,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
    #[serde(default)]
    pub leader_id: i64,
    #[serde(default)]
    pub leader_name: String,
    #[serde(default)]
    pub creator_id: i64,
    #[serde(default)]
    pub creator_name: String,
    #[serde(default)]
    pub old_name: String,
    #[serde(default)]
    pub old_tag: String,
    #[serde(default)]
    pub renamed_at: i64,
    #[serde(default)]
    pub is_clan_disbanded: bool,
    #[serde(default)]
    pub members: Vec<ClanMember>,
}

/// One clan member entry.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ClanMember {
    #[serde(default)]
    pub account_id: u64,
    #[serde(default)]
    pub account_name: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub joined_at: i64,
}
