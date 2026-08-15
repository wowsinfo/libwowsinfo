//! Combined modifier set.

use super::views::LocalBuildConfig;
use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::modifiers::ModifierSet;

/// Combine the selected skills, upgrades, flags and active trigger modifiers.
#[must_use]
pub fn combined_modifiers(
    data: &GameData,
    _ship: &ShipInfo,
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
