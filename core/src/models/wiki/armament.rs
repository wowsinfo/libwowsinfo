//! Ship armament profile models.

use facet::Facet; use serde::{Deserialize, Serialize}; 
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ArtilleryProfile {
    #[serde(default)]
    pub slots: Vec<GunSlot>,
    #[serde(default)]
    pub shells: Vec<ShellInfo>,
    #[serde(default)]
    pub gun_rate: f64,
    #[serde(default)]
    pub max_dispersion: i64,
    #[serde(default)]
    pub distance: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct GunSlot {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub barrels: i64,
    #[serde(default)]
    pub guns: i64,
}

/// One shell type (HE/AP) of the main battery.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShellInfo {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub damage: i64,
    #[serde(default)]
    pub bullet_mass: f64,
    #[serde(default)]
    pub bullet_speed: f64,
    #[serde(default)]
    pub burn_probability: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct TorpedoProfile {
    #[serde(default)]
    pub distance: f64,
    #[serde(default)]
    pub shells: Vec<ShellInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct AntiAircraftProfile {
    #[serde(default)]
    pub defense: i64,
    #[serde(default)]
    pub slots: Vec<AntiAircraftSlot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct AntiAircraftSlot {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub caliber: i64,
    #[serde(default)]
    pub guns: i64,
    #[serde(default)]
    pub avg_damage: Option<i64>,
}
