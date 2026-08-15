//! Commander-skill views.

use std::collections::HashSet;

use super::summary::modifier_summary;
use super::views::SkillView;
use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::LangMap;

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
                summary: skill_summary(lang, &ship.r#type, skill),
            })
        })
        .collect();
    out.sort_by(|a, b| a.tier.cmp(&b.tier).then(a.name.cmp(&b.name)));
    out
}

pub(crate) fn skill_summary(lang: &LangMap, ship_class: &str, skill: &crate::wiki::gamedata::SkillInfo) -> String {
    let base = modifier_summary(lang, ship_class, &skill.modifiers);
    let trigger = modifier_summary(lang, ship_class, &skill.trigger_modifiers);
    let condition = match skill.trigger_type.as_str() {
        "entityIsVisibleTrigger" => Some("While spotted"),
        "entityIsInvisibleTrigger" => Some("While unspotted"),
        _ => None,
    };
    match (base.is_empty(), condition, trigger.is_empty()) {
        (true, Some(label), false) => format!("{label}: {trigger}"),
        (false, Some(label), false) => format!("{base} 路 {label}: {trigger}"),
        (_, _, _) => base,
    }
}

