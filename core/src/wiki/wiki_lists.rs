//! Wiki-list views: every consumable and commander skill in the game data.

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::gamedata::GameData;
use super::loadouts::{modifier_summary_any, skill_summary, ConsumableView};
use super::LangMap;

/// One commander-skill tier entry (per ship class).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SkillTierEntry {
    pub ship_class: String,
    pub tier: i64,
}

/// One commander skill entry for the wiki list.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct LocalSkillWikiEntry {
    pub key: String,
    pub name: String,
    pub description: String,
    /// Per-class tiers rendered as "Cruiser 4 · Battleship 2".
    pub tier_display: String,
    /// Structured per-class tiers for the skill-point builder.
    pub tiers: Vec<SkillTierEntry>,
    pub summary: String,
}

/// One achievement entry for the wiki grid (localised, constants resolved).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct LocalAchievementEntry {
    pub key: String,
    pub icon: String,
    pub name: String,
    pub description: String,
}

/// One modernization (module upgrade) entry for the wiki grid.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct LocalUpgradeEntry {
    pub key: String,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub slot: i64,
    pub cost_cr: i64,
    pub summary: String,
}

/// One signal-flag entry for the wiki list.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct LocalFlagEntry {
    pub key: String,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub cost_cr: i64,
    pub summary: String,
}

/// Substitute `%(name)s`-style placeholders in a localised string with the
/// values from the achievement's `constants` map (e.g. `%(timeInterval)s`).
fn format_constants(text: &str, constants: &Value) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find("%(") {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + 2..];
        let Some(close) = after.find(')') else {
            out.push_str(rest);
            return out;
        };
        let key = &after[..close];
        let tail = &after[close + 1..];
        let conv = tail.chars().next().unwrap_or('s');
        let conv_len = conv.len_utf8();
        match constants.get(key).and_then(Value::as_f64) {
            Some(value) => {
                out.push_str(&format_placeholder(value, conv));
                rest = &tail[conv_len..];
            }
            None => {
                // Unknown constant: keep the placeholder text intact.
                out.push_str("%(");
                out.push_str(key);
                out.push(')');
                rest = tail;
            }
        }
    }
    out.push_str(rest);
    out
}

fn format_placeholder(value: f64, conv: char) -> String {
    match conv {
        'd' | 'i' => format!("{value:.0}"),
        'f' => {
            let s = format!("{value:.2}");
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        }
        _ => {
            let s = format!("{value}");
            s.strip_suffix(".0").unwrap_or(&s).to_string()
        }
    }
}

/// Every achievement in the game data (wiki grid), sorted by name.
#[must_use]
pub fn all_achievement_views(data: &GameData, lang: &LangMap) -> Vec<LocalAchievementEntry> {
    let mut out: Vec<LocalAchievementEntry> = data
        .achievements
        .values()
        .map(|achievement| LocalAchievementEntry {
            key: achievement.id.to_string(),
            icon: achievement.icon.clone(),
            name: lang.get(&achievement.name),
            description: format_constants(&lang.get(&achievement.description), &achievement.constants),
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Every modernization in the game data (wiki grid), sorted by slot then name.
#[must_use]
pub fn all_upgrade_views(data: &GameData, lang: &LangMap) -> Vec<LocalUpgradeEntry> {
    let mut out: Vec<LocalUpgradeEntry> = data
        .modernizations
        .iter()
        .map(|(key, upgrade)| LocalUpgradeEntry {
            key: key.clone(),
            icon: upgrade.icon.clone(),
            name: lang.get(&upgrade.name),
            description: lang.get(&upgrade.description),
            slot: upgrade.slot,
            cost_cr: upgrade.cost_cr,
            summary: modifier_summary_any(lang, &upgrade.modifiers),
        })
        .collect();
    out.sort_by(|a, b| a.slot.cmp(&b.slot).then(a.name.cmp(&b.name)));
    out
}

/// Every signal flag in the game data (wiki list).
#[must_use]
pub fn all_flag_views(data: &GameData, lang: &LangMap) -> Vec<LocalFlagEntry> {
    let mut out: Vec<LocalFlagEntry> = data
        .flags
        .iter()
        .map(|flag| LocalFlagEntry {
            key: flag.key.clone(),
            icon: flag.key.clone(),
            name: lang.get(&flag.name),
            description: lang.get(&flag.description),
            cost_cr: flag.cost_cr,
            summary: modifier_summary_any(lang, &flag.modifiers),
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Every consumable in the game data (wiki list), using the first class
/// variant's parameters.
#[must_use]
pub fn all_consumable_views(data: &GameData, lang: &LangMap) -> Vec<ConsumableView> {
    let mut out: Vec<ConsumableView> = data
        .abilities
        .values()
        .filter_map(|ability| {
            let params = ability
                .abilities
                .as_object()
                .and_then(|classes| classes.values().next())
                .and_then(serde_json::Value::as_object)?;
            Some(ConsumableView {
                key: ability.icon.clone(),
                name: lang.get(&ability.name),
                description: lang.get(&ability.description),
                r#type: lang.get(&ability.r#type),
                reload_s: params
                    .get("reloadTime")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0),
                work_s: params
                    .get("workTime")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0),
                preparation_s: params
                    .get("workPreparationTime")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0),
                charges: params
                    .get("numConsumables")
                    .and_then(serde_json::Value::as_i64)
                    .unwrap_or(-1),
                alters: super::loadouts::alter_views(&ability.alter, lang),
            })
        })
        .collect();
    out.sort_by(|a, b| a.r#type.cmp(&b.r#type).then(a.name.cmp(&b.name)));
    out
}

/// Every commander skill in the game data (wiki list).
#[must_use]
pub fn all_skill_views(data: &GameData, lang: &LangMap) -> Vec<LocalSkillWikiEntry> {
    let mut out: Vec<LocalSkillWikiEntry> = data
        .skills
        .values()
        .map(|skill| {
            let ship_class = skill.tiers.keys().next().cloned().unwrap_or_default();
            let mut tiers: Vec<(&String, &i64)> = skill.tiers.iter().collect();
            tiers.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
            let tier_display = tiers
                .iter()
                .map(|(class, tier)| format!("{class} {tier}"))
                .collect::<Vec<_>>()
                .join(" · ");
            LocalSkillWikiEntry {
                key: skill.key.clone(),
                name: lang.get(&skill.name),
                description: lang.get(&skill.description),
                tier_display,
                tiers: tiers
                    .iter()
                    .map(|(ship_class, tier)| SkillTierEntry {
                        ship_class: (*ship_class).clone(),
                        tier: **tier,
                    })
                    .collect(),
                summary: skill_summary(lang, &ship_class, skill),
            }
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}
