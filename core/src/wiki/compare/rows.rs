//! Per-row value builders for the comparison table.

use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::ship_builder::{build_ship_build, ModuleSelection, ShipBuild};
use crate::wiki::LangMap;

fn fmt(value: f64, digits: usize) -> String {
    format!("{:.*}", digits, value)
}

fn fmt_int(value: f64) -> String {
    format!("{}", value.round() as i64)
}

fn main_battery_values(build: &ShipBuild) -> (String, String, String, String) {
    let Some(guns) = &build.main_battery else {
        return ("-".to_string(), "-".to_string(), "-".to_string(), "-".to_string());
    };
    let config = guns
        .guns
        .iter()
        .map(|g| format!("{} x {}", g.count, g.each))
        .collect::<Vec<_>>()
        .join(" ");
    let range = guns.range_m * build.fire_control.as_ref().map_or(1.0, |fc| fc.max_dist_coef.max(0.0));
    let reload = guns.guns.first().map_or(0.0, |g| g.reload);
    let sigma = guns.sigma;
    (
        config,
        format!("{} km", fmt(range / 1000.0, 1)),
        format!("{} s", fmt(reload, 1)),
        fmt(sigma, 2),
    )
}

fn torpedo_values(data: &GameData, build: &ShipBuild) -> (String, String) {
    let Some(torps) = &build.torpedoes else {
        return ("-".to_string(), "-".to_string());
    };
    let mut max_range: f64 = 0.0;
    let mut max_damage: f64 = 0.0;
    for launcher in &torps.launchers {
        for key in &launcher.ammo {
            let Some(shell) = data.projectiles.get(key) else {
                continue;
            };
            max_range = max_range.max(shell.range.unwrap_or(0.0) / (100.0 / 3.0));
            let damage = shell.alpha_damage.unwrap_or(0.0) / 3.0 + shell.damage;
            max_damage = max_damage.max(damage);
        }
    }
    let range = if max_range > 0.0 {
        format!("{} km", fmt(max_range, 1))
    } else {
        "-".to_string()
    };
    let damage = if max_damage > 0.0 {
        fmt_int(max_damage)
    } else {
        "-".to_string()
    };
    (range, damage)
}

fn aa_dps(build: &ShipBuild) -> String {
    let Some(aa) = &build.air_defense else {
        return "-".to_string();
    };
    let total = aa
        .near
        .iter()
        .chain(&aa.medium)
        .chain(&aa.far)
        .map(|aura| aura.dps)
        .sum::<f64>();
    fmt_int(total)
}

fn secondaries_values(build: &ShipBuild) -> String {
    build
        .secondaries
        .as_ref()
        .map(|guns| {
            guns.guns
                .iter()
                .map(|g| format!("{} x {}", g.count, g.each))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_else(|| "-".to_string())
}

/// Build one ship's comparison row values.
pub(super) fn ship_values(data: &GameData, lang: &LangMap, ship: &ShipInfo) -> Vec<String> {
    let build = build_ship_build(ship, ModuleSelection::default());
    let (config, range, reload, sigma) = main_battery_values(&build);
    let (torp_range, torp_damage) = torpedo_values(data, &build);
    let hull = &build.hull;
    vec![
        ship.tier.to_string(),
        lang.get(&ship.type_id),
        ship.region.clone(),
        hull.as_ref().map_or_else(|| "-".to_string(), |h| fmt_int(h.health)),
        hull.as_ref()
            .map_or_else(|| "-".to_string(), |h| format!("{} kn", fmt(h.mobility.speed, 1))),
        hull.as_ref()
            .map_or_else(|| "-".to_string(), |h| format!("{} s", fmt(h.mobility.rudder_time, 1))),
        hull.as_ref()
            .map_or_else(|| "-".to_string(), |h| format!("{} km", fmt(h.visibility.sea, 1))),
        config,
        range,
        reload,
        sigma,
        torp_range,
        torp_damage,
        aa_dps(&build),
        secondaries_values(&build),
    ]
}

