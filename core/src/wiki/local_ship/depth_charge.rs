//! Depth-charge view model.

use std::collections::HashSet;

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::components::DepthChargeStats;
use crate::wiki::gamedata::GameData;
use crate::wiki::LangMap;

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

pub(super) fn depth_charge_view(lang: &LangMap, data: &GameData, stats: &DepthChargeStats) -> DepthChargeView {
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

