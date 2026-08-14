//! Aircraft parsing (`wowsinfo.json -> aircrafts`).
//!
//! Carriers reference aircraft keys from their squadrons; the parsed structs
//! feed the wiki's carrier section and the air-support cards.

use std::collections::HashMap;

use serde_json::Value;

/// One aircraft entry (`aircrafts.<name>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AircraftInfo {
    pub key: String,
    pub r#type: String,
    pub nation: String,
    /// Localisation key (`IDS_...`).
    pub name: String,
    pub health: f64,
    pub total_planes: i64,
    pub visibility: f64,
    pub speed: f64,
    pub attack_count: Option<i64>,
    pub attacker: Option<i64>,
    pub max_aircraft: Option<i64>,
    pub restore_time: Option<f64>,
    pub bomb_name: Option<String>,
    // Top-level squadron fields (v15.7 `aircrafts.<name>`).
    pub attack_cooldown: Option<f64>,
    pub attack_interval: Option<f64>,
    pub aiming_time: Option<f64>,
    pub aiming_speed_limit_min: Option<f64>,
    pub aiming_speed_limit_max: Option<f64>,
    pub aiming_accuracy_increase_rate: Option<f64>,
    pub aiming_accuracy_decrease_rate: Option<f64>,
    pub aiming_turn_speed_limit: Option<f64>,
    pub preparation_time: Option<f64>,
    pub preparation_speed_limit_min: Option<f64>,
    pub preparation_speed_limit_max: Option<f64>,
    pub preparation_turn_speed_limit: Option<f64>,
    pub climb_speed_coef: Option<f64>,
    pub dive_speed_coef: Option<f64>,
    pub angle_of_climb: Option<f64>,
    pub angle_of_dive: Option<f64>,
    pub post_attack_invulnerability_duration: Option<f64>,
    pub jato_duration: Option<f64>,
    pub jato_speed_multiplier: Option<f64>,
    pub max_forsage_amount: Option<f64>,
    pub forsage_regeneration: Option<f64>,
    pub speed_min: Option<f64>,
    pub speed_max: Option<f64>,
    pub visibility_factor_by_plane: Option<f64>,
    pub attacker_damage_taken_multiplier: Option<f64>,
    pub damage_taken_multiplier: Option<f64>,
    pub bomb_falling_time: Option<f64>,
    pub bombing_drop_point_time: Option<f64>,
    pub empty_return_speed_multiplier: Option<f64>,
    pub max_rotate_speed: Option<f64>,
    pub plane_speedup_coef: Option<f64>,
    pub can_stop: bool,
    pub max_number_on_deck: Option<i64>,
    pub restoration_time: Option<f64>,
    pub restore_amount: Option<i64>,
    pub start_on_deck: Option<i64>,
    pub plane_consumables: Vec<PlaneConsumableSlot>,
}

/// One plane consumable slot (`planeConsumables.AbilitySlotN`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlaneConsumableSlot {
    pub slot: i64,
    pub abilities: Vec<String>,
    pub special: bool,
}

fn f64_field(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn i64_field(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}

fn str_field(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
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

fn speed_limits(json: &Value, key: &str) -> (Option<f64>, Option<f64>) {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            (
                arr.first().and_then(Value::as_f64),
                arr.get(1).and_then(Value::as_f64),
            )
        })
        .unwrap_or((None, None))
}

fn plane_consumables(json: &Value) -> Vec<PlaneConsumableSlot> {
    let mut slots: Vec<PlaneConsumableSlot> = json
        .get("planeConsumables")
        .and_then(Value::as_object)
        .map(|map| {
            map.values()
                .filter_map(|v| {
                    let abils = v.get("abils").and_then(Value::as_array)?;
                    Some(PlaneConsumableSlot {
                        slot: v.get("slot").and_then(Value::as_i64).unwrap_or(0),
                        abilities: abils
                            .iter()
                            .filter_map(|entry| {
                                entry
                                    .as_array()
                                    .and_then(|pair| pair.first())
                                    .and_then(Value::as_str)
                                    .map(ToOwned::to_owned)
                            })
                            .collect(),
                        special: v
                            .get("isSpecial")
                            .and_then(Value::as_bool)
                            .unwrap_or(false),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    slots.sort_by_key(|slot| slot.slot);
    slots
}

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
                attack_count: aircraft.and_then(|a| {
                    a.get("attackCount")
                        .and_then(Value::as_i64)
                }),
                attacker: aircraft.and_then(|a| a.get("attacker").and_then(Value::as_i64)),
                max_aircraft: aircraft.and_then(|a| a.get("maxAircraft").and_then(Value::as_i64)),
                restore_time: aircraft.and_then(|a| a.get("restoreTime").and_then(Value::as_f64)),
                bomb_name: aircraft.and_then(|a| a.get("bombName").and_then(Value::as_str).map(ToOwned::to_owned)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_bombers_and_fighters() {
        let json = json!({
            "PAAB001_Douglas_TBD": {
                "type": "Bomber", "nation": "USA",
                "name": "IDS_PAAB001_DOUGLAS_TBD",
                "health": 12.1, "totalPlanes": 1, "visibility": 10.0, "speed": 130.0
            },
            "PBAL001_M_THESEUS": {
                "type": "Skip", "nation": "United_Kingdom",
                "name": "IDS_PBAL001_M_THESEUS",
                "health": 2490.0, "totalPlanes": 6, "visibility": 10.0, "speed": 182.0,
                "aircraft": {
                    "restoreTime": 77.0, "maxAircraft": 12, "attacker": 3,
                    "attackCount": 1, "cooldown": 9, "bombName": "PBPS727_M_THESEUS"
                }
            }
        });
        let map = parse_aircrafts(&json);
        let bomber = &map["PAAB001_Douglas_TBD"];
        assert_eq!(bomber.r#type, "Bomber");
        assert!(bomber.attack_count.is_none());
        let skip = &map["PBAL001_M_THESEUS"];
        assert_eq!(skip.attack_count, Some(1));
        assert_eq!(skip.attacker, Some(3));
        assert_eq!(skip.max_aircraft, Some(12));
        assert_eq!(skip.restore_time, Some(77.0));
        assert_eq!(skip.bomb_name.as_deref(), Some("PBPS727_M_THESEUS"));
    }

    #[test]
    fn tolerant_of_missing_fields() {
        let map = parse_aircrafts(&json!({ "x": {"name": "IDS_X"} }));
        let entry = &map["x"];
        assert_eq!(entry.health, 0.0);
        assert!(entry.bomb_name.is_none());
    }

    #[test]
    fn parses_full_squadron_fields() {
        let json = json!({
            "PAAB052_Grumman_TBM3": {
                "type": "Bomber", "nation": "USA",
                "name": "IDS_PAAB052_GRUMMAN_TBM3",
                "health": 12.1, "totalPlanes": 6, "visibility": 10.0, "speed": 130.0,
                "attackCooldown": 5, "attackInterval": 7.5,
                "aimingTime": 15.0, "aimingSpeedLimits": [0.5, 1.5],
                "preparationTime": 1.0, "preparationSpeedLimits": [0.5, 1.5],
                "angleOfClimb": 35.0, "angleOfDive": -35.0,
                "jatoDuration": 0.0, "maxForsageAmount": 2,
                "maxNumberOnDeck": 0, "restorationTime": 0.1,
                "bombName": "PAPT910_Mk_13_mod0A_Independence",
                "planeConsumables": {
                    "AbilitySlot0": {
                        "abils": [["PCY034_ForsageBooster", "Allplanes"]],
                        "isSpecial": false, "slot": 0
                    },
                    "AbilitySlot1": {
                        "abils": [], "isSpecial": true, "slot": 1
                    }
                }
            }
        });
        let map = parse_aircrafts(&json);
        let bomber = &map["PAAB052_Grumman_TBM3"];
        assert_eq!(bomber.attack_cooldown, Some(5.0));
        assert_eq!(bomber.attack_interval, Some(7.5));
        assert_eq!(bomber.aiming_time, Some(15.0));
        assert_eq!(bomber.aiming_speed_limit_min, Some(0.5));
        assert_eq!(bomber.aiming_speed_limit_max, Some(1.5));
        assert_eq!(bomber.angle_of_climb, Some(35.0));
        assert_eq!(bomber.angle_of_dive, Some(-35.0));
        assert_eq!(bomber.jato_duration, Some(0.0));
        assert_eq!(bomber.max_forsage_amount, Some(2.0));
        assert_eq!(bomber.max_number_on_deck, Some(0));
        assert_eq!(bomber.restoration_time, Some(0.1));
        assert_eq!(bomber.plane_consumables.len(), 2);
        assert_eq!(bomber.plane_consumables[0].abilities, vec!["PCY034_ForsageBooster"]);
        assert!(bomber.plane_consumables[1].special);
    }
}
