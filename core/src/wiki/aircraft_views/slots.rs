//! Aircraft squadron slot views.

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::detail::AircraftDetail;
use super::helpers::resolve_aircraft;
use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::modifiers::ModifierSet;
use crate::wiki::ship_builder::ModuleSelection;
use crate::wiki::LangMap;

/// One aircraft squadron slot (fighter / torpedo / dive / skip bombers).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AircraftSlotView {
    pub slot: String,
    pub label: String,
    pub selected: i64,
    pub options: Vec<AircraftOptionView>,
}

/// One squadron option with its resolved aircraft.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AircraftOptionView {
    pub index: i64,
    pub name: String,
    pub aircraft: Option<AircraftDetail>,
}

/// Build the squadron views for a carrier (or empty for other classes).
#[must_use]
pub fn aircraft_slot_views(
    data: &GameData,
    lang: &LangMap,
    ship: &ShipInfo,
    selection: ModuleSelection,
    mods: &ModifierSet,
    ship_class: &str,
) -> Vec<AircraftSlotView> {
    const SLOTS: [(&str, &str, &str); 4] = [
        ("_Fighter", "fighter", "Fighter"),
        ("_TorpedoBomber", "torpedoBomber", "Torpedo Bombers"),
        ("_DiveBomber", "diveBomber", "Dive Bombers"),
        ("_SkipBomber", "skipBomber", "Skip Bombers"),
    ];
    let selected = |slot: &str| match slot {
        "fighter" => selection.fighter,
        "torpedoBomber" => selection.torpedo_bomber,
        "diveBomber" => selection.dive_bomber,
        "skipBomber" => selection.skip_bomber,
        _ => 0,
    };
    SLOTS
        .iter()
        .filter_map(|(module_key, component_key, label)| {
            let options = ship.modules.get(*module_key)?.as_array()?;
            if options.is_empty() {
                return None;
            }
            let option_views = options
                .iter()
                .enumerate()
                .filter_map(|(index, option)| {
                    let name = option
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    let aircraft = option
                        .get("components")
                        .and_then(|c| c.get(*component_key))
                        .and_then(|ids| ids.as_array())
                        .and_then(|ids| ids.first())
                        .and_then(Value::as_str)
                        .and_then(|key| resolve_aircraft(data, ship, key))
                        .map(|aircraft| {
                            AircraftDetail::from_aircraft(lang, data, aircraft, mods, ship_class)
                        });
                    Some(AircraftOptionView {
                        index: index as i64,
                        name: lang.get(&name),
                        aircraft,
                    })
                })
                .collect();
            Some(AircraftSlotView {
                slot: (*component_key).to_string(),
                label: (*label).to_string(),
                selected: selected(*component_key) as i64,
                options: option_views,
            })
        })
        .collect()
}
