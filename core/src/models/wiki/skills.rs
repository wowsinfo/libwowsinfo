//! Commander-skill wiki models.

use facet::Facet; use serde::{Deserialize, Serialize}; 
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
