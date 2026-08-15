//! `parse_game_data`: assemble the typed `GameData` from `wowsinfo.json`.

use serde_json::Map;
use serde_json::Value;

use super::helpers::{get, i64_field, i64_list, str_field, str_list, u64_list};
use super::types::{AbilityInfo, AchievementInfo, CommanderSkill, ConsumableInfo, FlagInfo, GameData, ModernizationInfo, ShipInfo, SkillInfo};
use crate::wiki::modifiers::parse_modifiers;

/// Parse a `wowsinfo.json` document into typed game data.
#[must_use]
pub fn parse_game_data(json: &Value) -> GameData {
    let empty = Value::Object(Map::new());

    let ships = get(json, "ships", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(key, value)| {
                    let id = value
                        .get("id")
                        .and_then(Value::as_u64)
                        .or_else(|| key.parse().ok())?;
                    let consumables = value
                        .get("consumables")
                        .and_then(Value::as_array)
                        .map(|slots| {
                            slots
                                .iter()
                                .map(|slot| {
                                    slot.as_array()
                                        .map(|consumables| {
                                            consumables
                                                .iter()
                                                .map(|consumable| ConsumableInfo {
                                                    name: str_field(consumable, "name"),
                                                    r#type: str_field(consumable, "type"),
                                                })
                                                .collect()
                                        })
                                        .unwrap_or_default()
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    Some((
                        id,
                        ShipInfo {
                            id,
                            name: str_field(value, "name"),
                            description: str_field(value, "description"),
                            year: str_field(value, "year"),
                            paper_ship: value
                                .get("paperShip")
                                .and_then(Value::as_bool)
                                .unwrap_or(false),
                            index: str_field(value, "index"),
                            tier: i64_field(value, "tier"),
                            region: str_field(value, "region"),
                            r#type: str_field(value, "type"),
                            region_id: str_field(value, "regionID"),
                            type_id: str_field(value, "typeID"),
                            group: str_field(value, "group"),
                            cost_xp: i64_field(value, "costXP"),
                            cost_gold: i64_field(value, "costGold"),
                            cost_cr: i64_field(value, "costCR"),
                            consumables,
                            permoflages: str_list(value, "permoflages"),
                            next_ships: value
                                .get("nextShips")
                                .and_then(Value::as_array)
                                .map(|list| {
                                    list.iter().filter_map(Value::as_u64).collect()
                                })
                                .unwrap_or_default(),
                            modules: value.get("modules").cloned().unwrap_or(Value::Null),
                            components: value
                                .get("components")
                                .cloned()
                                .unwrap_or(Value::Null),
                        },
                    ))
                })
                .collect()
        })
        .unwrap_or_default();

    let abilities = get(json, "abilities", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(key, value)| {
                    let id = value
                        .get("id")
                        .and_then(Value::as_u64)
                        .or_else(|| key.parse().ok())?;
                    Some((
                        id,
                        AbilityInfo {
                            id,
                            nation: str_field(value, "nation"),
                            name: str_field(value, "name"),
                            icon: str_field(value, "icon"),
                            description: str_field(value, "description"),
                            filter: str_field(value, "filter"),
                            r#type: str_field(value, "type"),
                            abilities: value.get("abilities").cloned().unwrap_or(Value::Null),
                            alter: value.get("alter").cloned().unwrap_or(Value::Null),
                        },
                    ))
                })
                .collect()
        })
        .unwrap_or_default();

    let achievements = get(json, "achievements", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(key, value)| {
                    let id = value
                        .get("id")
                        .and_then(Value::as_u64)
                        .or_else(|| key.parse().ok())?;
                    Some((
                        id,
                        AchievementInfo {
                            id,
                            icon: str_field(value, "icon"),
                            name: str_field(value, "name"),
                            description: str_field(value, "description"),
                            r#type: value
                                .get("type")
                                .and_then(Value::as_array)
                                .map(|list| {
                                    list.iter()
                                        .filter_map(Value::as_str)
                                        .map(str::to_string)
                                        .collect()
                                })
                                .unwrap_or_default(),
                            constants: value.get("constants").cloned().unwrap_or(Value::Null),
                        },
                    ))
                })
                .collect()
        })
        .unwrap_or_default();

    let command_skills = get(json, "commandSkills", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(class, tiers)| {
                    tiers.as_array().map(|tiers| {
                        (
                            class.clone(),
                            tiers
                                .iter()
                                .map(|tier| {
                                    tier.as_array()
                                        .map(|columns| {
                                            columns
                                                .iter()
                                                .map(|skill| CommanderSkill {
                                                    name: str_field(skill, "name"),
                                                    tier: i64_field(skill, "tier"),
                                                    column: i64_field(skill, "column"),
                                                    description: str_field(skill, "description"),
                                                    icon: str_field(skill, "icon"),
                                                })
                                                .collect()
                                        })
                                        .unwrap_or_default()
                                })
                                .collect(),
                        )
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let modernizations = get(json, "modernizations", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        ModernizationInfo {
                            key: key.clone(),
                            icon: str_field(value, "icon"),
                            name: str_field(value, "name"),
                            description: str_field(value, "description"),
                            slot: i64_field(value, "slot"),
                            cost_cr: i64_field(value, "costCR"),
                            levels: i64_list(value, "level"),
                            r#types: str_list(value, "type"),
                            nations: str_list(value, "nation"),
                            ships: u64_list(value, "ships"),
                            excludes: u64_list(value, "excludes"),
                            modifiers: parse_modifiers(get(value, "modifiers", &Value::Object(Map::new()))),
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    let flags = get(json, "exteriors", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .filter(|(_, value)| str_field(value, "type") == "Flags")
                .map(|(key, value)| FlagInfo {
                    key: key.clone(),
                    name: str_field(value, "name"),
                    description: str_field(value, "description"),
                    cost_cr: i64_field(value, "costCR"),
                    modifiers: parse_modifiers(get(value, "modifiers", &Value::Object(Map::new()))),
                })
                .collect()
        })
        .unwrap_or_default();

    let skills = get(json, "skills", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .map(|(key, value)| {
                    let trigger = value.get("LogicTrigger").unwrap_or(&Value::Null);
                    (
                        key.clone(),
                        SkillInfo {
                            key: key.clone(),
                            name: str_field(value, "name"),
                            description: str_field(value, "description"),
                            tiers: value
                                .get("tier")
                                .and_then(Value::as_object)
                                .map(|map| {
                                    map.iter()
                                        .filter_map(|(class, tier)| {
                                            tier.as_i64().map(|t| (class.clone(), t))
                                        })
                                        .collect()
                                })
                                .unwrap_or_default(),
                            modifiers: parse_modifiers(get(value, "modifiers", &Value::Object(Map::new()))),
                            trigger_type: str_field(trigger, "triggerType"),
                            trigger_modifiers: parse_modifiers(
                                get(trigger, "modifiers", &Value::Object(Map::new())),
                            ),
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    let exteriors = get(json, "exteriors", &empty)
        .as_object()
        .map(|map| {
            map.iter()
                .map(|(key, value)| (key.clone(), str_field(value, "name")))
                .collect()
        })
        .unwrap_or_default();

    GameData {
        ships,
        abilities,
        achievements,
        command_skills,
        projectiles: crate::wiki::projectile::parse_projectiles(get(json, "projectiles", &empty)),
        aircraft: crate::wiki::aircraft::parse_aircrafts(get(json, "aircrafts", &empty)),
        modernizations,
        flags,
        skills,
        exteriors,
    }
}
