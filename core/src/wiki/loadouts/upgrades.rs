//! Module-upgrade views.

use std::collections::HashSet;

use super::summary::modifier_summary;
use super::views::UpgradeView;
use crate::wiki::gamedata::{GameData, ModernizationInfo, ShipInfo};
use crate::wiki::LangMap;

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
