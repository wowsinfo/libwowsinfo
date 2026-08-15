//! Wiki collection / card / map models.

use facet::Facet; use serde::{Deserialize, Serialize}; 
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

/// Wiki battle arena / map (`/wows/encyclopedia/battlearenas/`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Facet, PartialEq)]
pub struct WikiMap {
    #[serde(default)]
    pub arena_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
}
