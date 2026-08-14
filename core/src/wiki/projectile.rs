//! Projectile parsing (`wowsinfo.json -> projectiles`).
//!
//! Every shell, bomb and torpedo in the game data is keyed by its internal
//! name (for example `PAPA011_Shell_406mm_AP_AP_Mk_8`). The parsed structs
//! feed the wiki's shell cards and the drag-based AP penetration chart.

use std::collections::HashMap;

use serde_json::Value;

/// Armor-piercing ballistic block (`projectiles.<name>.ap`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApInfo {
    pub diameter_m: f64,
    pub weight_kg: f64,
    pub drag: f64,
    pub velocity: f64,
    pub krupp: f64,
}

/// Acoustic homing block of a torpedo (`acousticDetection`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AcousticDetectionInfo {
    pub countdown: f64,
    pub max_depth_level: f64,
    pub max_pitch: f64,
    pub max_yaw: f64,
    pub path_length: f64,
    pub search_angle: f64,
    pub search_radius: f64,
    pub speed_decr_coef: f64,
    pub vertical_acceleration: f64,
    pub yaw_change_speed: f64,
}

/// One projectile entry (`projectiles.<name>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProjectileInfo {
    pub key: String,
    pub r#type: String,
    pub nation: String,
    /// Localisation key (`IDS_...`) resolved against the bundled lang data.
    pub name: String,
    pub ammo_type: String,
    pub speed: f64,
    pub weight: f64,
    pub damage: f64,
    /// Calibre in metres (0.406 for a 406 mm shell).
    pub diameter: f64,
    pub ricochet_angle: Option<f64>,
    pub ricochet_always: Option<f64>,
    pub pen_he: Option<f64>,
    pub pen_sap: Option<f64>,
    pub burn_chance: Option<f64>,
    pub overmatch: Option<i64>,
    pub fuse_time: Option<f64>,
    pub range: Option<f64>,
    pub flood_chance: Option<f64>,
    pub visibility: Option<f64>,
    pub alpha_damage: Option<f64>,
    pub deep_water: bool,
    pub ap: Option<ApInfo>,
    /// Torpedo fields (v15.7 `projectiles.<name>`).
    pub arming_distance: Option<f64>,
    pub depth: Option<f64>,
    pub splash_armor_coeff: Option<f64>,
    pub splash_cube_size: Option<f64>,
    pub underwater_splash_damage_multiplier: Option<f64>,
    /// Ping damage coefficient (`damageCoeffMaxPing`).
    pub damage_coeff_max_ping: Option<f64>,
    pub acoustic_detection: Option<AcousticDetectionInfo>,
    pub maneuver_dist: Option<f64>,
    /// Acoustic detection range (`alertDist`).
    pub alert_dist: Option<f64>,
    pub can_hit_classes: Vec<String>,
    /// Depth-charge ammo fields (v15.7 `projectiles.<name>`, type
    /// `DepthCharge`). `sinkSpeed` mirrors the game's `bulletSpeed` raw value.
    pub sink_speed: Option<f64>,
    pub detonation_depth: Option<f64>,
    pub splash_radius: Option<f64>,
    pub fire_chance: Option<f64>,
    pub flood_generation: Option<bool>,
    /// Depth range -> damage coefficient segments (`pointsOfDamage`).
    pub points_of_damage: Vec<(f64, f64)>,
    pub ignore_classes: Vec<String>,
    pub explosive_power: Option<f64>,
    pub integral_power: Option<f64>,
    pub fall_distance: Option<f64>,
    pub fall_time: Option<f64>,
    /// Buoyancy state -> damage coefficient (`buoyancyToDamageCoeff`).
    pub buoyancy_to_damage_coeff: Vec<(String, f64)>,
    // Ballistics fields (every projectile, zero gaps).
    pub air_drag: Option<f64>,
    pub arming_threshold: Option<f64>,
    pub cap_normalize_max_angle: Option<f64>,
    pub explosion_radius: Option<f64>,
    pub krupp: Option<f64>,
    pub shell_cap: Option<bool>,
    pub underwater_dist_factor: Option<f64>,
    pub underwater_penetration_factor: Option<f64>,
    pub water_drag: Option<f64>,
    /// Dispersion distance parameters (`distParams`, 4 values) + tile size.
    pub dist_params: Vec<f64>,
    pub dist_tile: Option<f64>,
}

impl ProjectileInfo {
    /// Calibre in millimetres (convenience for the penetration model).
    #[must_use]
    pub fn calibre_mm(&self) -> f64 {
        self.diameter * 1000.0
    }
}

fn f64_field(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn opt_f64(json: &Value, key: &str) -> Option<f64> {
    json.get(key)
        .filter(|v| !v.is_null())
        .and_then(Value::as_f64)
}

fn opt_i64(json: &Value, key: &str) -> Option<i64> {
    json.get(key)
        .filter(|v| !v.is_null())
        .and_then(Value::as_i64)
}

fn str_field(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

/// Parse a list of `[number, number]` pairs (e.g. `pointsOfDamage`).
fn f64_pairs(json: &Value, key: &str) -> Vec<(f64, f64)> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    let pair = v.as_array()?;
                    let first = pair.first()?.as_f64()?;
                    let second = pair.get(1)?.as_f64()?;
                    Some((first, second))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Parse a string list (e.g. `ignoreClasses`).
fn str_list(json: &Value, key: &str) -> Vec<String> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

/// Parse a `{key: number}` map into sorted pairs (e.g. `buoyancyToDamageCoeff`).
fn str_f64_map(json: &Value, key: &str) -> Vec<(String, f64)> {
    json.get(key)
        .and_then(Value::as_object)
        .map(|map| {
            let mut pairs: Vec<(String, f64)> = map
                .iter()
                .filter_map(|(k, v)| v.as_f64().map(|value| (k.clone(), value)))
                .collect();
            pairs.sort_by(|a, b| a.0.cmp(&b.0));
            pairs
        })
        .unwrap_or_default()
}

/// Parse a list of numbers (e.g. `distParams`).
fn f64_list(json: &Value, key: &str) -> Vec<f64> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_f64).collect())
        .unwrap_or_default()
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_ap_and_he_shells() {
        let json = json!({
            "PAPA011_Shell_406mm_AP_AP_Mk_8": {
                "type": "Artillery", "nation": "USA",
                "name": "IDS_PAPA011_SHELL_406MM_AP_AP_MK_8",
                "ammoType": "AP", "speed": 701.0, "weight": 1225.0,
                "damage": 13100.0, "ricochetAngle": 45.0,
                "ricochetAlways": 60.0, "diameter": 0.406,
                "ap": {"diameter": 0.406, "weight": 1225.0, "drag": 0.352,
                       "velocity": 701.0, "krupp": 2598.0},
                "overmatch": 28, "fuseTime": 0.033,
                "airDrag": 0.352, "armingThreshold": 68.0,
                "capNormalizeMaxAngle": 6.0, "explosionRadius": 200.0,
                "krupp": 2598.0, "shellCap": true, "splashRadius": 2.15,
                "underwaterDistFactor": 0.8, "underwaterPenetrationFactor": 0.9,
                "waterDrag": 10.0, "distParams": [-0.12, 0.1, 0.12, 0.0],
                "distTile": 3.0
            },
            "PAPA002_Shell_203mm_HE_HC_Mk_25": {
                "type": "Artillery", "nation": "USA",
                "name": "IDS_PAPA002_SHELL_203MM_HE_HC_MK_25",
                "ammoType": "HE", "speed": 823.0, "weight": 118.0,
                "damage": 2800.0, "penHE": 34.0, "burnChance": 0.14,
                "diameter": 0.203
            }
        });
        let map = parse_projectiles(&json);
        assert_eq!(map.len(), 2);
        let ap = &map["PAPA011_Shell_406mm_AP_AP_Mk_8"];
        assert_eq!(ap.ammo_type, "AP");
        assert_eq!(ap.overmatch, Some(28));
        assert_eq!(ap.calibre_mm(), 406.0);
        let ap_block = ap.ap.as_ref().expect("ap block");
        assert_eq!(ap_block.krupp, 2598.0);
        assert_eq!(ap_block.drag, 0.352);
        assert_eq!(ap.air_drag, Some(0.352));
        assert_eq!(ap.arming_threshold, Some(68.0));
        assert_eq!(ap.cap_normalize_max_angle, Some(6.0));
        assert_eq!(ap.explosion_radius, Some(200.0));
        assert_eq!(ap.krupp, Some(2598.0));
        assert_eq!(ap.shell_cap, Some(true));
        assert_eq!(ap.splash_radius, Some(2.15));
        assert_eq!(ap.underwater_dist_factor, Some(0.8));
        assert_eq!(ap.underwater_penetration_factor, Some(0.9));
        assert_eq!(ap.water_drag, Some(10.0));
        assert_eq!(ap.dist_params, vec![-0.12, 0.1, 0.12, 0.0]);
        assert_eq!(ap.dist_tile, Some(3.0));
        let he = &map["PAPA002_Shell_203mm_HE_HC_Mk_25"];
        assert_eq!(he.pen_he, Some(34.0));
        assert_eq!(he.burn_chance, Some(0.14));
        assert!(he.ap.is_none());
    }

    #[test]
    fn parses_torpedoes() {
        let json = json!({
            "PAPT001_Torpedo_533mm_Mk_15": {
                "type": "Torpedo", "nation": "USA",
                "name": "IDS_PAPT001_TORPEDO_533MM_MK_15",
                "speed": 55.0, "visibility": 1.1, "range": 305.0,
                "floodChance": 190.0, "alphaDamage": 32100.0,
                "damage": 900.0, "deepWater": false,
                "armingDistance": 55.0, "depth": 0.14,
                "splashArmorCoeff": 0.3, "splashCubeSize": 1.0,
                "damageCoeffMaxPing": 2.0, "alertDist": 100.0,
                "canHitClasses": ["Battleship", "Cruiser", "Destroyer"],
                "acousticDetection": {
                    "countdown": 1.0, "maxDepthLevel": 5.0, "maxPitch": 5.0,
                    "maxYaw": 5.0, "pathLength": 500, "searchAngle": 45,
                    "searchRadius": 2000, "speedDecrCoef": 0.8,
                    "verticalAcceleration": 5.0, "yawChangeSpeed": 5.0
                }
            }
        });
        let map = parse_projectiles(&json);
        let torp = &map["PAPT001_Torpedo_533mm_Mk_15"];
        assert_eq!(torp.r#type, "Torpedo");
        assert_eq!(torp.range, Some(305.0));
        assert_eq!(torp.flood_chance, Some(190.0));
        assert_eq!(torp.visibility, Some(1.1));
        assert_eq!(torp.alpha_damage, Some(32100.0));
        assert_eq!(torp.arming_distance, Some(55.0));
        assert_eq!(torp.depth, Some(0.14));
        assert_eq!(torp.splash_armor_coeff, Some(0.3));
        assert_eq!(torp.damage_coeff_max_ping, Some(2.0));
        assert_eq!(torp.alert_dist, Some(100.0));
        assert_eq!(torp.can_hit_classes, vec!["Battleship", "Cruiser", "Destroyer"]);
        let acoustic = torp.acoustic_detection.as_ref().expect("acoustic block");
        assert_eq!(acoustic.search_radius, 2000.0);
        assert_eq!(acoustic.speed_decr_coef, 0.8);
    }

    #[test]
    fn tolerant_of_missing_fields() {
        let json = json!({ "weird": {"name": "IDS_X"} });
        let map = parse_projectiles(&json);
        let entry = &map["weird"];
        assert_eq!(entry.damage, 0.0);
        assert!(entry.ap.is_none());
        assert!(entry.pen_he.is_none());
    }

    #[test]
    fn parses_depth_charge_ammo() {
        let json = json!({
            "PAPD004_mk6_shoot": {
                "type": "DepthCharge", "nation": "USA",
                "name": "IDS_PAPD004_MK6_SHOOT",
                "damage": 3800.0, "burnChance": 0.15,
                "floodChance": 23.0, "sinkSpeed": 300.0,
                "detonationDepth": -80, "splashRadius": 26.67,
                "floodGeneration": true,
                "pointsOfDamage": [[0.0, 1.0], [0.15, 1.0], [0.151, 0.33], [1.0, 0.33]],
                "ignoreClasses": ["AirCarrier", "Battleship", "Cruiser", "Destroyer", "Auxiliary"],
                "alertDist": 100.0, "explosivePower": 100.0, "integralPower": 10.0,
                "fallDistance": 20.0, "fallTime": 20.0,
                "buoyancyToDamageCoeff": {
                    "DEEP_WATER": 1.0, "PERISCOPE": 1.0, "SURFACE": 1.0
                }
            }
        });
        let map = parse_projectiles(&json);
        let dc = &map["PAPD004_mk6_shoot"];
        assert_eq!(dc.r#type, "DepthCharge");
        assert_eq!(dc.damage, 3800.0);
        assert_eq!(dc.burn_chance, Some(0.15));
        assert_eq!(dc.flood_chance, Some(23.0));
        assert_eq!(dc.sink_speed, Some(300.0));
        assert_eq!(dc.detonation_depth, Some(-80.0));
        assert_eq!(dc.splash_radius, Some(26.67));
        assert_eq!(dc.flood_generation, Some(true));
        assert_eq!(dc.points_of_damage, vec![(0.0, 1.0), (0.15, 1.0), (0.151, 0.33), (1.0, 0.33)]);
        assert_eq!(dc.ignore_classes.len(), 5);
        assert_eq!(dc.alert_dist, Some(100.0));
        assert_eq!(dc.explosive_power, Some(100.0));
        assert_eq!(dc.integral_power, Some(10.0));
        assert_eq!(dc.fall_distance, Some(20.0));
        assert_eq!(dc.fall_time, Some(20.0));
        assert_eq!(dc.buoyancy_to_damage_coeff.len(), 3);
    }
}
