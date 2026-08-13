//! Typed ship component parsers (`ships.<id>.components`).
//!
//! Every component id in the game data maps to one of the shapes below;
//! following the wows-toolkit convention each shape is one typed struct.

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A weapon group inside an artillery/torpedo component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct WeaponInfo {
    pub reload: f64,
    pub rotation: f64,
    pub each: i64,
    pub ammo: Vec<String>,
    pub vert_sector: f64,
    pub count: i64,
}

/// Burst-fire block (`guns[].burst`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct BurstInfo {
    pub burst_reload_time: f64,
    pub full_reload_time: f64,
    pub shot_intensity: f64,
    pub shots_count: i64,
}

/// An artillery component (`range`, `sigma`, `guns`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct GunStats {
    pub range_m: f64,
    pub sigma: f64,
    pub guns: Vec<WeaponInfo>,
    pub burst: Option<BurstInfo>,
}

/// One AA aura band (`near`/`medium`/`far`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AuraInfo {
    pub min_range: f64,
    pub max_range: f64,
    pub hit_chance: f64,
    pub damage: f64,
    pub rof: f64,
    pub dps: f64,
    pub guns: Vec<WeaponInfo>,
}

/// Torpedo component (`singleShot` + `launchers`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TorpedoStats {
    pub single_shot: bool,
    pub launchers: Vec<WeaponInfo>,
}

/// AA component with its three bands.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirDefenseStats {
    pub near: Vec<AuraInfo>,
    pub medium: Vec<AuraInfo>,
    pub far: Vec<AuraInfo>,
}

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
    pub visibility: VisibilityStats,
    pub submarine_battery: Option<SubmarineBatteryStats>,
}

/// Depth charge component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargeStats {
    pub reload: f64,
    pub ammo: String,
    pub bombs: i64,
    pub groups: i64,
}

/// Air support component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirSupportStats {
    pub name: String,
    pub charges_num: i64,
    pub plane: String,
    pub reload: f64,
    pub range: f64,
}

/// Submarine sonar (pinger) component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct PingerStats {
    pub reload: f64,
    pub range: f64,
    pub life_time1: f64,
    pub life_time2: f64,
    pub speed: f64,
}

/// Tier-11 rage-mode component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SpecialStats {
    pub boost_duration: f64,
    pub decrement_count: i64,
    pub decrement_delay: f64,
    pub decrement_period: f64,
    pub guns_for_salvo: i64,
    pub radius: f64,
    pub rage_mode_name: String,
    pub required_hits: i64,
}

fn as_f64(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn as_i64(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}

/// Parse one `guns`/`launchers` entry.
pub(crate) fn parse_weapon(json: &Value) -> WeaponInfo {
    WeaponInfo {
        reload: as_f64(json, "reload"),
        rotation: as_f64(json, "rotation"),
        each: as_i64(json, "each"),
        ammo: json
            .get("ammo")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default(),
        vert_sector: as_f64(json, "vertSector"),
        count: as_i64(json, "count"),
    }
}

/// Parse an artillery/secondary component.
pub(crate) fn parse_guns(json: &Value) -> GunStats {
    GunStats {
        range_m: as_f64(json, "range"),
        sigma: as_f64(json, "sigma"),
        guns: json
            .get("guns")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().map(parse_weapon).collect())
            .unwrap_or_default(),
        burst: json.get("burst").filter(|v| !v.is_null()).map(|burst| BurstInfo {
            burst_reload_time: as_f64(burst, "burstReloadTime"),
            full_reload_time: as_f64(burst, "fullReloadTime"),
            shot_intensity: as_f64(burst, "shotIntensity"),
            shots_count: as_i64(burst, "shotsCount"),
        }),
    }
}

/// Parse one AA aura band entry.
pub(crate) fn parse_aura(json: &Value) -> AuraInfo {
    AuraInfo {
        min_range: as_f64(json, "minRange"),
        max_range: as_f64(json, "maxRange"),
        hit_chance: as_f64(json, "hitChance"),
        damage: as_f64(json, "damage"),
        rof: as_f64(json, "rof"),
        dps: as_f64(json, "dps"),
        guns: json
            .get("guns")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().map(parse_weapon).collect())
            .unwrap_or_default(),
    }
}

/// Parse an AA component's `near`/`medium`/`far` bands.
pub(crate) fn parse_band(json: &Value, key: &str) -> Vec<AuraInfo> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().map(parse_aura).collect())
        .unwrap_or_default()
}

/// Parse a hull component.
pub(crate) fn parse_hull(json: &Value) -> HullStats {
    HullStats {
        health: as_f64(json, "health"),
        protection: as_f64(json, "protection"),
        mobility: {
            let m = json.get("mobility").unwrap_or(&Value::Null);
            MobilityStats {
                speed: as_f64(m, "speed"),
                turning_radius: as_f64(m, "turningRadius"),
                rudder_time: as_f64(m, "rudderTime"),
            }
        },
        visibility: {
            let v = json.get("visibility").unwrap_or(&Value::Null);
            VisibilityStats {
                sea: as_f64(v, "sea"),
                plane: as_f64(v, "plane"),
                sea_in_smoke: as_f64(v, "seaInSmoke"),
                plane_in_smoke: as_f64(v, "planeInSmoke"),
                submarine: as_f64(v, "submarine"),
                sea_fire_coeff: as_f64(v, "seaFireCoeff"),
                plane_fire_coeff: as_f64(v, "planeFireCoeff"),
            }
        },
        submarine_battery: json
            .get("submarineBattery")
            .filter(|v| !v.is_null())
            .map(|b| SubmarineBatteryStats {
                capacity: as_i64(b, "capacity"),
                regen: as_f64(b, "regen"),
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_and_burst_parse() {
        let json = serde_json::json!({
            "range": 14699.0, "sigma": 2.0,
            "guns": [{"reload": 15.0, "rotation": 25.7, "each": 3,
                      "ammo": ["PAPA002", "PAPA001"], "vertSector": 41.0, "count": 3}],
            "burst": {"burstReloadTime": 4.0, "fullReloadTime": 30.0,
                      "shotIntensity": 0.5, "shotsCount": 4}
        });
        let guns = parse_guns(&json);
        assert_eq!(guns.range_m, 14699.0);
        assert_eq!(guns.guns.len(), 1);
        assert_eq!(guns.guns[0].each, 3);
        assert_eq!(guns.guns[0].ammo, vec!["PAPA002", "PAPA001"]);
        let burst = guns.burst.expect("burst");
        assert_eq!(burst.shots_count, 4);
    }

    #[test]
    fn hull_and_aa_parse() {
        let hull = serde_json::json!({
            "health": 30500.0, "protection": 4.0,
            "mobility": {"speed": 32.5, "turningRadius": 660.0, "rudderTime": 9.0},
            "visibility": {"sea": 11.5, "plane": 6.0},
            "submarineBattery": {"capacity": 10, "regen": 0.5}
        });
        let parsed = parse_hull(&hull);
        assert_eq!(parsed.health, 30500.0);
        assert_eq!(parsed.mobility.speed, 32.5);
        assert_eq!(parsed.submarine_battery.as_ref().map(|b| b.capacity), Some(10));

        let aa = serde_json::json!({
            "medium": [{"minRange": 0.1, "maxRange": 3.5, "hitChance": 0.9,
                        "damage": 37.0, "rof": 0.29, "dps": 129.5,
                        "guns": [{"ammo": "PAGA002", "each": 2, "reload": 5.0,
                                  "name": "IDS_X", "count": 6}]}]
        });
        let band = parse_band(&aa, "medium");
        assert_eq!(band.len(), 1);
        assert_eq!(band[0].dps, 129.5);
        assert_eq!(band[0].guns[0].count, 6);
    }
}
