//! Parser for the `wows-constants` `latest.json` game-constants file.

use std::collections::HashMap;

use serde_json::{Map, Value};

/// Game version triplet.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameVersion {
    pub version: String,
    pub build: i64,
    pub patch: f64,
}

/// One battle type (`BATTLE_TYPES.<id>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleType {
    pub players_per_team: u64,
    pub name: String,
    pub scenario: String,
    pub teams_count: u64,
}

/// One death reason (`DEATH_REASONS.<id>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeathReason {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub sound: String,
}

/// The constants used to interpret arena/replay data and enrich the wiki.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameConstants {
    pub version: Option<GameVersion>,
    /// Ship class name -> id (`SHIP_TYPES`).
    pub ship_types: HashMap<String, u64>,
    /// Battle type id -> info (`BATTLE_TYPES`).
    pub battle_types: HashMap<u64, BattleType>,
    /// Consumable name -> id (`CONSUMABLE_IDS`).
    pub consumable_ids: HashMap<String, u64>,
    /// Ship class -> tier -> skill ids (`SKILLS_BY_SHIP_TYPE`).
    pub skills_by_ship_type: HashMap<String, Vec<HashMap<String, Vec<u64>>>>,
    /// Ribbon id -> name (`RIBBONS`).
    pub ribbons: HashMap<u64, String>,
    /// Death reason id -> info (`DEATH_REASONS`).
    pub death_reasons: HashMap<u64, DeathReason>,
}

fn get<'a>(json: &'a Value, key: &str, default: &'a Value) -> &'a Value {
    json.get(key).filter(|v| !v.is_null()).unwrap_or(default)
}

fn as_u64_map(json: &Value) -> HashMap<u64, String> {
    json.as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(id, value)| {
                    id.parse::<u64>()
                        .ok()
                        .map(|id| (id, value.as_str().unwrap_or("").to_string()))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Parse `latest.json` (wows-constants format) into typed constants.
#[must_use]
pub fn parse_constants(json: &Value) -> GameConstants {
    let empty = Value::Object(Map::new());
    let version = json.get("VERSION").and_then(Value::as_object).map(|v| GameVersion {
        version: v
            .get("VERSION")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        build: v.get("BUILD").and_then(Value::as_i64).unwrap_or(0),
        patch: v.get("PATCH").and_then(Value::as_f64).unwrap_or(0.0),
    });

    let ship_types = get(json, "SHIP_TYPES", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(name, value)| {
                    value.as_u64().map(|id| (name.clone(), id))
                })
                .collect()
        })
        .unwrap_or_default();

    let battle_types = get(json, "BATTLE_TYPES", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(id, value)| {
                    let id = id.parse::<u64>().ok()?;
                    let info = value.as_object()?;
                    Some((
                        id,
                        BattleType {
                            players_per_team: info
                                .get("playersPerTeam")
                                .and_then(Value::as_u64)
                                .unwrap_or(0),
                            name: info
                                .get("name")
                                .and_then(Value::as_str)
                                .unwrap_or("")
                                .to_string(),
                            scenario: info
                                .get("scenario")
                                .and_then(Value::as_str)
                                .unwrap_or("")
                                .to_string(),
                            teams_count: info
                                .get("teamsCount")
                                .and_then(Value::as_u64)
                                .unwrap_or(0),
                        },
                    ))
                })
                .collect()
        })
        .unwrap_or_default();

    let consumable_ids = get(json, "CONSUMABLE_IDS", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(name, value)| value.as_u64().map(|id| (name.clone(), id)))
                .collect()
        })
        .unwrap_or_default();

    let skills_by_ship_type = get(json, "SKILLS_BY_SHIP_TYPE", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(class, tiers)| {
                    tiers.as_array().map(|tiers| {
                        (
                            class.clone(),
                            tiers
                                .iter()
                                .filter_map(|tier| {
                                    tier.as_object().map(|tier| {
                                        tier
                                            .iter()
                                            .map(|(key, ids)| {
                                                (
                                                    key.clone(),
                                                    ids
                                                        .as_array()
                                                        .map(|ids| {
                                                            ids.iter()
                                                                .filter_map(Value::as_u64)
                                                                .collect()
                                                        })
                                                        .unwrap_or_default(),
                                                )
                                            })
                                            .collect()
                                    })
                                })
                                .collect(),
                        )
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    GameConstants {
        version,
        ship_types,
        battle_types,
        consumable_ids,
        skills_by_ship_type,
        ribbons: as_u64_map(get(json, "RIBBONS", &empty)),
        death_reasons: get(json, "DEATH_REASONS", &empty)
            .as_object()
            .map(|map| {
                map.iter()
                    .filter_map(|(id, value)| {
                        let id = id.parse::<u64>().ok()?;
                        let info = value.as_object()?;
                        Some((
                            id,
                            DeathReason {
                                id: info.get("id").and_then(Value::as_u64).unwrap_or(id),
                                name: info
                                    .get("name")
                                    .and_then(Value::as_str)
                                    .unwrap_or("")
                                    .to_string(),
                                icon: info
                                    .get("icon")
                                    .and_then(Value::as_str)
                                    .unwrap_or("")
                                    .to_string(),
                                sound: info
                                    .get("sound")
                                    .and_then(Value::as_str)
                                    .unwrap_or("")
                                    .to_string(),
                            },
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}
