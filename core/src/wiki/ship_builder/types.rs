//! Ship builder types (module option, selection, computed build).

use std::collections::HashMap;

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::components::{
    AirDefenseStats, AirSupportStats, DepthChargeStats, EngineStats, FireControlStats, GunStats,
    HullStats, PingerStats, SpecialStats, TorpedoStats,
};

/// One module option (`modules.<slot>[i]`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ModuleOption {
    pub index: i64,
    /// Localisation key of the module name (`IDS_...`).
    pub name: String,
    pub cost_xp: i64,
    pub cost_cr: i64,
    /// Slot key -> component ids (e.g. `artillery -> ["A1_203_55"]`).
    pub components: HashMap<String, Vec<String>>,
}

/// The computed stats for the currently selected modules.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ShipBuild {
    pub hull: Option<HullStats>,
    pub main_battery: Option<GunStats>,
    pub secondaries: Option<GunStats>,
    pub torpedoes: Option<TorpedoStats>,
    pub air_defense: Option<AirDefenseStats>,
    pub fire_control: Option<FireControlStats>,
    pub engine: Option<EngineStats>,
    pub depth_charges: Option<DepthChargeStats>,
    pub air_support: Option<AirSupportStats>,
    pub pinger: Option<PingerStats>,
    pub special: Option<SpecialStats>,
}

/// Per-module selection indices (defaults to the stock modules).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModuleSelection {
    pub hull: usize,
    pub artillery: usize,
    pub torpedoes: usize,
    pub fire_control: usize,
    pub engine: usize,
    pub fighter: usize,
    pub torpedo_bomber: usize,
    pub dive_bomber: usize,
    pub skip_bomber: usize,
}

