//! Ship encyclopedia models.

use facet::Facet; use serde::{Deserialize, Serialize}; 
/// Raw `/wows/encyclopedia/ships/` entry before `getWarship` post-processing.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RawEncyclopediaShip {
    pub ship_id: u64,
    #[serde(default)]
    pub index: String,
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
    pub index: String,
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
            index: raw.index,
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
