//! Submarine sonar (pinger) component shape.

use facet::Facet;
use serde::{Deserialize, Serialize};

/// Submarine sonar (pinger) component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct PingerStats {
    pub reload: f64,
    pub range: f64,
    pub life_time1: f64,
    pub life_time2: f64,
    pub speed: f64,
}
