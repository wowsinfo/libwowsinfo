//! Aircraft parsing (`wowsinfo.json -> aircrafts`).

use std::collections::HashMap;

use serde_json::Value;

use super::helpers::{f64_field, i64_field, opt_f64, opt_i64, plane_consumables, speed_limits, str_field};
use super::types::AircraftInfo;

/// Parse the `aircrafts` section into a map keyed by aircraft name.
#[must_use]
pub fn parse_aircrafts(json: &Value) -> HashMap<String, AircraftInfo> {
    let mut out = HashMap::new();
    let Some(map) = json.as_object() else {
        return out;
    };
    for (key, value) in map {
        let aircraft = value.get("aircraft").filter(|v| !v.is_null());
        let aiming_limits = speed_limits(value, "aimingSpeedLimits");
        let preparation_limits = speed_limits(value, "preparationSpeedLimits");
        out.insert(
            key.clone(),
            AircraftInfo {
                key: key.clone(),
                r#type: str_field(value, "type"),
                nation: str_field(value, "nation"),
                name: str_field(value, "name"),
                health: f64_field(value, "health"),
                total_planes: i64_field(value, "totalPlanes"),
                visibility: f64_field(value, "visibility"),
                speed: f64_field(value, "speed"),
                // v15.7 flattens these to the top level (`attackerSize`,
                // `restorationTime`, `attackCount`, `bombName`); older shapes
                // nest them under `aircraft`. Prefer the flat form, fall back
                // to the nested block.
                attack_count: aircraft
                    .and_then(|a| a.get("attackCount"))
                    .or_else(|| value.get("attackCount"))
                    .and_then(Value::as_i64),
                attacker: aircraft
                    .and_then(|a| a.get("attacker"))
                    .or_else(|| value.get("attackerSize"))
                    .and_then(Value::as_i64),
                max_aircraft: aircraft.and_then(|a| a.get("maxAircraft").and_then(Value::as_i64)),
                restore_time: aircraft
                    .and_then(|a| a.get("restoreTime"))
                    .or_else(|| value.get("restorationTime"))
                    .and_then(Value::as_f64),
                bomb_name: aircraft
                    .and_then(|a| a.get("bombName"))
                    .or_else(|| value.get("bombName"))
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned),
                attack_cooldown: opt_f64(value, "attackCooldown"),
                attack_interval: opt_f64(value, "attackInterval"),
                aiming_time: opt_f64(value, "aimingTime"),
                aiming_speed_limit_min: aiming_limits.0,
                aiming_speed_limit_max: aiming_limits.1,
                aiming_accuracy_increase_rate: opt_f64(value, "aimingAccuracyIncreaseRate"),
                aiming_accuracy_decrease_rate: opt_f64(value, "aimingAccuracyDecreaseRate"),
                aiming_turn_speed_limit: opt_f64(value, "aimingTurnSpeedLimit"),
                preparation_time: opt_f64(value, "preparationTime"),
                preparation_speed_limit_min: preparation_limits.0,
                preparation_speed_limit_max: preparation_limits.1,
                preparation_turn_speed_limit: opt_f64(value, "preparationTurnSpeedLimit"),
                climb_speed_coef: opt_f64(value, "climbSpeedCoef"),
                dive_speed_coef: opt_f64(value, "diveSpeedCoef"),
                angle_of_climb: opt_f64(value, "angleOfClimb"),
                angle_of_dive: opt_f64(value, "angleOfDive"),
                post_attack_invulnerability_duration: opt_f64(
                    value,
                    "postAttackInvulnerabilityDuration",
                ),
                jato_duration: opt_f64(value, "jatoDuration"),
                jato_speed_multiplier: opt_f64(value, "jatoSpeedMultiplier"),
                max_forsage_amount: opt_f64(value, "maxForsageAmount"),
                forsage_regeneration: opt_f64(value, "forsageRegeneration"),
                speed_min: opt_f64(value, "speedMin"),
                speed_max: opt_f64(value, "speedMax"),
                visibility_factor_by_plane: opt_f64(value, "visibilityFactorByPlane"),
                attacker_damage_taken_multiplier: opt_f64(value, "attackerDamageTakenMultiplier"),
                damage_taken_multiplier: opt_f64(value, "damageTakenMultiplier"),
                bomb_falling_time: opt_f64(value, "bombFallingTime"),
                bombing_drop_point_time: opt_f64(value, "bombingDropPointTime"),
                empty_return_speed_multiplier: opt_f64(value, "emptyReturnSpeedMultiplier"),
                max_rotate_speed: opt_f64(value, "maxRotateSpeed"),
                plane_speedup_coef: opt_f64(value, "planeSpeedupCoef"),
                can_stop: value.get("canStop").and_then(Value::as_bool).unwrap_or(false),
                max_number_on_deck: opt_i64(value, "maxNumberOnDeck"),
                restoration_time: opt_f64(value, "restorationTime"),
                restore_amount: opt_i64(value, "restoreAmount"),
                start_on_deck: opt_i64(value, "startOnDeck"),
                plane_consumables: plane_consumables(value),
            },
        );
    }
    out
}

