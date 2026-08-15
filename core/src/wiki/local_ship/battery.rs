//! Main / secondary battery view models.

use std::collections::HashSet;

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::dispersion::{dispersion_view, DispersionView};
use super::shell::ShellView;
use crate::wiki::components::{BurstInfo, GunStats, WeaponInfo};
use crate::wiki::gamedata::GameData;
use crate::wiki::ship_builder::ShipBuild;
use crate::wiki::LangMap;


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
    /// Fires per minute (`burn_chance * barrels * rof`), rounded to 2
    /// decimals (ShipBuilder `PotentialFpm`); 0 for non-HE shells.
    pub potential_fpm: f64,
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

pub(super) fn shells_from_weapons(lang: &LangMap, data: &GameData, weapons: &[WeaponInfo]) -> Vec<ShellView> {
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

pub(super) fn configuration(weapons: &[WeaponInfo]) -> String {
    weapons
        .iter()
        .map(|w| format!("{} x {}", w.count, w.each))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn main_battery_view(
    lang: &LangMap,
    data: &GameData,
    module_name: &str,
    build: &ShipBuild,
    guns: &GunStats,
) -> MainBatteryView {
    let first = guns.guns.first().cloned().unwrap_or_default();
    let mut shells = shells_from_weapons(lang, data, &guns.guns);
    if let Some(burst) = &guns.burst {
        for key in &burst.secondary_ammo {
            if let Some(projectile) = data.projectiles.get(key) {
                if !shells.iter().any(|shell| shell.key == *key) {
                    shells.push(ShellView::from_projectile(lang, projectile));
                }
            }
        }
    }
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
            potential_fpm: shell
                .burn_chance
                .map(|chance| ((chance * barrels as f64 * rof) * 100.0).round() / 100.0)
                .unwrap_or(0.0),
        })
        .collect();
    view
}
