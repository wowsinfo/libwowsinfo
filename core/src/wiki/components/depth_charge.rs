//! Depth-charge component shapes.

use facet::Facet;
use serde::{Deserialize, Serialize};

/// Depth-charge pack settings (`depthCharge.packs`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargePackStats {
    pub num_shots: i64,
    pub shots_in_pack: i64,
    pub max_packs: i64,
    pub shot_delay: f64,
    pub guns_sequence_type: i64,
    pub center_zone_width_part: f64,
    pub use_shot_nodes_for_sequence: bool,
}

/// One depth-charge thrower (`depthCharge.launchers[i]`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargeLauncherStats {
    pub name: String,
    pub num_bombs: i64,
    pub shoot_angle: f64,
    pub shoot_dist: f64,
    pub start_fall_speed: f64,
    pub horiz_sector_min: f64,
    pub horiz_sector_max: f64,
    pub vert_sector_min: f64,
    pub vert_sector_max: f64,
    pub fall_roll_acceleration: f64,
    pub roll_speed: f64,
    pub rotation_speed_x: f64,
    pub rotation_speed_y: f64,
}

/// Depth charge component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargeStats {
    pub reload: f64,
    pub ammo: String,
    pub bombs: i64,
    pub groups: i64,
    pub packs: DepthChargePackStats,
    pub launchers: Vec<DepthChargeLauncherStats>,
}
