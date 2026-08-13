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

/// Parse the `aircrafts` section into a map keyed by aircraft name.
#[must_use]
pub fn parse_aircrafts(json: &Value) -> HashMap<String, AircraftInfo> {
    let mut out = HashMap::new();
    let Some(map) = json.as_object() else {
        return out;
    };
    for (key, value) in map {
        let aircraft = value.get("aircraft").filter(|v| !v.is_null());
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
}
