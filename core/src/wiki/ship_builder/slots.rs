//! Module slot list + option deltas.

use super::build::build_ship_build;
use super::helpers::parse_module_options;
use super::types::{ModuleOption, ModuleSelection, ShipBuild};
use crate::wiki::gamedata::ShipInfo;

pub fn module_slots(ship: &ShipInfo) -> Vec<(String, Vec<ModuleOption>)> {
    let slots = [
        ("_Hull", "hull"),
        ("_Artillery", "artillery"),
        ("_Torpedoes", "torpedoes"),
        ("_Suo", "fire_control"),
        ("_FlightControl", "flight_control"),
        ("_Engine", "engine"),
        ("_Fighter", "fighter"),
        ("_TorpedoBomber", "torpedo_bomber"),
        ("_DiveBomber", "dive_bomber"),
        ("_SkipBomber", "skip_bomber"),
    ];
    slots
        .into_iter()
        .filter_map(|(key, label)| {
            let options = parse_module_options(ship, key);
            (options.len() > 1).then_some((label.to_string(), options))
        })
        .collect()
}

/// Describe what selecting one module option changes vs the current build
/// (e.g. "HP +5,300 · Reload -3.0 s"). Empty when nothing meaningful changes.
#[must_use]
pub fn module_option_delta(
    ship: &ShipInfo,
    selection: ModuleSelection,
    slot: &str,
    option_index: usize,
    current: &ShipBuild,
) -> String {
    let mut next = selection;
    match slot {
        "hull" => next.hull = option_index,
        "artillery" => next.artillery = option_index,
        "torpedoes" => next.torpedoes = option_index,
        "fire_control" | "flight_control" => next.fire_control = option_index,
        "engine" => next.engine = option_index,
        "fighter" => next.fighter = option_index,
        "torpedo_bomber" => next.torpedo_bomber = option_index,
        "dive_bomber" => next.dive_bomber = option_index,
        "skip_bomber" => next.skip_bomber = option_index,
        _ => return String::new(),
    }
    if next == selection {
        return String::new();
    }
    let build = build_ship_build(ship, next);
    let mut parts = Vec::new();

    let health = |b: &ShipBuild| b.hull.as_ref().map_or(0.0, |h| h.health);
    let delta = health(&build) - health(current);
    if delta.abs() > 0.5 {
        parts.push(format!("HP {:+.0}", delta));
    }
    let reload = |b: &ShipBuild| {
        b.main_battery
            .as_ref()
            .and_then(|g| g.guns.first())
            .map_or(0.0, |w| w.reload)
    };
    let delta = reload(&build) - reload(current);
    if delta.abs() > 0.01 {
        parts.push(format!("Reload {:+.1}s", delta));
    }
    let range = |b: &ShipBuild| {
        b.main_battery
            .as_ref()
            .map_or(0.0, |g| g.range_m / 1000.0)
    };
    let delta = range(&build) - range(current);
    if delta.abs() > 0.05 {
        parts.push(format!("Range {:+.1}km", delta));
    }
    let torp_reload = |b: &ShipBuild| {
        b.torpedoes
            .as_ref()
            .and_then(|t| t.launchers.first())
            .map_or(0.0, |l| l.reload)
    };
    let delta = torp_reload(&build) - torp_reload(current);
    if delta.abs() > 0.01 {
        parts.push(format!("Torpedo reload {:+.1}s", delta));
    }
    let speed = |b: &ShipBuild| b.hull.as_ref().map_or(0.0, |h| h.mobility.speed);
    let delta = speed(&build) - speed(current);
    if delta.abs() > 0.01 {
        parts.push(format!("Speed {:+.1}kn", delta));
    }
    let concealment = |b: &ShipBuild| b.hull.as_ref().map_or(0.0, |h| h.visibility.sea);
    let delta = concealment(&build) - concealment(current);
    if delta.abs() > 0.01 {
        parts.push(format!("Concealment {:+.1}km", delta));
    }
    parts.truncate(3);
    parts.join(" · ")
}

