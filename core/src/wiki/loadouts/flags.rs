//! Signal-flag views.

use std::collections::HashSet;

use super::summary::modifier_summary;
use super::views::FlagView;
use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::LangMap;

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
