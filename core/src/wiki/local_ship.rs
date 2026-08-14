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
    AirDefenseStats, AirSupportStats, BurstInfo, DepthChargeStats, DispersionStats, EngineStats,
    FireControlStats, GunStats, HullStats, PingerStats, SpecialStats, TorpedoStats, WeaponInfo,
};
use super::loadouts::{
    combined_modifiers, consumable_views, flag_views, modifier_lines, next_ship_views, skill_views,
    upgrade_views, ConsumableView, FlagView, LocalBuildConfig, NextShip, SkillView, UpgradeView,
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
    // Ballistics (ShipBuilder shell card).
    pub air_drag: Option<f64>,
    pub arming_threshold: Option<f64>,
    pub cap_normalize_max_angle: Option<f64>,
    pub explosion_radius: Option<f64>,
    pub krupp: Option<f64>,
    pub shell_cap: Option<bool>,
    pub splash_radius: Option<f64>,
    pub underwater_dist_factor: Option<f64>,
    pub underwater_penetration_factor: Option<f64>,
    pub water_drag: Option<f64>,
    /// Dispersion distance parameters (`distParams`, 4 values) + tile size.
    pub dist_params: Vec<f64>,
    pub dist_tile: Option<f64>,
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
            air_drag: projectile.air_drag,
            arming_threshold: projectile.arming_threshold,
            cap_normalize_max_angle: projectile.cap_normalize_max_angle,
            explosion_radius: projectile.explosion_radius,
            krupp: projectile.krupp,
            shell_cap: projectile.shell_cap,
            splash_radius: projectile.splash_radius,
            underwater_dist_factor: projectile.underwater_dist_factor,
            underwater_penetration_factor: projectile.underwater_penetration_factor,
            water_drag: projectile.water_drag,
            dist_params: projectile.dist_params.clone(),
            dist_tile: projectile.dist_tile,
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
    /// Caliber in mm (`battery.caliber * 1000`).
    pub caliber_mm: f64,
    /// Total barrel count (`battery.barrels`).
    pub barrels: i64,
    /// Rounds per minute per barrel (`battery.rof`).
    pub rof: f64,
    /// Horizontal turret traverse in deg/s (`battery.traverse[0]`).
    pub traverse_deg_s: f64,
    /// Time for a 180° turret turn in seconds.
    pub turn_time_s: f64,
    /// Ammo-switch time in seconds (`reload * ammoSwitchCoeff`).
    pub ammo_switch_s: f64,
    pub dispersion: Option<DispersionView>,
    pub firing_arcs: Vec<FiringArcView>,
    /// Per-shell-type DPM / full-salvo values.
    pub per_shell_dpm: Vec<ShellDpmView>,
}

/// Per-shell-type DPM and full-salvo values (ShipBuilder semantics).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ShellDpmView {
    pub shell_key: String,
    /// Theoretical DPM (`shell damage * barrels * rof`).
    pub dpm: i64,
    /// Full-salvo damage (`shell damage * barrels`).
    pub salvo_damage: i64,
    /// Full-salvo weight in kg (`shell weight * barrels`).
    pub salvo_weight_kg: f64,
}

/// One dispersion sample (ellipse radii in meters).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DispersionPointView {
    pub range_m: f64,
    pub horizontal_m: f64,
    pub vertical_m: f64,
}

/// Resolved dispersion model for a battery, computed in Rust.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DispersionView {
    pub normal_distribution: bool,
    pub taper_dist_m: f64,
    pub delim_dist_m: f64,
    /// Ellipse at the battery's maximum range.
    pub at_max: DispersionPointView,
    /// Samples at 5 km / 10 km (only when below max range).
    pub samples: Vec<DispersionPointView>,
    /// Horizontal formula, `X` = range in km, result in meters.
    pub formula_horizontal: String,
    /// Vertical-coefficient formula at long range, `X` = range in km.
    pub formula_vertical: String,
    /// Vertical-coefficient formula below the delimiter distance.
    pub formula_vertical_short: String,
}

/// One turret firing arc.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct FiringArcView {
    pub name: String,
    pub horiz_min: f64,
    pub horiz_max: f64,
    pub vert_min: f64,
    pub vert_max: f64,
}

/// Acoustic homing block of a torpedo.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AcousticDetectionView {
    pub countdown: f64,
    pub max_depth_level: f64,
    pub max_pitch: f64,
    pub max_yaw: f64,
    pub path_length: f64,
    pub search_angle: f64,
    pub search_radius: f64,
    pub speed_decr_coef: f64,
    pub vertical_acceleration: f64,
    pub yaw_change_speed: f64,
}

/// One resolved torpedo type in a launcher bank.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TorpedoDetailView {
    pub key: String,
    pub name: String,
    pub deep_water: bool,
    /// Display damage: `(alpha / 3 + damage).round()`.
    pub damage: i64,
    pub alpha_damage: i64,
    /// Full-salvo damage (`torpedo_count * damage`).
    pub salvo_damage: i64,
    /// Range in km (`range / (100/3)`).
    pub range_km: f64,
    pub speed_kt: f64,
    /// Detectability in km.
    pub detectability_km: f64,
    /// Reaction time in seconds (`detectability / (speed * 2.6854) * 1000`).
    pub reaction_time_s: f64,
    pub arming_distance_m: Option<f64>,
    pub depth_m: Option<f64>,
    pub flood_chance: Option<f64>,
    pub splash_armor_coeff: Option<f64>,
    pub splash_cube_size: Option<f64>,
    /// Ping damage coefficient (`damageCoeffMaxPing`).
    pub ping_damage_coeff: Option<f64>,
    pub acoustic_detection: Option<AcousticDetectionView>,
    pub can_hit_classes: Vec<String>,
}

/// Resolved special ability (F / rage mode) view.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SpecialAbilityView {
    /// Humanised mode name (e.g. "Main Gun Accuracy").
    pub name: String,
    /// Raw mode key (e.g. `main_gun_accuracy`).
    pub mode: String,
    pub duration_s: f64,
    pub preparation_s: f64,
    pub progress_per_action: f64,
    pub progress_name: String,
    pub required_count: i64,
    pub sub_ribbons: Vec<i64>,
    pub time_limit_s: f64,
    pub separate_tracking: bool,
    pub start_enabled: bool,
    pub inactivity_delay_s: f64,
    pub progress_loss_interval_s: f64,
    pub progress_loss_per_interval: f64,
    pub auto_usage: bool,
    /// Friendly modifier lines ("Secondary reload -25%", ...).
    pub modifiers: Vec<String>,
}

/// Resolved torpedo view.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TorpedoView {
    pub name: String,
    pub single_shot: bool,
    pub reload_s: f64,
    pub rotation_deg_s: f64,
    /// Time for a 180° launcher turn in seconds.
    pub turn_time_s: f64,
    pub configuration: String,
    pub shells: Vec<ShellView>,
    /// Total torpedo tubes across all launchers.
    pub torpedo_count: i64,
    /// One detail card per torpedo type carried by the launchers.
    pub torpedoes: Vec<TorpedoDetailView>,
}

/// One depth-charge damage-coefficient segment (`pointsOfDamage`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargePointView {
    pub range: f64,
    pub coefficient: f64,
}

/// Resolved depth-charge pack settings (`depthCharge.packs`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargePackView {
    pub shots: i64,
    pub shots_in_pack: i64,
    pub max_packs: i64,
    pub shot_delay_s: f64,
    pub guns_sequence_type: i64,
    pub center_zone_width_part: f64,
    pub use_shot_nodes_for_sequence: bool,
}

/// One resolved depth-charge thrower.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargeLauncherView {
    pub name: String,
    pub bombs: i64,
    pub shoot_angle_deg: f64,
    pub shoot_distance: f64,
    pub start_fall_speed: f64,
    /// `min° .. max°` horizontal sector.
    pub horizontal_sector: String,
    /// `min° .. max°` vertical sector.
    pub vertical_sector: String,
    pub roll_speed: f64,
}

/// Buoyancy-state damage coefficient (`buoyancyToDamageCoeff`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct BuoyancyView {
    pub state: String,
    pub coefficient: f64,
}

/// Resolved depth-charge panel (flutter-two basics + ShipBuilder pack/ammo).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargeView {
    pub name: String,
    pub reload_s: f64,
    pub groups: i64,
    pub bombs: i64,
    /// Bombs dropped per attack (`sum(launcher.numBombs) * numShots`).
    pub bombs_per_charge: i64,
    pub launcher_count: i64,
    pub packs: Option<DepthChargePackView>,
    pub launchers: Vec<DepthChargeLauncherView>,
    pub damage: f64,
    /// Fire chance in percent.
    pub fire_chance: f64,
    /// Flood chance in percent.
    pub flood_chance: f64,
    /// Raw bundle `sinkSpeed` (game `bulletSpeed`).
    pub sink_speed: Option<f64>,
    /// Detonation depth in metres (absolute value of the raw field).
    pub detonation_depth_m: Option<f64>,
    pub splash_radius_m: Option<f64>,
    pub alert_dist: Option<f64>,
    pub explosive_power: Option<f64>,
    pub integral_power: Option<f64>,
    pub fall_distance: Option<f64>,
    pub fall_time: Option<f64>,
    pub points_of_damage: Vec<DepthChargePointView>,
    pub can_hit_classes: Vec<String>,
    pub ignore_classes: Vec<String>,
    pub buoyancy: Vec<BuoyancyView>,
}

/// Resolved airstrike / ASW panel.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirstrikeView {
    pub auto_usage: bool,
    pub charges: i64,
    pub reload_s: f64,
    /// Strike range in km (component `range`).
    pub range_km: f64,
    /// Minimum strike distance in metres.
    pub min_dist_m: f64,
    /// Maximum strike distance in metres.
    pub max_dist_m: f64,
    /// Maximum flight distance of the strike plane in metres.
    pub max_plane_flight_dist_m: f64,
    pub climb_angle_deg: f64,
    pub fly_away_time_s: f64,
    pub time_between_shots_s: f64,
    /// Time the strike plane spends over the target ("time from heaven").
    pub time_from_heaven_s: f64,
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
    pub adjusted: super::modifiers::AdjustedStats,
    pub hp_fraction: f64,
    pub spotted: bool,
    pub penetration_curves: Vec<PenCurveView>,
    pub similar_ships: Vec<SimilarShip>,
    /// Armor digest (hull zones + turret/barbette armor).
    pub armor: Option<ArmorView>,
}

/// One hull-armor thickness group (zone distribution).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ZoneThicknessGroup {
    pub thickness: f64,
    pub count: i64,
}

/// One turret's armor block (turret face + barbette).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TurretArmorView {
    pub name: String,
    pub caliber: f64,
    pub barrels: i64,
    pub armor: f64,
    pub barbette: f64,
}

/// The armor digest for the ship detail screen.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ArmorView {
    pub zone_count: i64,
    pub max_zone_thickness: f64,
    pub zone_groups: Vec<ZoneThicknessGroup>,
    pub turrets: Vec<TurretArmorView>,
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
    let battery = guns.battery.as_ref();
    let barrels = battery.map_or_else(
        || guns.turrets.iter().map(|t| t.barrels).sum::<i64>().max(1),
        |b| b.barrels.max(1),
    );
    let rof = battery.map_or_else(
        || {
            if first.reload > 0.0 {
                60.0 / first.reload
            } else {
                0.0
            }
        },
        |b| b.rof,
    );
    let traverse_deg_s = guns.rotation_deg_s();
    let turn_time_s = if traverse_deg_s > 0.0 {
        180.0 / traverse_deg_s
    } else {
        0.0
    };
    let caliber_mm = battery.map_or_else(
        || shells.first().map_or(0.0, |s| s.calibre_mm),
        |b| b.caliber * 1000.0,
    );
    let mut view = MainBatteryView {
        name: lang.get(module_name),
        range_m: guns.range_m,
        sigma: guns.sigma,
        reload_s: first.reload,
        rotation_deg_s: traverse_deg_s,
        configuration: configuration(&guns.guns),
        burst: guns.burst.clone(),
        shells,
        caliber_mm,
        barrels,
        rof,
        traverse_deg_s,
        turn_time_s,
        ammo_switch_s: battery.map_or(0.0, |b| first.reload * b.ammo_switch_coeff),
        dispersion: None,
        firing_arcs: battery
            .map(|b| {
                b.firing_arcs
                    .iter()
                    .map(|arc| FiringArcView {
                        name: arc.name.clone(),
                        horiz_min: arc.horiz_min,
                        horiz_max: arc.horiz_max,
                        vert_min: arc.vert_min,
                        vert_max: arc.vert_max,
                    })
                    .collect()
            })
            .unwrap_or_default(),
        per_shell_dpm: Vec::new(),
    };
    if let Some(fc) = &build.fire_control
        && fc.max_dist_coef > 0.0
    {
        view.range_m *= fc.max_dist_coef;
    }
    view.dispersion = battery
        .and_then(|b| b.dispersion.as_ref())
        .filter(|d| d.ideal_distance > 0.0)
        .map(|d| dispersion_view(d, view.range_m));
    view.per_shell_dpm = view
        .shells
        .iter()
        .map(|shell| ShellDpmView {
            shell_key: shell.key.clone(),
            dpm: (shell.damage as f64 * barrels as f64 * rof).round() as i64,
            salvo_damage: shell.damage * barrels,
            salvo_weight_kg: shell.weight * barrels as f64,
        })
        .collect();
    view
}

/// Horizontal dispersion in meters at `range_m` (port of ShipBuilder's
/// `Dispersion.CalculateHorizontalDispersion`, modifier = 1.0 for base stats).
fn horizontal_dispersion(disp: &DispersionStats, range_m: f64) -> f64 {
    if range_m <= 0.0 {
        return 0.0;
    }
    let x = range_m / 30.0;
    let effective_taper = disp.taper_dist / 30.0;
    if effective_taper > 0.0 && x <= effective_taper {
        (x * (disp.ideal_radius - disp.min_radius) / disp.ideal_distance
            + disp.min_radius * (x / effective_taper))
            * 30.0
    } else {
        (x * (disp.ideal_radius - disp.min_radius) / disp.ideal_distance + disp.min_radius) * 30.0
    }
}

/// Vertical dispersion in meters at `range_m` (port of ShipBuilder's
/// `Dispersion.CalculateVerticalDispersion`).
fn vertical_dispersion(disp: &DispersionStats, max_range_m: f64, horizontal_m: f64, range_m: f64) -> f64 {
    if max_range_m <= 0.0 {
        return 0.0;
    }
    let max_range_bw = max_range_m / 30.0;
    let x = range_m / 30.0;
    let delim_dist = max_range_bw * disp.delim;
    let coeff = if x < delim_dist {
        if delim_dist > f64::EPSILON {
            disp.radius_on_zero + (disp.radius_on_delim - disp.radius_on_zero) * (x / delim_dist)
        } else {
            disp.radius_on_zero
        }
    } else {
        let denom = max_range_bw - delim_dist;
        if denom.abs() > f64::EPSILON {
            disp.radius_on_delim + (disp.radius_on_max - disp.radius_on_delim) * (x - delim_dist) / denom
        } else {
            disp.radius_on_max
        }
    };
    horizontal_m * coeff
}

/// Build the resolved dispersion model (base stats, no modifiers).
fn dispersion_view(disp: &DispersionStats, max_range_m: f64) -> DispersionView {
    let at_max_h = horizontal_dispersion(disp, max_range_m);
    let at_max = DispersionPointView {
        range_m: max_range_m,
        horizontal_m: at_max_h,
        vertical_m: vertical_dispersion(disp, max_range_m, at_max_h, max_range_m),
    };
    let mut samples = Vec::new();
    for range in [5_000.0, 10_000.0] {
        if range < max_range_m {
            let h = horizontal_dispersion(disp, range);
            samples.push(DispersionPointView {
                range_m: range,
                horizontal_m: h,
                vertical_m: vertical_dispersion(disp, max_range_m, h, range),
            });
        }
    }

    let max_range_bw = max_range_m / 30.0;
    let delim_dist = max_range_bw * disp.delim;
    let v_radius_coeff = if max_range_bw > 0.0 && (1.0 - disp.delim).abs() > f64::EPSILON {
        (disp.radius_on_max - disp.radius_on_delim) / (max_range_bw * (1.0 - disp.delim))
    } else {
        0.0
    };
    let h_coeff = if disp.ideal_distance > 0.0 {
        (disp.ideal_radius - disp.min_radius) / disp.ideal_distance * 1000.0
    } else {
        0.0
    };
    let formula_horizontal =
        format!("X * {} + {}", fmt_disp(h_coeff), fmt_disp(30.0 * disp.min_radius));
    let formula_vertical = format!(
        "(X * {} + {})",
        fmt_disp((v_radius_coeff / 30.0) * 1000.0),
        fmt_disp((-max_range_bw * disp.delim * v_radius_coeff) + disp.radius_on_delim),
    );
    let formula_vertical_short = if max_range_bw > 0.0 && delim_dist > f64::EPSILON {
        format!(
            "(X * {} + {})",
            fmt_disp(((disp.radius_on_delim - disp.radius_on_zero) / delim_dist / 30.0) * 1000.0),
            fmt_disp(disp.radius_on_zero),
        )
    } else {
        formula_vertical.clone()
    };

    DispersionView {
        normal_distribution: disp.normal_distribution,
        taper_dist_m: disp.taper_dist,
        delim_dist_m: max_range_m * disp.delim,
        at_max,
        samples,
        formula_horizontal,
        formula_vertical,
        formula_vertical_short,
    }
}

/// Format a dispersion-formula coefficient like C# `Math.Round(x, 4)`
/// (trailing zeros trimmed).
fn fmt_disp(value: f64) -> String {
    let s = format!("{value:.4}");
    let trimmed = s.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() || trimmed == "-0" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

fn torpedo_view(
    lang: &LangMap,
    data: &GameData,
    module_name: &str,
    torps: &TorpedoStats,
) -> TorpedoView {
    let first = torps.launchers.first().cloned().unwrap_or_default();
    // Legacy `rotation` is the 180° launcher turn time in seconds.
    let rotation_deg_s = if first.rotation > 0.0 {
        180.0 / first.rotation
    } else {
        0.0
    };
    let torpedo_count = torps.launchers.iter().map(|launcher| launcher.count * launcher.each).sum::<i64>();
    let mut seen = HashSet::new();
    let torpedoes = torps
        .launchers
        .iter()
        .flat_map(|launcher| launcher.ammo.iter())
        .filter(|key| seen.insert((*key).clone()))
        .filter_map(|key| {
            let projectile = data.projectiles.get(key)?;
            (projectile.r#type == "Torpedo").then(|| torpedo_detail(lang, projectile, torpedo_count))
        })
        .collect();
    TorpedoView {
        name: lang.get(module_name),
        single_shot: torps.single_shot,
        reload_s: first.reload,
        rotation_deg_s,
        turn_time_s: first.rotation,
        configuration: configuration(&torps.launchers),
        shells: shells_from_weapons(lang, data, &torps.launchers),
        torpedo_count,
        torpedoes,
    }
}

fn torpedo_detail(
    lang: &LangMap,
    projectile: &ProjectileInfo,
    torpedo_count: i64,
) -> TorpedoDetailView {
    let alpha = projectile.alpha_damage.unwrap_or(0.0);
    let damage = (alpha / 3.0 + projectile.damage).round() as i64;
    let speed = projectile.speed.max(0.0);
    let visibility = projectile.visibility.unwrap_or(0.0);
    let reaction_time = if speed > 0.0 {
        visibility / speed / 2.6854 * 1000.0
    } else {
        0.0
    };
    TorpedoDetailView {
        key: projectile.key.clone(),
        name: lang.get(&projectile.name),
        deep_water: projectile.deep_water,
        damage,
        alpha_damage: alpha.round() as i64,
        salvo_damage: damage * torpedo_count,
        range_km: projectile.range.map_or(0.0, |range| range / (100.0 / 3.0)),
        speed_kt: speed,
        detectability_km: visibility,
        reaction_time_s: reaction_time,
        arming_distance_m: projectile.arming_distance,
        depth_m: projectile.depth,
        flood_chance: projectile.flood_chance,
        splash_armor_coeff: projectile.splash_armor_coeff,
        splash_cube_size: projectile.splash_cube_size,
        ping_damage_coeff: projectile.damage_coeff_max_ping,
        acoustic_detection: projectile.acoustic_detection.as_ref().map(|ad| AcousticDetectionView {
            countdown: ad.countdown,
            max_depth_level: ad.max_depth_level,
            max_pitch: ad.max_pitch,
            max_yaw: ad.max_yaw,
            path_length: ad.path_length,
            search_angle: ad.search_angle,
            search_radius: ad.search_radius,
            speed_decr_coef: ad.speed_decr_coef,
            vertical_acceleration: ad.vertical_acceleration,
            yaw_change_speed: ad.yaw_change_speed,
        }),
        can_hit_classes: projectile.can_hit_classes.clone(),
    }
}

/// All ship classes known to the game, used to derive depth-charge hit
/// classes from the `ignoreClasses` list.
const SHIP_CLASSES: [&str; 6] = [
    "AirCarrier",
    "Auxiliary",
    "Battleship",
    "Cruiser",
    "Destroyer",
    "Submarine",
];

/// Normalise a chance to percent (fractions <= 1 are scaled by 100; the
/// bundle already stores some chances as percentages).
fn percent(value: f64) -> f64 {
    if value.abs() <= 1.0 {
        value * 100.0
    } else {
        value
    }
}

fn depth_charge_view(lang: &LangMap, data: &GameData, stats: &DepthChargeStats) -> DepthChargeView {
    let projectile = data.projectiles.get(&stats.ammo);
    let name = projectile
        .map(|p| lang.get(&p.name))
        .unwrap_or_else(|| lang.get(&stats.ammo));
    let fire_chance = projectile
        .and_then(|p| p.fire_chance.or(p.burn_chance))
        .map(percent)
        .unwrap_or(0.0);
    let flood_chance = projectile
        .and_then(|p| p.flood_chance)
        .map(percent)
        .unwrap_or(0.0);
    let ignore: HashSet<&str> = projectile
        .map(|p| p.ignore_classes.iter().map(String::as_str).collect())
        .unwrap_or_default();
    let can_hit = SHIP_CLASSES
        .iter()
        .filter(|class| !ignore.contains(**class))
        .map(|class| class.to_string())
        .collect();
    let packs = (stats.packs.num_shots > 0
        || stats.packs.max_packs > 0
        || stats.packs.shot_delay > 0.0)
        .then(|| DepthChargePackView {
            shots: stats.packs.num_shots,
            shots_in_pack: stats.packs.shots_in_pack,
            max_packs: stats.packs.max_packs,
            shot_delay_s: stats.packs.shot_delay,
            guns_sequence_type: stats.packs.guns_sequence_type,
            center_zone_width_part: stats.packs.center_zone_width_part,
            use_shot_nodes_for_sequence: stats.packs.use_shot_nodes_for_sequence,
        });
    let launchers = stats
        .launchers
        .iter()
        .map(|l| DepthChargeLauncherView {
            name: l.name.clone(),
            bombs: l.num_bombs,
            shoot_angle_deg: l.shoot_angle,
            shoot_distance: l.shoot_dist,
            start_fall_speed: l.start_fall_speed,
            horizontal_sector: format!("{}° .. {}°", l.horiz_sector_min, l.horiz_sector_max),
            vertical_sector: format!("{}° .. {}°", l.vert_sector_min, l.vert_sector_max),
            roll_speed: l.roll_speed,
        })
        .collect::<Vec<_>>();
    DepthChargeView {
        name,
        reload_s: stats.reload,
        groups: stats.groups,
        bombs: stats.bombs,
        bombs_per_charge: stats
            .launchers
            .iter()
            .map(|l| l.num_bombs)
            .sum::<i64>()
            * stats.packs.num_shots.max(1),
        launcher_count: stats.launchers.len() as i64,
        packs,
        launchers,
        damage: projectile.map_or(0.0, |p| p.damage),
        fire_chance,
        flood_chance,
        sink_speed: projectile.and_then(|p| p.sink_speed),
        detonation_depth_m: projectile.and_then(|p| p.detonation_depth).map(f64::abs),
        splash_radius_m: projectile.and_then(|p| p.splash_radius),
        alert_dist: projectile.and_then(|p| p.alert_dist),
        explosive_power: projectile.and_then(|p| p.explosive_power),
        integral_power: projectile.and_then(|p| p.integral_power),
        fall_distance: projectile.and_then(|p| p.fall_distance),
        fall_time: projectile.and_then(|p| p.fall_time),
        points_of_damage: projectile
            .map(|p| {
                p.points_of_damage
                    .iter()
                    .map(|(range, coefficient)| DepthChargePointView {
                        range: *range,
                        coefficient: *coefficient,
                    })
                    .collect()
            })
            .unwrap_or_default(),
        can_hit_classes: can_hit,
        ignore_classes: projectile
            .map(|p| p.ignore_classes.clone())
            .unwrap_or_default(),
        buoyancy: projectile
            .map(|p| {
                p.buoyancy_to_damage_coeff
                    .iter()
                    .map(|(state, coefficient)| BuoyancyView {
                        state: state.clone(),
                        coefficient: *coefficient,
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn airstrike_view(stats: &AirSupportStats) -> AirstrikeView {
    AirstrikeView {
        auto_usage: stats.airstrike.auto_usage,
        charges: stats.charges_num.max(stats.airstrike.charges_num),
        reload_s: stats.reload.max(stats.airstrike.reload_time),
        range_km: stats.range,
        min_dist_m: stats.airstrike.min_dist,
        max_dist_m: stats.airstrike.max_dist,
        max_plane_flight_dist_m: stats.airstrike.max_plane_flight_dist,
        climb_angle_deg: stats.airstrike.climb_angle,
        fly_away_time_s: stats.airstrike.fly_away_time,
        time_between_shots_s: stats.airstrike.time_between_shots,
        time_from_heaven_s: stats.airstrike.time_from_heaven,
    }
}

fn special_ability_view(lang: &LangMap, stats: &SpecialStats) -> SpecialAbilityView {
    SpecialAbilityView {
        name: humanize_mode(&stats.mode),
        mode: stats.mode.clone(),
        duration_s: stats.boost_duration,
        preparation_s: stats.boost_preparation,
        progress_per_action: stats.progress_per_action,
        progress_name: stats.progress_name.clone(),
        required_count: stats.required_count,
        sub_ribbons: stats.sub_ribbons.clone(),
        time_limit_s: stats.time_limit,
        separate_tracking: stats.separate_tracking,
        start_enabled: stats.start_enabled,
        inactivity_delay_s: stats.decrement_delay,
        progress_loss_interval_s: stats.decrement_period,
        progress_loss_per_interval: stats.decrement_count,
        auto_usage: stats.auto_usage,
        modifiers: modifier_lines(lang, &stats.modifiers),
    }
}

/// Humanise a raw rage-mode key (`main_gun_accuracy` -> "Main Gun Accuracy").
fn humanize_mode(mode: &str) -> String {
    if mode.is_empty() {
        return "Special Ability".to_string();
    }
    mode.split('_')
        .filter(|word| !word.is_empty() && *word != "te")
        .map(|word| match word {
            "atba" => "Secondary".to_string(),
            _ => {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
    // Dense enough that the app's range slider can interpolate smoothly.
    let points = penetration_curve(&ballistics, max_range_m, 101);
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

/// Build the armor digest from the hull `armor` block plus main-battery
/// turret data. The barbette value is the thickest zone in the turret's
/// barbette group (there is no plate geometry in `wowsinfo.json`).
fn armor_view(hull: &Option<HullStats>, main_battery: Option<&GunStats>) -> Option<ArmorView> {
    let armor = hull.as_ref()?.armor.as_ref()?;
    let mut zone_groups: Vec<ZoneThicknessGroup> = Vec::new();
    for zone in &armor.zones {
        match zone_groups
            .iter_mut()
            .find(|group| (group.thickness - zone.thickness).abs() < f64::EPSILON)
        {
            Some(group) => group.count += 1,
            None => zone_groups.push(ZoneThicknessGroup {
                thickness: zone.thickness,
                count: 1,
            }),
        }
    }
    zone_groups.sort_by(|a, b| b.thickness.total_cmp(&a.thickness));
    let turrets = main_battery
        .map(|guns| {
            guns.turrets
                .iter()
                .map(|turret| TurretArmorView {
                    name: turret.name.clone(),
                    caliber: turret.caliber,
                    barrels: turret.barrels,
                    armor: turret.armor,
                    barbette: armor
                        .barbettes
                        .iter()
                        .find(|barbette| barbette.turret == turret.name)
                        .map(|barbette| barbette.max_thickness)
                        .unwrap_or(0.0),
                })
                .collect()
        })
        .unwrap_or_default();
    Some(ArmorView {
        zone_count: armor.zones.len() as i64,
        max_zone_thickness: armor.zones.first().map(|zone| zone.thickness).unwrap_or(0.0),
        zone_groups,
        turrets,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::components::BatteryStats;

    fn kawachi_dispersion() -> DispersionStats {
        DispersionStats {
            normal_distribution: true,
            taper_dist: 3000.0,
            delim: 0.5,
            ellipse_range_min: 50.0,
            ellipse_range_max: 250.0,
            radius_on_zero: 0.2,
            radius_on_delim: 0.6,
            radius_on_max: 0.8,
            ideal_distance: 1000.0,
            ideal_radius: 10.0,
            min_radius: 2.8,
            ..DispersionStats::default()
        }
    }

    #[test]
    fn dispersion_matches_shipbuilder_at_max_and_samples() {
        let disp = kawachi_dispersion();
        let max = 9880.0;
        let h = horizontal_dispersion(&disp, max);
        let v = vertical_dispersion(&disp, max, h, max);
        // ShipBuilder ground truth: 155.136 m horizontal, 124.1088 m vertical.
        assert!((h - 155.136).abs() < 1e-3, "horizontal at max: {h}");
        assert!((v - 124.1088).abs() < 1e-3, "vertical at max: {v}");

        // Short-range sample at 5 km (still beyond the 3 km taper distance).
        let h5 = horizontal_dispersion(&disp, 5000.0);
        let v5 = vertical_dispersion(&disp, max, h5, 5000.0);
        assert!((h5 - 120.0).abs() < 1e-6, "horizontal at 5 km: {h5}");
        assert!((v5 - 72.2915).abs() < 1e-3, "vertical at 5 km: {v5}");
    }

    #[test]
    fn dispersion_taper_branch_scales_radius() {
        // Within the taper distance the formula adds MinRadius * (x / taper).
        let disp = kawachi_dispersion();
        let h = horizontal_dispersion(&disp, 1500.0);
        let x = 1500.0 / 30.0;
        let expected = (x * (disp.ideal_radius - disp.min_radius) / disp.ideal_distance
            + disp.min_radius * (x / (disp.taper_dist / 30.0)))
            * 30.0;
        assert!((h - expected).abs() < 1e-9);
    }

    #[test]
    fn rotation_deg_s_prefers_battery_traverse() {
        let guns = GunStats {
            battery: Some(BatteryStats {
                traverse: vec![5.0, 5.0],
                ..BatteryStats::default()
            }),
            guns: vec![WeaponInfo {
                rotation: 36.0,
                ..WeaponInfo::default()
            }],
            ..GunStats::default()
        };
        assert_eq!(guns.rotation_deg_s(), 5.0);

        // Legacy fallback: legacy `rotation` is the 180° turn time.
        let legacy = GunStats {
            guns: vec![WeaponInfo {
                rotation: 36.0,
                ..WeaponInfo::default()
            }],
            ..GunStats::default()
        };
        assert!((legacy.rotation_deg_s() - 5.0).abs() < 1e-9);
    }
}
