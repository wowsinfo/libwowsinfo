//! Airstrike / air-support component shapes.

use facet::Facet;
use serde::{Deserialize, Serialize};

/// Airstrike module settings (v15.7 `airstrike` block).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirstrikeStats {
    pub auto_usage: bool,
    pub charges_num: i64,
    pub climb_angle: f64,
    pub fly_away_time: f64,
    pub max_dist: f64,
    pub max_plane_flight_dist: f64,
    pub min_dist: f64,
    pub reload_time: f64,
    pub time_between_shots: f64,
    pub time_from_heaven: f64,
}

/// Air support component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirSupportStats {
    pub name: String,
    pub charges_num: i64,
    pub plane: String,
    pub reload: f64,
    pub range: f64,
    pub airstrike: AirstrikeStats,
}
