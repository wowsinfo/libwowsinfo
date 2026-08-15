//! Adjusted ship stats after modifiers.

use facet::Facet;
use serde::{Deserialize, Serialize};

/// Stats after applying the selected modifiers, conditions and HP level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Facet)]
pub struct AdjustedStats {
    pub health: f64,
    pub gun_reload_s: f64,
    pub gun_range_m: f64,
    pub gun_rotation_deg_s: f64,
    pub torp_reload_s: f64,
    pub torp_rotation_deg_s: f64,
    pub secondary_reload_s: f64,
    pub secondary_range_m: f64,
    pub speed: f64,
    pub rudder_time: f64,
    pub concealment_sea: f64,
    pub concealment_air: f64,
    pub aa_dps: f64,
    pub battery_capacity: f64,
    pub battery_regen: f64,
    /// Multipliers for consumable reload / work time (applied per consumable).
    pub consumable_reload_mult: f64,
    pub consumable_work_mult: f64,
    /// Extra consumable charges (additive) and capacity multiplier.
    pub consumable_charges_extra: f64,
    pub consumable_capacity_mult: f64,
    pub pinger_reload_s: f64,
    pub pinger_speed: f64,
}

impl Default for AdjustedStats {
    fn default() -> Self {
        Self {
            health: 0.0,
            gun_reload_s: 0.0,
            gun_range_m: 0.0,
            gun_rotation_deg_s: 0.0,
            torp_reload_s: 0.0,
            torp_rotation_deg_s: 0.0,
            secondary_reload_s: 0.0,
            secondary_range_m: 0.0,
            speed: 0.0,
            rudder_time: 0.0,
            concealment_sea: 0.0,
            concealment_air: 0.0,
            aa_dps: 0.0,
            battery_capacity: 0.0,
            battery_regen: 0.0,
            consumable_reload_mult: 1.0,
            consumable_work_mult: 1.0,
            consumable_charges_extra: 0.0,
            consumable_capacity_mult: 1.0,
            pinger_reload_s: 0.0,
            pinger_speed: 0.0,
        }
    }
}
