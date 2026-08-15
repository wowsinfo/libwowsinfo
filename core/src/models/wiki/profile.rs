//! Ship profile models (default profile).

use facet::Facet; use serde::{Deserialize, Serialize};

use super::armament::{AntiAircraftProfile, ArtilleryProfile, TorpedoProfile};
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShipProfile {
    #[serde(default)]
    pub armour: ShipArmour,
    #[serde(default)]
    pub mobility: ShipMobility,
    #[serde(default)]
    pub concealment: ShipConcealment,
    #[serde(default)]
    pub weaponry: ShipWeaponry,
    #[serde(default)]
    pub artillery: Option<ArtilleryProfile>,
    #[serde(default)]
    pub torpedoes: Option<TorpedoProfile>,
    #[serde(default)]
    pub anti_aircraft: Option<AntiAircraftProfile>,
    #[serde(default)]
    pub hull: HullProfile,
    #[serde(default)]
    pub engine: EngineProfile,
}

/// Armour summary (thickness values are -1 when not applicable).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShipArmour {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub health: i64,
    #[serde(default)]
    pub citadel: MinMax,
    #[serde(default)]
    pub extremities: MinMax,
    #[serde(default)]
    pub casemate: MinMax,
    #[serde(default)]
    pub deck: MinMax,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct MinMax {
    #[serde(default)]
    pub min: i64,
    #[serde(default)]
    pub max: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShipMobility {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub max_speed: f64,
    #[serde(default)]
    pub turning_radius: i64,
    #[serde(default)]
    pub rudder_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShipConcealment {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub detect_distance_by_ship: f64,
    #[serde(default)]
    pub detect_distance_by_plane: f64,
    #[serde(default)]
    pub detect_distance_by_submarine: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShipWeaponry {
    #[serde(default)]
    pub artillery: i64,
    #[serde(default)]
    pub torpedoes: i64,
    #[serde(default)]
    pub anti_aircraft: i64,
    #[serde(default)]
    pub aircraft: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct HullProfile {
    #[serde(default)]
    pub health: i64,
    #[serde(default)]
    pub artillery_barrels: i64,
    #[serde(default)]
    pub torpedoes_barrels: i64,
    #[serde(default)]
    pub anti_aircraft_barrels: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct EngineProfile {
    #[serde(default)]
    pub max_speed: f64,
}
