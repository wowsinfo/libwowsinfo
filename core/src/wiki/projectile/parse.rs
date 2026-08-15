//! Projectile parsing (`wowsinfo.json -> projectiles`).

use std::collections::HashMap;

use serde_json::Value;

use super::helpers::{f64_field, f64_list, f64_pairs, opt_f64, opt_i64, str_field, str_f64_map, str_list};
use super::types::{AcousticDetectionInfo, ApInfo, ProjectileInfo};

/// Parse the `projectiles` section into a map keyed by projectile name.
#[must_use]
pub fn parse_projectiles(json: &Value) -> HashMap<String, ProjectileInfo> {
    let mut out = HashMap::new();
    let Some(map) = json.as_object() else {
        return out;
    };
    for (key, value) in map {
        let ap = value.get("ap").filter(|v| !v.is_null()).map(|ap| ApInfo {
            diameter_m: f64_field(ap, "diameter"),
            weight_kg: f64_field(ap, "weight"),
            drag: f64_field(ap, "drag"),
            velocity: f64_field(ap, "velocity"),
            krupp: f64_field(ap, "krupp"),
        });
        out.insert(
            key.clone(),
            ProjectileInfo {
                key: key.clone(),
                r#type: str_field(value, "type"),
                nation: str_field(value, "nation"),
                name: str_field(value, "name"),
                ammo_type: str_field(value, "ammoType"),
                speed: f64_field(value, "speed"),
                weight: f64_field(value, "weight"),
                damage: f64_field(value, "damage"),
                diameter: f64_field(value, "diameter"),
                ricochet_angle: opt_f64(value, "ricochetAngle"),
                ricochet_always: opt_f64(value, "ricochetAlways"),
                pen_he: opt_f64(value, "penHE"),
                pen_sap: opt_f64(value, "penSAP"),
                burn_chance: opt_f64(value, "burnChance"),
                overmatch: opt_i64(value, "overmatch"),
                fuse_time: opt_f64(value, "fuseTime"),
                range: opt_f64(value, "range"),
                flood_chance: opt_f64(value, "floodChance"),
                visibility: opt_f64(value, "visibility"),
                alpha_damage: opt_f64(value, "alphaDamage"),
                deep_water: value
                    .get("deepWater")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                ap,
                arming_distance: opt_f64(value, "armingDistance"),
                depth: opt_f64(value, "depth"),
                splash_armor_coeff: opt_f64(value, "splashArmorCoeff"),
                splash_cube_size: opt_f64(value, "splashCubeSize"),
                underwater_splash_damage_multiplier: opt_f64(
                    value,
                    "underwaterSplashDamageMultiplier",
                ),
                damage_coeff_max_ping: opt_f64(value, "damageCoeffMaxPing"),
                acoustic_detection: value
                    .get("acousticDetection")
                    .filter(|v| v.is_object())
                    .map(|ad| AcousticDetectionInfo {
                        countdown: f64_field(ad, "countdown"),
                        max_depth_level: f64_field(ad, "maxDepthLevel"),
                        max_pitch: f64_field(ad, "maxPitch"),
                        max_yaw: f64_field(ad, "maxYaw"),
                        path_length: f64_field(ad, "pathLength"),
                        search_angle: f64_field(ad, "searchAngle"),
                        search_radius: f64_field(ad, "searchRadius"),
                        speed_decr_coef: f64_field(ad, "speedDecrCoef"),
                        vertical_acceleration: f64_field(ad, "verticalAcceleration"),
                        yaw_change_speed: f64_field(ad, "yawChangeSpeed"),
                    }),
                maneuver_dist: opt_f64(value, "maneuverDist"),
                alert_dist: opt_f64(value, "alertDist"),
                can_hit_classes: value
                    .get("canHitClasses")
                    .and_then(Value::as_array)
                    .map(|arr| {
                        arr.iter()
                            .filter_map(Value::as_str)
                            .map(ToOwned::to_owned)
                            .collect()
                    })
                    .unwrap_or_default(),
                sink_speed: opt_f64(value, "sinkSpeed"),
                detonation_depth: opt_f64(value, "detonationDepth"),
                splash_radius: opt_f64(value, "splashRadius"),
                fire_chance: opt_f64(value, "fireChance"),
                flood_generation: value.get("floodGeneration").and_then(Value::as_bool),
                points_of_damage: f64_pairs(value, "pointsOfDamage"),
                ignore_classes: str_list(value, "ignoreClasses"),
                explosive_power: opt_f64(value, "explosivePower"),
                integral_power: opt_f64(value, "integralPower"),
                fall_distance: opt_f64(value, "fallDistance"),
                fall_time: opt_f64(value, "fallTime"),
                buoyancy_to_damage_coeff: str_f64_map(value, "buoyancyToDamageCoeff"),
                air_drag: opt_f64(value, "airDrag"),
                arming_threshold: opt_f64(value, "armingThreshold"),
                cap_normalize_max_angle: opt_f64(value, "capNormalizeMaxAngle"),
                explosion_radius: opt_f64(value, "explosionRadius"),
                krupp: opt_f64(value, "krupp"),
                shell_cap: value.get("shellCap").and_then(Value::as_bool),
                underwater_dist_factor: opt_f64(value, "underwaterDistFactor"),
                underwater_penetration_factor: opt_f64(value, "underwaterPenetrationFactor"),
                water_drag: opt_f64(value, "waterDrag"),
                dist_params: f64_list(value, "distParams"),
                dist_tile: opt_f64(value, "distTile"),
            },
        );
    }
    out
}

