//! UI state as seen by the shells.

use std::collections::HashMap;

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::datasets::SearchResult;
use crate::models;
use crate::wiki;

/// UI state as seen by the shells.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Facet, PartialEq)]
pub struct ViewModel {
    pub phase: Phase,
    #[serde(default)]
    pub search_results: Vec<SearchResult>,
    #[serde(default)]
    pub player: Option<models::PlayerView>,
    #[serde(default)]
    pub clan_search_results: Vec<models::ClanSearchResult>,
    #[serde(default)]
    pub selected_clan: Option<models::ClanInfo>,
    /// Players online across the game; -1 when the request failed.
    #[serde(default)]
    pub online: i64,
    #[serde(default)]
    pub warship: HashMap<u64, models::EncyclopediaShip>,
    #[serde(default)]
    pub wiki_collections: HashMap<u64, models::WikiCollection>,
    #[serde(default)]
    pub wiki_collection_cards: HashMap<u64, models::CollectionCard>,
    #[serde(default)]
    pub wiki_consumables: HashMap<u64, models::Consumable>,
    #[serde(default)]
    pub wiki_commander_skills: HashMap<u64, models::CommanderSkill>,
    #[serde(default)]
    pub wiki_maps: HashMap<u64, models::WikiMap>,
    #[serde(default)]
    pub selected_ship_wiki: Option<models::ShipWiki>,
    #[serde(default)]
    pub local_ship: Option<wiki::LocalShipWiki>,
    #[serde(default)]
    pub local_compare: Option<wiki::LocalCompare>,
    #[serde(default)]
    pub local_consumables: Vec<wiki::ConsumableView>,
    #[serde(default)]
    pub local_skills_wiki: Vec<wiki::LocalSkillWikiEntry>,
    #[serde(default)]
    pub local_achievements: Vec<wiki::LocalAchievementEntry>,
    #[serde(default)]
    pub local_upgrades_wiki: Vec<wiki::LocalUpgradeEntry>,
    #[serde(default)]
    pub local_flags_wiki: Vec<wiki::LocalFlagEntry>,
    /// True once the bundled game data has been parsed successfully.
    #[serde(default)]
    pub local_data_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Facet, PartialEq)]
#[repr(C)]
pub enum Phase {
    Idle,
    Searching,
    LoadingPlayer,
    Player,
    Error(String),
}

impl Default for Phase {
    fn default() -> Self {
        Self::Idle
    }
}
