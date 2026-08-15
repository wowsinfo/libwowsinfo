//! Ship wiki entry + builder.

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::aircraft_views::{aircraft_slot_views, air_support_plane, AircraftDetail, AircraftSlotView};
use super::airstrike::{airstrike_view, AirstrikeView};
use super::armor::{armor_view, ArmorView};
use super::battery::{main_battery_view, MainBatteryView};
use crate::wiki::compare::{similar_ships, SimilarShip};
use super::depth_charge::{depth_charge_view, DepthChargeView};
use super::module::{module_slot_views, ModuleSlotView};
use super::pen::{pen_curve, PenCurveView};
use super::special::{special_ability_view, SpecialAbilityView};
use super::torpedo::{torpedo_view, TorpedoView};
use crate::wiki::components::{AirDefenseStats, EngineStats, FireControlStats, HullStats, PingerStats};
use crate::wiki::gamedata::GameData;
use crate::wiki::loadouts::{combined_modifiers, consumable_views, flag_views, next_ship_views, skill_views, upgrade_views, ConsumableView, FlagView, LocalBuildConfig, NextShip, SkillView, UpgradeView};
use crate::wiki::modifiers::apply_modifiers;
use crate::wiki::ship_builder::{build_ship_build, ModuleSelection};
use crate::wiki::LangMap;

/// The full local ship wiki entry.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct LocalShipWiki {
    pub ship_id: u64,
    pub index: String,
    pub name: String,
    pub description: String,
    pub year: String,
    pub tier: i64,
    pub nation: String,
    pub region: String,
    pub ship_type: String,
    pub group: String,
    pub premium: bool,
    pub special: bool,
    pub cost_credit: i64,
    pub cost_gold: i64,
    pub cost_xp: i64,
    pub next_ships: Vec<NextShip>,
    pub camo_count: i64,
    /// Localised names of the ship's permanent camouflages.
    pub camos: Vec<String>,
    /// Exterior keys of the ship's permanent camouflages (icon lookup).
    pub camo_keys: Vec<String>,
    pub modules: Vec<ModuleSlotView>,
    pub hull: Option<HullStats>,
    pub main_battery: Option<MainBatteryView>,
    pub secondaries: Option<MainBatteryView>,
    pub torpedoes: Option<TorpedoView>,
    pub air_defense: Option<AirDefenseStats>,
    pub fire_control: Option<FireControlStats>,
    pub engine: Option<EngineStats>,
    pub depth_charges: Option<DepthChargeView>,
    pub air_support: Option<AirstrikeView>,
    pub pinger: Option<PingerStats>,
    pub special_ability: Option<SpecialAbilityView>,
    /// Carrier squadrons (fighters, torpedo/dive/skip bombers).
    pub aircraft: Vec<AircraftSlotView>,
    /// Plane used by the air-support consumable (when present).
    pub air_support_plane: Option<AircraftDetail>,
    pub consumables: Vec<ConsumableView>,
    pub skills: Vec<SkillView>,
    pub upgrades: Vec<UpgradeView>,
    pub flags: Vec<FlagView>,
    /// Stats after skills/upgrades/flags/conditions are applied.
    pub adjusted: crate::wiki::modifiers::AdjustedStats,
    pub hp_fraction: f64,
    pub spotted: bool,
    pub penetration_curves: Vec<PenCurveView>,
    pub similar_ships: Vec<SimilarShip>,
    /// Armor digest (hull zones + turret/barbette armor).
    pub armor: Option<ArmorView>,
}

const SPECIAL_GROUPS: &[&str] = &[
    "ultimate",
    "specialUnsellable",
    "upgradeableUltimate",
    "upgradeableExclusive",
    "unavailable",
    "disabled",
    "preserved",
    "clan",
    "earlyAccess",
    "demoWithoutStats",
    "demoWithStats",
];


/// Build the local ship wiki entry for `ship_id`.
#[must_use]
pub fn build_local_ship_wiki(
    data: &GameData,
    lang: &LangMap,
    ship_id: u64,
    selection: ModuleSelection,
    config: &LocalBuildConfig,
) -> Option<LocalShipWiki> {
    let ship = data.ships.get(&ship_id)?;
    let build = build_ship_build(ship, selection);

    let art_module_name = ship
        .modules
        .get("_Artillery")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.get(selection.artillery))
        .and_then(|o| o.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("")
        .to_string();
    let main_battery = build
        .main_battery
        .as_ref()
        .map(|guns| main_battery_view(lang, data, &art_module_name, &build, guns));

    let secondaries = build
        .secondaries
        .as_ref()
        .map(|guns| main_battery_view(lang, data, "", &build, guns));

    let torpedoes = build
        .torpedoes
        .as_ref()
        .map(|torps| {
            let module_name = ship
                .modules
                .get("_Torpedoes")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.get(selection.torpedoes))
                .and_then(|o| o.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string();
            torpedo_view(lang, data, &module_name, torps)
        });

    let penetration_curves = main_battery
        .as_ref()
        .map(|mb| {
            mb.shells
                .iter()
                .filter_map(|shell| pen_curve(data, shell, mb.range_m))
                .collect()
        })
        .unwrap_or_default();

    let slots = module_slot_views(
        lang,
        ship,
        crate::wiki::ship_builder::module_slots(ship),
        selection,
        &build,
    );
    let combined = combined_modifiers(data, ship, config);
    let adjusted = apply_modifiers(&build, &ship.r#type, &combined, config.hp_fraction);
    let aircraft = aircraft_slot_views(
        data,
        lang,
        ship,
        selection,
        &combined,
        &ship.r#type,
    );
    let air_support_plane = build
        .air_support
        .as_ref()
        .and_then(|support| air_support_plane(data, lang, &support.plane, &combined, &ship.r#type));

    let similar_ships = similar_ships(data, lang, ship);

    Some(LocalShipWiki {
        ship_id,
        index: ship.index.clone(),
        name: lang.get(&ship.name),
        description: lang.get(&ship.description),
        year: lang.get(&ship.year),
        tier: ship.tier,
        nation: ship.region.clone(),
        region: lang.get(&ship.region_id),
        ship_type: lang.get(&ship.type_id),
        group: ship.group.clone(),
        premium: ship.group == "special",
        special: SPECIAL_GROUPS.contains(&ship.group.as_str()),
        cost_credit: ship.cost_cr,
        cost_gold: ship.cost_gold,
        cost_xp: ship.cost_xp,
        next_ships: next_ship_views(data, lang, ship),
        camo_count: ship.permoflages.len() as i64,
        camos: ship
            .permoflages
            .iter()
            .filter_map(|key| data.exteriors.get(key))
            .map(|name| lang.get(name))
            .collect(),
        camo_keys: ship.permoflages.clone(),
        modules: slots,
        hull: build.hull.clone(),
        main_battery,
        secondaries,
        torpedoes,
        air_defense: build.air_defense.clone(),
        fire_control: build.fire_control.clone(),
        engine: build.engine.clone(),
        depth_charges: build
            .depth_charges
            .as_ref()
            .map(|dc| depth_charge_view(lang, data, dc)),
        air_support: build.air_support.as_ref().map(airstrike_view),
        pinger: build.pinger.clone(),
        special_ability: build.special.as_ref().map(|special| special_ability_view(lang, special)),
        aircraft,
        air_support_plane,
        consumables: consumable_views(data, lang, ship),
        skills: skill_views(data, lang, ship, &config.skills),
        upgrades: upgrade_views(data, lang, ship, &config.upgrades),
        flags: flag_views(data, lang, ship, &config.flags),
        adjusted,
        hp_fraction: config.hp_fraction,
        spotted: config.spotted,
        penetration_curves,
        similar_ships,
        armor: armor_view(&build.hull, build.main_battery.as_ref()),
    })
}
