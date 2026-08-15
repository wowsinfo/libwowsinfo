//! Hull component parser (`parse_hull`).

use serde_json::Value;

use super::helpers::{as_f64, as_i64, depth_value, num_key_f64_table, str_f64_map_sorted};
use super::hull::{
    parse_fire_flood, parse_submarine_mobility, section_label, ArmorStats, ArmorZone, BarbetteArmor,
    ConcealmentStats, HpSectionStats, HullStats, ManeuverabilityRaw, ManeuverabilityStats,
    MobilityStats, SubmarineBatteryStats, SurvivabilityStats, VisibilityStats, SECTION_ORDER,
};

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
        maneuverability: json.get("maneuverability").filter(|v| !v.is_null()).map(|m| {
            let raw = m
                .get("raw")
                .filter(|v| v.is_object())
                .map(|raw| ManeuverabilityRaw {
                    engine_power: as_f64(raw, "enginePower"),
                    side_drag_coef: as_f64(raw, "sideDragCoef"),
                    backward_movement_drag_coef: as_f64(raw, "backwardMovementDragCoef"),
                    backward_power_coef: as_f64(raw, "backwardPowerCoef"),
                    cooling_off_speed: as_f64(raw, "coolingOffSpeed"),
                    speed_coef: as_f64(raw, "speedCoef"),
                    max_rudder_angle: as_f64(raw, "maxRudderAngle"),
                    rudder_power: as_f64(raw, "rudderPower"),
                    underwater_max_rudder_angle: as_f64(raw, "underwaterMaxRudderAngle"),
                });
            ManeuverabilityStats {
                max_reverse_speed: as_f64(m, "maxReverseSpeed"),
                submarine: parse_submarine_mobility(
                    m.get("submarine").unwrap_or(&Value::Null),
                ),
                raw,
            }
        }),
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
        concealment: json.get("concealment").filter(|v| !v.is_null()).map(|c| {
            ConcealmentStats {
                sea_fire: as_f64(c, "seaFire"),
                air_fire: as_f64(c, "airFire"),
                periscope_depth: depth_value(
                    c.get("bySubmarineDepth").unwrap_or(&Value::Null),
                    "PERISCOPE",
                ),
                deep_water_depth: depth_value(
                    c.get("bySubmarineDepth").unwrap_or(&Value::Null),
                    "DEEP_WATER",
                ),
                smoke_factor: as_f64(c, "visibilityFactorInSmoke"),
                by_submarine_depth: str_f64_map_sorted(c, "bySubmarineDepth"),
                smoke_factor_gk: as_f64(c, "visibilityCoefGKInSmoke"),
                visibility_coef_gk_by_plane: as_f64(c, "visibilityCoefGKByPlane"),
                underwater_depth_coeff: num_key_f64_table(c, "visibilityCoeffUnderwaterDepths"),
                underwater_depth_coeff_plane: num_key_f64_table(
                    c,
                    "visibilityCoeffUnderwaterDepthsByPlane",
                ),
                deepwater_vision_coeff: str_f64_map_sorted(c, "deepwaterVisionCoeff"),
                deepwater_vision_to_plane_coeff: str_f64_map_sorted(
                    c,
                    "deepwaterVisionToPlaneCoeff",
                ),
            }
        }),
        armor: json.get("armor").filter(|v| !v.is_null()).map(|a| {
            let zone_map = a.get("zones").and_then(Value::as_object);
            let mut zones: Vec<ArmorZone> = a
                .get("zones")
                .and_then(Value::as_object)
                .map(|map| {
                    map.iter()
                        .map(|(id, value)| ArmorZone {
                            zone_id: id.clone(),
                            thickness: value.as_f64().unwrap_or(0.0),
                        })
                        .collect()
                })
                .unwrap_or_default();
            zones.sort_by(|x, y| y.thickness.total_cmp(&x.thickness).then(x.zone_id.cmp(&y.zone_id)));
            let mut barbettes: Vec<BarbetteArmor> = a
                .get("barbettes")
                .and_then(Value::as_object)
                .map(|map| {
                    map.iter()
                        .map(|(turret, ids)| {
                            let max_thickness = ids
                                .as_array()
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|id| {
                                            let zone_id = id
                                                .as_u64()
                                                .map(|n| n.to_string())
                                                .or_else(|| id.as_str().map(str::to_string))?;
                                            zone_map
                                                .and_then(|m| m.get(&zone_id))
                                                .and_then(Value::as_f64)
                                        })
                                        .fold(0.0, f64::max)
                                })
                                .unwrap_or(0.0);
                            BarbetteArmor {
                                turret: turret.clone(),
                                max_thickness,
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();
            barbettes.sort_by(|x, y| x.turret.cmp(&y.turret));
            ArmorStats { zones, barbettes }
        }),
        submarine_battery: json
            .get("submarineBattery")
            .filter(|v| !v.is_null())
            .map(|b| SubmarineBatteryStats {
                capacity: as_i64(b, "capacity"),
                regen: as_f64(b, "regen"),
            }),
        survivability: json.get("survivability").filter(|v| !v.is_null()).map(|s| {
            let mut sections: Vec<HpSectionStats> = s
                .get("sections")
                .and_then(Value::as_object)
                .map(|map| {
                    map.iter()
                        .filter_map(|(key, value)| {
                            let hp = value.get("hp").and_then(Value::as_f64)?;
                            Some(HpSectionStats {
                                name: section_label(key),
                                hp,
                                regen_ratio: as_f64(value, "regenRatio"),
                                auto_repair_time: as_f64(value, "autoRepairTime"),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            sections.sort_by_key(|section| {
                SECTION_ORDER
                    .iter()
                    .position(|name| section.name.eq_ignore_ascii_case(name))
                    .unwrap_or(usize::MAX)
            });
            SurvivabilityStats {
                sections,
                fire: parse_fire_flood(s.get("fire").unwrap_or(&Value::Null)),
                flood: parse_fire_flood(s.get("flood").unwrap_or(&Value::Null)),
            }
        }),
    }
}
