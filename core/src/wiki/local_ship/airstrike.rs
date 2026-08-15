//! Airstrike / ASW view model.

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::components::AirSupportStats;

/// Resolved airstrike / ASW panel.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirstrikeView {
    pub auto_usage: bool,
    pub charges: i64,
    pub reload_s: f64,
    /// Strike range in km (component `range`).
    pub range_km: f64,
    /// Minimum strike distance in metres.
    pub min_dist_m: f64,
    /// Maximum strike distance in metres.
    pub max_dist_m: f64,
    /// Maximum flight distance of the strike plane in metres.
    pub max_plane_flight_dist_m: f64,
    pub climb_angle_deg: f64,
    pub fly_away_time_s: f64,
    pub time_between_shots_s: f64,
    /// Time the strike plane spends over the target ("time from heaven").
    pub time_from_heaven_s: f64,
}

pub(super) fn airstrike_view(stats: &AirSupportStats) -> AirstrikeView {
    AirstrikeView {
        auto_usage: stats.airstrike.auto_usage,
        charges: stats.charges_num.max(stats.airstrike.charges_num),
        reload_s: stats.reload.max(stats.airstrike.reload_time),
        range_km: stats.range,
        min_dist_m: stats.airstrike.min_dist,
        max_dist_m: stats.airstrike.max_dist,
        max_plane_flight_dist_m: stats.airstrike.max_plane_flight_dist,
        climb_angle_deg: stats.airstrike.climb_angle,
        fly_away_time_s: stats.airstrike.fly_away_time,
        time_between_shots_s: stats.airstrike.time_between_shots,
        time_from_heaven_s: stats.airstrike.time_from_heaven,
    }
}

