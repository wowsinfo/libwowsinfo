//! Wiki dataset and search-result types.

use facet::Facet;
use serde::{Deserialize, Serialize};

/// A wiki dataset that can be loaded on demand (`/wows/encyclopedia/*`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Facet)]
#[repr(C)]
pub enum WikiDataset {
    Collections,
    CollectionCards,
    Consumables,
    CommanderSkills,
    Maps,
}

/// One search hit shown in the UI.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Facet, PartialEq)]
pub struct SearchResult {
    pub account_id: u64,
    pub nickname: String,
}
