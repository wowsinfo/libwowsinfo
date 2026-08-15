//! Loadout view structs.

use std::collections::HashSet;

use facet::Facet;
use serde::{Deserialize, Serialize};

/// Selection state for skills, upgrades, flags and the simulated conditions.
#[derive(Debug, Clone, Default)]
pub struct LocalBuildConfig {
    pub skills: HashSet<String>,
    pub upgrades: HashSet<String>,
    pub flags: HashSet<String>,
    /// 0..1, drives low-HP skills like Adrenaline Rush.
    pub hp_fraction: f64,
    /// True while the ship is spotted (drives trigger skills).
    pub spotted: bool,
}

/// One consumable variant shown on the ship detail.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ConsumableView {
    pub key: String,
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub reload_s: f64,
    pub work_s: f64,
    /// -1 means unlimited charges.
    pub charges: i64,
    /// Ship-specific alter variants (name/description only).
    pub alters: Vec<ConsumableAlterView>,
}

/// One alter variant of a consumable (`abilities.<id>.alter.<key>`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ConsumableAlterView {
    pub key: String,
    pub name: String,
    pub description: String,
}

/// One commander skill of the ship's class.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SkillView {
    pub key: String,
    pub name: String,
    pub description: String,
    pub tier: i64,
    pub trigger_type: String,
    pub selected: bool,
    pub summary: String,
}

/// One applicable module upgrade (modernization).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct UpgradeView {
    pub key: String,
    pub name: String,
    pub description: String,
    pub slot: i64,
    pub cost_cr: i64,
    pub selected: bool,
    pub summary: String,
}

/// One signal flag.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct FlagView {
    pub key: String,
    pub name: String,
    pub description: String,
    pub cost_cr: i64,
    pub selected: bool,
    pub summary: String,
}

/// A ship in the research tree (`next_ships`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct NextShip {
    pub ship_id: u64,
    pub index: String,
    pub name: String,
    pub tier: i64,
}
