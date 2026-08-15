//! Hull component shapes (survivability, armor, mobility, concealment).

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::helpers::{as_f64, as_i64};

/// Fire control component (`maxDistCoef`, `sigmaCountCoef`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct FireControlStats {
    pub max_dist_coef: f64,
    pub sigma_count_coef: f64,
}

/// Engine component (`speedCoef`, usually empty).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct EngineStats {
    pub speed_coef: f64,
}

/// Mobility block of a hull component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct MobilityStats {
    pub speed: f64,
    pub turning_radius: f64,
    pub rudder_time: f64,
}

/// The hull `maneuverability` block (newer game data).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ManeuverabilityStats {
    pub max_reverse_speed: f64,
    pub submarine: Option<SubmarineMobilityStats>,
    /// Raw engine/drag coefficients (`maneuverability.raw`). Acceleration
    /// times are not directly computable from the bundle (no tonnage or
    /// engine up-times), but the raw coefficients are shipped per hull.
    pub raw: Option<ManeuverabilityRaw>,
}

/// Raw engine/drag coefficients (`maneuverability.raw`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ManeuverabilityRaw {
    pub engine_power: f64,
    pub side_drag_coef: f64,
    pub backward_movement_drag_coef: f64,
    pub backward_power_coef: f64,
    pub cooling_off_speed: f64,
    pub speed_coef: f64,
    pub max_rudder_angle: f64,
    pub rudder_power: f64,
    pub underwater_max_rudder_angle: f64,
}

/// Submarine speed modes (`maneuverability.submarine`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SubmarineMobilityStats {
    pub surface_speed: f64,
    pub surface_reverse: f64,
    pub periscope_speed: f64,
    pub periscope_reverse: f64,
    pub max_depth_speed: f64,
    pub max_depth_reverse: f64,
    pub dive_speed: f64,
    pub diving_plane_shift_time: f64,
}

/// Visibility block of a hull component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct VisibilityStats {
    pub sea: f64,
    pub plane: f64,
    pub sea_in_smoke: f64,
    pub plane_in_smoke: f64,
    pub submarine: f64,
    pub sea_fire_coeff: f64,
    pub plane_fire_coeff: f64,
}

/// The hull `concealment` block (newer game data).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ConcealmentStats {
    pub sea_fire: f64,
    pub air_fire: f64,
    pub periscope_depth: f64,
    pub deep_water_depth: f64,
    pub smoke_factor: f64,
    /// `bySubmarineDepth` full table (state -> detectability in km).
    pub by_submarine_depth: Vec<(String, f64)>,
    pub smoke_factor_gk: f64,
    pub visibility_coef_gk_by_plane: f64,
    /// Depth (m) -> visibility coefficient (`visibilityCoeffUnderwaterDepths`).
    pub underwater_depth_coeff: Vec<(f64, f64)>,
    pub underwater_depth_coeff_plane: Vec<(f64, f64)>,
    pub deepwater_vision_coeff: Vec<(String, f64)>,
    pub deepwater_vision_to_plane_coeff: Vec<(String, f64)>,
}

/// Submarine battery block.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SubmarineBatteryStats {
    pub capacity: i64,
    pub regen: f64,
}

/// Hull component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct HullStats {
    pub health: f64,
    pub protection: f64,
    pub mobility: MobilityStats,
    pub maneuverability: Option<ManeuverabilityStats>,
    pub visibility: VisibilityStats,
    pub concealment: Option<ConcealmentStats>,
    pub armor: Option<ArmorStats>,
    pub submarine_battery: Option<SubmarineBatteryStats>,
    /// HP sections + fire/flood model (newer game data, optional).
    pub survivability: Option<SurvivabilityStats>,
}

/// One HP section (citadel, casemate, bow, ...) with its regen ratio.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct HpSectionStats {
    pub name: String,
    pub hp: f64,
    pub regen_ratio: f64,
    pub auto_repair_time: f64,
}

/// Fire or flood damage model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct FireFloodStats {
    pub spots: i64,
    pub chance: f64,
    pub duration: f64,
    pub dps: f64,
    pub total_damage: f64,
}

/// The hull `survivability` block (HP sections + fire/flood).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SurvivabilityStats {
    pub sections: Vec<HpSectionStats>,
    pub fire: Option<FireFloodStats>,
    pub flood: Option<FireFloodStats>,
}

/// One hull armor zone (`armor.zones`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ArmorZone {
    pub zone_id: String,
    pub thickness: f64,
}

/// Barbette armor for one turret (`armor.barbettes`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct BarbetteArmor {
    pub turret: String,
    pub max_thickness: f64,
}

/// The hull `armor` block (zones + barbette associations).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ArmorStats {
    pub zones: Vec<ArmorZone>,
    pub barbettes: Vec<BarbetteArmor>,
}

pub(super) const SECTION_ORDER: [&str; 7] = [
    "citadel",
    "casemate",
    "bow",
    "stern",
    "superstructure",
    "auxiliaryRooms",
    "hull",
];

pub(super) fn section_label(key: &str) -> String {
    match key {
        "citadel" => "Citadel".to_string(),
        "casemate" => "Casemate".to_string(),
        "bow" => "Bow".to_string(),
        "stern" => "Stern".to_string(),
        "superstructure" => "Superstructure".to_string(),
        "auxiliaryRooms" => "Auxiliary Rooms".to_string(),
        "hull" => "Hull".to_string(),
        _ => {
            let mut chars = key.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        }
    }
}

pub(super) fn parse_fire_flood(json: &Value) -> Option<FireFloodStats> {
    if json.is_null() {
        return None;
    }
    Some(FireFloodStats {
        spots: as_i64(json, "spots"),
        chance: as_f64(json, "chance"),
        duration: as_f64(json, "duration"),
        dps: as_f64(json, "dps"),
        total_damage: as_f64(json, "totalDamage"),
    })
}

pub(super) fn parse_submarine_mobility(json: &Value) -> Option<SubmarineMobilityStats> {
    if json.is_null() {
        return None;
    }
    Some(SubmarineMobilityStats {
        surface_speed: as_f64(json, "maxSpeedAtSurface"),
        surface_reverse: as_f64(json, "maxReverseSpeedAtSurface"),
        periscope_speed: as_f64(json, "maxSpeedAtPeriscope"),
        periscope_reverse: as_f64(json, "maxReverseSpeedAtPeriscope"),
        max_depth_speed: as_f64(json, "maxSpeedAtMaxDepth"),
        max_depth_reverse: as_f64(json, "maxReverseSpeedAtMaxDepth"),
        dive_speed: as_f64(json, "maxDiveSpeed"),
        diving_plane_shift_time: as_f64(json, "divingPlaneShiftTime"),
    })
}


