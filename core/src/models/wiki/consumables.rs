//! Wiki consumable models.

use facet::Facet; use serde::{Deserialize, Serialize}; 
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
