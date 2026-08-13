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
