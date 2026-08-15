//! Consumable views.

use super::views::{ConsumableAlterView, ConsumableView};
use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::LangMap;

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
                key: consumable.name.clone(),
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
                preparation_s: params
                    .get("workPreparationTime")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0),
                charges: params
                    .get("numConsumables")
                    .and_then(serde_json::Value::as_i64)
                    .unwrap_or(-1),
                alters: alter_views(&ability.alter, lang),
            });
        }
    }
    out
}

/// Resolve an ability's alter map into sorted, localised variant views.
pub(crate) fn alter_views(
    alter: &serde_json::Value,
    lang: &LangMap,
) -> Vec<ConsumableAlterView> {
    let mut alters: Vec<ConsumableAlterView> = alter
        .as_object()
        .map(|map| {
            map.iter()
                .filter_map(|(key, value)| {
                    let name = value.get("name").and_then(serde_json::Value::as_str)?;
                    let description =
                        value.get("description").and_then(serde_json::Value::as_str)?;
                    Some(ConsumableAlterView {
                        key: key.clone(),
                        name: lang.get(name),
                        description: lang.get(description),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    alters.sort_by(|a, b| a.name.cmp(&b.name));
    alters
}
