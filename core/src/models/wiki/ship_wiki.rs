//! Ship wiki entry model.

use facet::Facet; use serde::{Deserialize, Serialize};

use super::profile::ShipProfile;
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
