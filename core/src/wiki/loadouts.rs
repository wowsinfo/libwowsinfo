//! Ship loadout views: consumables, commander skills, module upgrades, flags
//! and the combined modifier set applied to the ship's stats.

use std::collections::HashSet;

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::gamedata::{GameData, ModernizationInfo, ShipInfo};
use super::modifiers::{ModifierSet, ModifierValue};
use super::LangMap;

/// Selection state for skills, upgrades, flags and the simulated conditions.
#[derive(Debug, Clone, Default)]
pub struct LocalBuildConfig {
    pub skills: HashSet<String>,
    pub upgrades: HashSet<String>,
    pub flags: HashSet<String>,
    /// 0..1, drives low-HP skills like Adrenaline Rush.
    pub hp_fraction: f64,
    /// True while the ship is spotted (drives trigger skills).
    pub spotted: bool,
}

/// One consumable variant shown on the ship detail.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ConsumableView {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub reload_s: f64,
    pub work_s: f64,
    /// -1 means unlimited charges.
    pub charges: i64,
}

/// One commander skill of the ship's class.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SkillView {
    pub key: String,
    pub name: String,
    pub description: String,
    pub tier: i64,
    pub trigger_type: String,
    pub selected: bool,
    pub summary: String,
}

/// One applicable module upgrade (modernization).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct UpgradeView {
    pub key: String,
    pub name: String,
    pub description: String,
    pub slot: i64,
    pub cost_cr: i64,
    pub selected: bool,
    pub summary: String,
}

/// One signal flag.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct FlagView {
    pub key: String,
    pub name: String,
    pub description: String,
    pub cost_cr: i64,
    pub selected: bool,
    pub summary: String,
}

fn fmt_percent(value: f64) -> String {
    if value >= 1.0 {
        format!("+{:.0}%", (value - 1.0) * 100.0)
    } else {
        format!("-{:.0}%", (1.0 - value) * 100.0)
    }
}

/// Format a modifier set into a short summary for the UI (up to 3 entries).
#[must_use]
pub fn modifier_summary(lang: &LangMap, ship_class: &str, mods: &ModifierSet) -> String {
    mods.entries
        .iter()
        .filter_map(|(key, value)| {
            let resolved = match value {
                ModifierValue::Number(v) => *v,
                ModifierValue::PerShipType(map) => map.get(ship_class).copied().unwrap_or(1.0),
            };
            if (resolved - 1.0).abs() < f64::EPSILON {
                return None;
            }
            let label = lang
                .get_raw(&format!("IDS_PARAMS_MODIFIER_{}", key.to_uppercase()))
                .unwrap_or(key)
                .to_string();
            Some(format!("{} {}", label, fmt_percent(resolved)))
        })
        .take(3)
        .collect::<Vec<_>>()
        .join(" · ")
}

fn ability_params<'a>(data: &'a GameData, name: &str, ship_class: &str) -> Option<&'a serde_json::Value> {
    let ability = data.abilities.values().find(|ability| ability.icon == name)?;
    let classes = ability.abilities.as_object()?;
    classes
        .get(ship_class)
        .or_else(|| classes.values().next())
}

/// Resolve the ship's consumable slots (best-effort per variant).
#[must_use]
pub fn consumable_views(data: &GameData, lang: &LangMap, ship: &ShipInfo) -> Vec<ConsumableView> {
    let mut out = Vec::new();
    for slot in &ship.consumables {
        for consumable in slot {
            let Some(params) = ability_params(data, &consumable.name, &ship.r#type) else {
                continue;
            };
            let Some(ability) = data.abilities.values().find(|a| a.icon == consumable.name) else {
                continue;
            };
            out.push(ConsumableView {
                name: lang.get(&ability.name),
                description: lang.get(&ability.description),
                r#type: consumable.r#type.clone(),
                reload_s: params
                    .get("reloadTime")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0),
                work_s: params
                    .get("workTime")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0),
                charges: params
                    .get("numConsumables")
                    .and_then(serde_json::Value::as_i64)
                    .unwrap_or(-1),
            });
        }
    }
    out
}

/// The commander skills available to the ship's class, ordered by tier.
#[must_use]
pub fn skill_views(
    data: &GameData,
    lang: &LangMap,
    ship: &ShipInfo,
    selected: &HashSet<String>,
) -> Vec<SkillView> {
    let mut out: Vec<SkillView> = data
        .skills
        .iter()
        .filter_map(|(key, skill)| {
            let tier = *skill.tiers.get(&ship.r#type)?;
            Some(SkillView {
                key: key.clone(),
                name: lang.get(&skill.name),
                description: lang.get(&skill.description),
                tier,
                trigger_type: skill.trigger_type.clone(),
                selected: selected.contains(key),
                summary: modifier_summary(lang, &ship.r#type, &skill.modifiers),
            })
        })
        .collect();
    out.sort_by(|a, b| a.tier.cmp(&b.tier).then(a.name.cmp(&b.name)));
    out
}

fn upgrade_applies(ship: &ShipInfo, upgrade: &ModernizationInfo) -> bool {
    if !upgrade.ships.is_empty() {
        return upgrade.ships.contains(&ship.id);
    }
    if upgrade.excludes.contains(&ship.id) {
        return false;
    }
    if !upgrade.r#types.is_empty() && !upgrade.r#types.contains(&ship.r#type) {
        return false;
    }
    if !upgrade.nations.is_empty() && !upgrade.nations.contains(&ship.region) {
        return false;
    }
    upgrade.levels.is_empty() || upgrade.levels.contains(&ship.tier)
}

/// Module upgrades (modernizations) applicable to the ship.
#[must_use]
pub fn upgrade_views(
    data: &GameData,
    lang: &LangMap,
    ship: &ShipInfo,
    selected: &HashSet<String>,
) -> Vec<UpgradeView> {
    let mut out: Vec<UpgradeView> = data
        .modernizations
        .iter()
        .filter(|(_, upgrade)| upgrade_applies(ship, upgrade))
        .map(|(key, upgrade)| UpgradeView {
            key: key.clone(),
            name: lang.get(&upgrade.name),
            description: lang.get(&upgrade.description),
            slot: upgrade.slot,
            cost_cr: upgrade.cost_cr,
            selected: selected.contains(key),
            summary: modifier_summary(lang, &ship.r#type, &upgrade.modifiers),
        })
        .collect();
    out.sort_by(|a, b| a.slot.cmp(&b.slot).then(a.name.cmp(&b.name)));
    out
}

/// Signal flags for the ship.
#[must_use]
pub fn flag_views(
    data: &GameData,
    lang: &LangMap,
    ship: &ShipInfo,
    selected: &HashSet<String>,
) -> Vec<FlagView> {
    data.flags
        .iter()
        .map(|flag| FlagView {
            key: flag.key.clone(),
            name: lang.get(&flag.name),
            description: lang.get(&flag.description),
            cost_cr: flag.cost_cr,
            selected: selected.contains(&flag.key),
            summary: modifier_summary(lang, &ship.r#type, &flag.modifiers),
        })
        .collect()
}

/// Combine the selected skills, upgrades, flags and active trigger modifiers.
#[must_use]
pub fn combined_modifiers(
    data: &GameData,
    ship: &ShipInfo,
    config: &LocalBuildConfig,
) -> ModifierSet {
    let mut combined = ModifierSet::default();
    for key in &config.skills {
        let Some(skill) = data.skills.get(key) else {
            continue;
        };
        combined = combined.merged(&skill.modifiers);
        let trigger_active = match skill.trigger_type.as_str() {
            "entityIsVisibleTrigger" => config.spotted,
            "entityIsInvisibleTrigger" => !config.spotted,
            _ => false,
        };
        if trigger_active {
            combined = combined.merged(&skill.trigger_modifiers);
        }
    }
    for key in &config.upgrades {
        if let Some(upgrade) = data.modernizations.get(key) {
            combined = combined.merged(&upgrade.modifiers);
        }
    }
    for flag in &data.flags {
        if config.flags.contains(&flag.key) {
            combined = combined.merged(&flag.modifiers);
        }
    }
    combined
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn test_data() -> GameData {
        let json = json!({
            "ships": {
                "1": {
                    "id": 1, "index": "T1", "name": "IDS_N", "description": "",
                    "year": "", "paperShip": false, "tier": 8, "region": "USA",
                    "type": "Cruiser", "regionID": "IDS_USA", "typeID": "IDS_CRUISER",
                    "group": "normal", "costXP": 0, "costGold": 0, "costCR": 0,
                    "consumables": [[{"name": "PCY006_SmokeGenerator", "type": "C_TierOne"}]],
                    "nextShips": [],
                    "modules": {}, "components": {}
                }
            },
            "abilities": {
                "PCY006_SmokeGenerator": {
                    "name": "IDS_SMOKE", "description": "IDS_SMOKE_DESC",
                    "abilities": {
                        "Cruiser": {"reloadTime": 120.0, "workTime": 30.0, "numConsumables": 3}
                    }
                }
            },
            "skills": {
                "TriggerGmReload": {
                    "name": "IDS_SKILL", "description": "IDS_SKILL_DESC",
                    "tier": {"Cruiser": 3},
                    "modifiers": {},
                    "LogicTrigger": {
                        "triggerType": "entityIsVisibleTrigger",
                        "modifiers": {"GMShotDelay": 0.9}
                    }
                }
            },
            "modernizations": {
                "PCM001": {
                    "name": "IDS_U", "description": "IDS_U_DESC", "slot": 1,
                    "costCR": 500, "level": [8], "type": ["Cruiser"], "nation": ["USA"],
                    "modifiers": {"GMShotDelay": 0.88}
                }
            },
            "exteriors": {
                "PCEF005_SM_SignalFlag": {
                    "type": "Flags", "name": "IDS_F", "description": "IDS_F_DESC",
                    "costCR": 1000, "modifiers": {"speedCoef": 1.05}
                }
            }
        });
        super::super::gamedata::parse_game_data(&json)
    }

    #[test]
    fn builds_loadout_views() {
        let data = test_data();
        let ship = &data.ships[&1];
        let lang = LangMap::from_entries([
            ("IDS_SMOKE".to_string(), "Smoke Generator".to_string()),
            ("IDS_SMOKE_DESC".to_string(), "Smoke".to_string()),
            ("IDS_SKILL".to_string(), "Rapid Reload".to_string()),
            ("IDS_U".to_string(), "Reload Mod 1".to_string()),
            ("IDS_F".to_string(), "SM Flag".to_string()),
        ]);
        let consumables = consumable_views(&data, &lang, ship);
        assert_eq!(consumables.len(), 1);
        assert_eq!(consumables[0].name, "Smoke Generator");
        assert_eq!(consumables[0].reload_s, 120.0);
        assert_eq!(consumables[0].charges, 3);

        let skills = skill_views(&data, &lang, ship, &HashSet::new());
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].tier, 3);
        assert_eq!(skills[0].trigger_type, "entityIsVisibleTrigger");

        let upgrades = upgrade_views(&data, &lang, ship, &HashSet::new());
        assert_eq!(upgrades.len(), 1);
        assert_eq!(upgrades[0].slot, 1);

        let flags = flag_views(&data, &lang, ship, &HashSet::new());
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0].summary, "Speed +5%");

        let config = LocalBuildConfig {
            skills: HashSet::from(["TriggerGmReload".to_string()]),
            upgrades: HashSet::from(["PCM001".to_string()]),
            flags: HashSet::from(["PCEF005_SM_SignalFlag".to_string()]),
            hp_fraction: 1.0,
            spotted: true,
        };
        let mods = combined_modifiers(&data, ship, &config);
        assert!((mods.multiply("Cruiser", "GMShotDelay") - 0.88 * 0.9).abs() < 1e-9);
        assert!((mods.multiply("Cruiser", "speedCoef") - 1.05).abs() < 1e-9);

        let unspotted = LocalBuildConfig {
            spotted: false,
            ..config
        };
        let mods = combined_modifiers(&data, ship, &unspotted);
        assert!((mods.multiply("Cruiser", "GMShotDelay") - 0.88).abs() < 1e-9);
    }
}
