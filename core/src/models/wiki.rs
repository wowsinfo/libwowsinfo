//! Ship and wiki models (`/wows/encyclopedia/*`).

use facet::Facet;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
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
    #[serde(default)]
    pub new: Option<bool>,
    #[serde(default)]
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

/// Wiki collection (`/wows/encyclopedia/collections/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct WikiCollection {
    #[serde(default)]
    pub collection_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub image: String,
}

/// Wiki collection card (`/wows/encyclopedia/collectioncards/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct CollectionCard {
    #[serde(default)]
    pub card_id: u64,
    #[serde(default)]
    pub collection_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub image: String,
}

/// Wiki consumable (`/wows/encyclopedia/consumables/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct Consumable {
    #[serde(default)]
    pub consumable_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub price_credit: i64,
    #[serde(default)]
    pub price_gold: i64,
    #[serde(default)]
    pub profile: Vec<ConsumableProfile>,
}

/// Consumable profile description.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ConsumableProfile {
    #[serde(default)]
    pub description: String,
}

/// Commander skill (`/wows/encyclopedia/crewskills/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct CommanderSkill {
    #[serde(default)]
    pub skill_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub tier: i64,
    #[serde(default)]
    pub type_id: i64,
    #[serde(default)]
    pub type_name: String,
    #[serde(default)]
    pub perks: Vec<Perk>,
}

/// One commander-skill perk.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct Perk {
    #[serde(default)]
    pub perk_id: u64,
    #[serde(default)]
    pub description: String,
}

/// Full wiki ship entry from `/wows/encyclopedia/ships/?ship_id=`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct ShipWiki {
    #[serde(default)]
    pub ship_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub nation: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub tier: i64,
    #[serde(default)]
    pub is_premium: bool,
    #[serde(default)]
    pub price_credit: i64,
    #[serde(default)]
    pub price_gold: i64,
    #[serde(default)]
    pub next_ships: Vec<u64>,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub profile: ShipProfile,
}

/// The ship's default profile with the wiki stat blocks.
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

/// Main battery profile with its shells.
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
