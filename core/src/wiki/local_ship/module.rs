//! Module-tree view models.

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::gamedata::ShipInfo;
use crate::wiki::ship_builder::{ModuleOption, ModuleSelection, ShipBuild};
use crate::wiki::LangMap;

/// One module option shown in the module dialog.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ModuleOptionView {
    pub index: i64,
    pub name: String,
    pub cost_xp: i64,
    pub cost_cr: i64,
    /// What selecting this option changes vs the current build.
    pub delta: String,
}

/// One changeable module slot shown in the UI.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ModuleSlotView {
    pub slot: String,
    pub label: String,
    pub selected: i64,
    pub options: Vec<ModuleOptionView>,
}

pub(super) fn module_slot_views(
    lang: &LangMap,
    ship: &ShipInfo,
    slots: Vec<(String, Vec<ModuleOption>)>,
    selection: ModuleSelection,
    build: &ShipBuild,
) -> Vec<ModuleSlotView> {
    let selected = |slot: &str| match slot {
        "hull" => selection.hull,
        "artillery" => selection.artillery,
        "torpedoes" => selection.torpedoes,
        "fire_control" | "flight_control" => selection.fire_control,
        "engine" => selection.engine,
        "fighter" => selection.fighter,
        "torpedo_bomber" => selection.torpedo_bomber,
        "dive_bomber" => selection.dive_bomber,
        "skip_bomber" => selection.skip_bomber,
        _ => 0,
    };
    slots
        .into_iter()
        .map(|(slot, options)| ModuleSlotView {
            selected: selected(&slot) as i64,
            label: slot_label(&slot),
            options: options
                .into_iter()
                .map(|option| ModuleOptionView {
                    index: option.index,
                    name: lang.get(&option.name),
                    cost_xp: option.cost_xp,
                    cost_cr: option.cost_cr,
                    delta: crate::wiki::ship_builder::module_option_delta(
                        ship,
                        selection,
                        &slot,
                        option.index as usize,
                        build,
                    ),
                })
                .collect(),
            slot,
        })
        .collect()
}

fn slot_label(slot: &str) -> String {
    match slot {
        "hull" => "Hull",
        "artillery" => "Main Battery",
        "torpedoes" => "Torpedoes",
        "fire_control" | "flight_control" => "Fire Control",
        "engine" => "Engine",
        "fighter" => "Fighter",
        "torpedo_bomber" => "Torpedo Bombers",
        "dive_bomber" => "Dive Bombers",
        "skip_bomber" => "Skip Bombers",
        _ => slot,
    }
    .to_string()
}

