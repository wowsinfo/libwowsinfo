//! Comparison table types.

use facet::Facet;
use serde::{Deserialize, Serialize};

/// One ship column of the comparison table.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct CompareShipHeader {
    pub ship_id: u64,
    pub index: String,
    pub name: String,
    pub tier: i64,
}

/// A similar ship (same tier and type).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SimilarShip {
    pub ship_id: u64,
    pub index: String,
    pub name: String,
    pub tier: i64,
    pub nation: String,
    pub ship_type: String,
}

/// One stat row aligned with `LocalCompare.ships`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct CompareRow {
    pub label: String,
    pub values: Vec<String>,
}

/// The full comparison table.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct LocalCompare {
    pub ships: Vec<CompareShipHeader>,
    pub rows: Vec<CompareRow>,
}


pub(super) const ROW_LABELS: [&str; 15] = [
    "Tier",
    "Type",
    "Nation",
    "Health",
    "Speed",
    "Rudder",
    "Concealment",
    "Main battery",
    "Gun range",
    "Reload",
    "Sigma",
    "Torpedo range",
    "Torpedo damage",
    "AA DPS",
    "Secondaries",
];
