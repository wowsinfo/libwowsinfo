//! Parser for the `wowsinfo.json` game-data file bundled by the Flutter app
//! (from the WoWs-Game-Data repository), covering the wiki-relevant datasets:
//! ships, abilities, achievements and commander skills.

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::modifiers::{parse_modifiers, ModifierSet};

/// One consumable slot (`consumables[][]`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumableInfo {
    pub name: String,
    pub r#type: String,
}

/// One ship entry (`ships.<id>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShipInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub year: String,
    pub paper_ship: bool,
    pub index: String,
    pub tier: i64,
    pub region: String,
    pub r#type: String,
    pub region_id: String,
    pub type_id: String,
    pub group: String,
    pub cost_xp: i64,
    pub cost_gold: i64,
    pub cost_cr: i64,
    pub consumables: Vec<Vec<ConsumableInfo>>,
    pub next_ships: Vec<u64>,
    pub permoflages: Vec<String>,
    /// Raw module/component trees; the sub-shapes are game-data specific and
    /// kept opaque for forward compatibility.
    pub modules: Value,
    pub components: Value,
}

/// One commander skill (`commandSkills.<class>[tier][column]`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommanderSkill {
    pub name: String,
    pub tier: i64,
    pub column: i64,
    pub description: String,
    pub icon: String,
}

/// One ability/consumable entry (`abilities.<id>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbilityInfo {
    pub id: u64,
    pub nation: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub filter: String,
    pub r#type: String,
    /// Raw modifier map (`abilities`), kept opaque.
    pub abilities: Value,
    /// Raw upgrade alter map (`alter`).
    pub alter: Value,
}

/// One achievement entry (`achievements.<id>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementInfo {
    pub id: u64,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub r#type: Vec<String>,
    /// Raw constants map.
    pub constants: Value,
}

/// One modernization (module upgrade) entry (`modernizations.<key>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModernizationInfo {
    pub key: String,
    pub name: String,
    pub description: String,
    pub slot: i64,
    pub cost_cr: i64,
    pub levels: Vec<i64>,
    pub r#types: Vec<String>,
    pub nations: Vec<String>,
    pub ships: Vec<u64>,
    pub excludes: Vec<u64>,
    pub modifiers: ModifierSet,
}

/// One flag / signal exterior (`exteriors.<key>` with type `Flags`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagInfo {
    pub key: String,
    pub name: String,
    pub description: String,
    pub cost_cr: i64,
    pub modifiers: ModifierSet,
}

/// One commander skill (`skills.<key>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillInfo {
    pub key: String,
    pub name: String,
    pub description: String,
    /// Skill tier per ship class (`tier: { "Cruiser": 3 }`).
    pub tiers: HashMap<String, i64>,
    pub modifiers: ModifierSet,
    /// Trigger condition (`LogicTrigger.triggerType`), empty when passive.
    pub trigger_type: String,
    /// Modifiers applied while the trigger condition is active.
    pub trigger_modifiers: ModifierSet,
}

/// The parsed `wowsinfo.json` datasets.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameData {
    pub ships: HashMap<u64, ShipInfo>,
    pub abilities: HashMap<u64, AbilityInfo>,
    pub achievements: HashMap<u64, AchievementInfo>,
    /// Ship class -> tiers -> columns of commander skills.
    pub command_skills: HashMap<String, Vec<Vec<CommanderSkill>>>,
    /// Shells, bombs and torpedoes keyed by projectile name.
    pub projectiles: HashMap<String, super::projectile::ProjectileInfo>,
    /// Carrier aircraft keyed by aircraft name.
    pub aircraft: HashMap<String, super::aircraft::AircraftInfo>,
    /// Module upgrades keyed by modernization name.
    pub modernizations: HashMap<String, ModernizationInfo>,
    /// Signal flags (exteriors with type `Flags`).
    pub flags: Vec<FlagInfo>,
    /// Commander skills keyed by skill name.
    pub skills: HashMap<String, SkillInfo>,
    /// Exteriors (camos/skins/flags) keyed by key, mapped to their name IDS.
    pub exteriors: HashMap<String, String>,
}

fn get<'a>(json: &'a Value, key: &str, default: &'a Value) -> &'a Value {
    json.get(key).filter(|v| !v.is_null()).unwrap_or(default)
}

fn str_field(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn i64_field(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}

fn u64_list(json: &Value, key: &str) -> Vec<u64> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_u64).collect())
        .unwrap_or_default()
}

fn str_list(json: &Value, key: &str) -> Vec<String> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_str).map(ToOwned::to_owned).collect())
        .unwrap_or_default()
}

fn i64_list(json: &Value, key: &str) -> Vec<i64> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_i64).collect())
        .unwrap_or_default()
}

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
        projectiles: super::projectile::parse_projectiles(get(json, "projectiles", &empty)),
        aircraft: super::aircraft::parse_aircrafts(get(json, "aircrafts", &empty)),
        modernizations,
        flags,
        skills,
        exteriors,
    }
}
