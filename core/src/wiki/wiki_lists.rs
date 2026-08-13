//! Wiki-list views: every consumable and commander skill in the game data.

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::gamedata::GameData;
use super::loadouts::{skill_summary, ConsumableView};
use super::LangMap;

/// One commander skill entry for the wiki list.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct LocalSkillWikiEntry {
    pub key: String,
    pub name: String,
    pub description: String,
    /// Per-class tiers rendered as "Cruiser 4 · Battleship 2".
    pub tier_display: String,
    pub summary: String,
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
                charges: params
                    .get("numConsumables")
                    .and_then(serde_json::Value::as_i64)
                    .unwrap_or(-1),
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
                summary: skill_summary(lang, &ship_class, skill),
            }
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}
