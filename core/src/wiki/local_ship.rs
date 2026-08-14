//! Local ship wiki view model.
//!
//! Resolves one `wowsinfo.json` ship into everything the wiki detail screen
//! needs: localised metadata, the module tree, computed stats for the current
//! module selection, shell cards, AP penetration curves and similar ships.

use std::collections::HashSet;

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::aircraft_views::{aircraft_slot_views, air_support_plane, AircraftDetail, AircraftSlotView};
use super::compare::{similar_ships, SimilarShip};
use super::gamedata::{GameData, ShipInfo};
use super::penetration::{penetration_curve, BallisticShell};
use super::projectile::ProjectileInfo;
use super::components::{
    AirDefenseStats, AirSupportStats, BurstInfo, DepthChargeStats, EngineStats, FireControlStats,
    GunStats, HullStats, PingerStats, SpecialStats, TorpedoStats, WeaponInfo,
};
use super::loadouts::{
    combined_modifiers, consumable_views, flag_views, skill_views, upgrade_views,
    next_ship_views, ConsumableView, FlagView, LocalBuildConfig, NextShip, SkillView, UpgradeView,
};
use super::modifiers::apply_modifiers;
use super::ship_builder::{build_ship_build, ModuleOption, ModuleSelection, ShipBuild};
use super::LangMap;

/// One shell card shown on the wiki (resolved against the lang map).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ShellView {
    pub key: String,
    pub name: String,
    pub ammo_type: String,
    pub damage: i64,
    pub burn_chance: Option<f64>,
    pub pen_he: Option<f64>,
    pub pen_sap: Option<f64>,
    pub speed: f64,
    pub weight: f64,
    pub calibre_mm: f64,
    pub fuse_time: Option<f64>,
    pub ricochet_angle: Option<f64>,
    pub ricochet_always: Option<f64>,
    pub overmatch: Option<i64>,
    pub range: Option<f64>,
    pub flood_chance: Option<f64>,
    pub visibility: Option<f64>,
    pub alpha_damage: Option<f64>,
}

impl ShellView {
    pub(crate) fn from_projectile(lang: &LangMap, projectile: &ProjectileInfo) -> Self {
        Self {
            key: projectile.key.clone(),
            name: lang.get(&projectile.name),
            ammo_type: projectile.ammo_type.clone(),
            damage: projectile.damage.round() as i64,
            burn_chance: projectile.burn_chance,
            pen_he: projectile.pen_he,
            pen_sap: projectile.pen_sap,
            speed: projectile.speed,
            weight: projectile.weight,
            calibre_mm: projectile.calibre_mm(),
            fuse_time: projectile.fuse_time,
            ricochet_angle: projectile.ricochet_angle,
            ricochet_always: projectile.ricochet_always,
            overmatch: projectile.overmatch,
            range: projectile.range,
            flood_chance: projectile.flood_chance,
            visibility: projectile.visibility,
            alpha_damage: projectile.alpha_damage,
        }
    }
}

/// Resolved main battery / secondary battery view.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct MainBatteryView {
    pub name: String,
    pub range_m: f64,
    pub sigma: f64,
    pub reload_s: f64,
    pub rotation_deg_s: f64,
    pub configuration: String,
    pub burst: Option<BurstInfo>,
    pub shells: Vec<ShellView>,
}

/// Resolved torpedo view.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TorpedoView {
    pub name: String,
    pub single_shot: bool,
    pub reload_s: f64,
    pub rotation_deg_s: f64,
    pub configuration: String,
    pub shells: Vec<ShellView>,
}

/// One AP penetration curve for the chart.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct PenCurveView {
    pub shell_key: String,
    pub shell_name: String,
    pub points: Vec<super::penetration::PenetrationPoint>,
}

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
    pub modules: Vec<ModuleSlotView>,
    pub hull: Option<HullStats>,
    pub main_battery: Option<MainBatteryView>,
    pub secondaries: Option<MainBatteryView>,
    pub torpedoes: Option<TorpedoView>,
    pub air_defense: Option<AirDefenseStats>,
    pub fire_control: Option<FireControlStats>,
    pub engine: Option<EngineStats>,
    pub depth_charges: Option<DepthChargeStats>,
    pub air_support: Option<AirSupportStats>,
    pub pinger: Option<PingerStats>,
    pub special_ability: Option<SpecialStats>,
    /// Carrier squadrons (fighters, torpedo/dive/skip bombers).
    pub aircraft: Vec<AircraftSlotView>,
    /// Plane used by the air-support consumable (when present).
    pub air_support_plane: Option<AircraftDetail>,
    pub consumables: Vec<ConsumableView>,
    pub skills: Vec<SkillView>,
    pub upgrades: Vec<UpgradeView>,
    pub flags: Vec<FlagView>,
    /// Stats after skills/upgrades/flags/conditions are applied.
    pub adjusted: super::modifiers::AdjustedStats,
    pub hp_fraction: f64,
    pub spotted: bool,
    pub penetration_curves: Vec<PenCurveView>,
    pub similar_ships: Vec<SimilarShip>,
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

fn shells_from_weapons(lang: &LangMap, data: &GameData, weapons: &[WeaponInfo]) -> Vec<ShellView> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for weapon in weapons {
        for key in &weapon.ammo {
            if !seen.insert(key.clone()) {
                continue;
            }
            if let Some(projectile) = data.projectiles.get(key) {
                out.push(ShellView::from_projectile(lang, projectile));
            }
        }
    }
    out
}

fn configuration(weapons: &[WeaponInfo]) -> String {
    weapons
        .iter()
        .map(|w| format!("{} x {}", w.count, w.each))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main_battery_view(
    lang: &LangMap,
    data: &GameData,
    module_name: &str,
    build: &ShipBuild,
    guns: &GunStats,
) -> MainBatteryView {
    let first = guns.guns.first().cloned().unwrap_or_default();
    let mut shells = shells_from_weapons(lang, data, &guns.guns);
    if shells.is_empty() {
        shells = shells_from_weapons(lang, data, &[first.clone()]);
    }
    let mut view = MainBatteryView {
        name: lang.get(module_name),
        range_m: guns.range_m,
        sigma: guns.sigma,
        reload_s: first.reload,
        rotation_deg_s: first.rotation,
        configuration: configuration(&guns.guns),
        burst: guns.burst.clone(),
        shells,
    };
    if let Some(fc) = &build.fire_control
        && fc.max_dist_coef > 0.0
    {
        view.range_m *= fc.max_dist_coef;
    }
    view
}

fn torpedo_view(
    lang: &LangMap,
    data: &GameData,
    module_name: &str,
    torps: &TorpedoStats,
) -> TorpedoView {
    let first = torps.launchers.first().cloned().unwrap_or_default();
    TorpedoView {
        name: lang.get(module_name),
        single_shot: torps.single_shot,
        reload_s: first.reload,
        rotation_deg_s: first.rotation,
        configuration: configuration(&torps.launchers),
        shells: shells_from_weapons(lang, data, &torps.launchers),
    }
}

fn module_slot_views(
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
                    delta: super::ship_builder::module_option_delta(
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

fn pen_curve(data: &GameData, shell: &ShellView, max_range_m: f64) -> Option<PenCurveView> {
    let ap = data.projectiles.get(&shell.key)?.ap.clone()?;
    let ballistics = BallisticShell {
        mass_kg: ap.weight_kg.max(shell.weight),
        calibre_mm: shell.calibre_mm,
        muzzle_velocity: ap.velocity,
        drag: ap.drag,
        krupp: ap.krupp,
        normalization_deg: 0.0,
    };
    let points = penetration_curve(&ballistics, max_range_m, 21);
    Some(PenCurveView {
        shell_key: shell.key.clone(),
        shell_name: shell.name.clone(),
        points,
    })
}

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
        super::ship_builder::module_slots(ship),
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
        modules: slots,
        hull: build.hull.clone(),
        main_battery,
        secondaries,
        torpedoes,
        air_defense: build.air_defense.clone(),
        fire_control: build.fire_control.clone(),
        engine: build.engine.clone(),
        depth_charges: build.depth_charges.clone(),
        air_support: build.air_support.clone(),
        pinger: build.pinger.clone(),
        special_ability: build.special.clone(),
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
    })
}
