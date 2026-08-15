//! Apply the combined modifier set to a ship build.

use super::adjusted::AdjustedStats;
use super::value::ModifierSet;
use crate::wiki::components::GunStats;
use crate::wiki::ship_builder::ShipBuild;

/// Effective low-HP reload multiplier: at full HP it is 1.0, at 0 HP it
/// reaches the `lastChanceReloadCoefficient` value (e.g. 0.2 = -80% reload).
pub(super) fn low_hp_multiplier(mods: &ModifierSet, ship_class: &str, hp_fraction: f64) -> f64 {
    let last_chance = mods.multiply(ship_class, "lastChanceReloadCoefficient");
    if !mods.has("lastChanceReloadCoefficient") {
        return 1.0;
    }
    let hp = hp_fraction.clamp(0.0, 1.0);
    1.0 - (1.0 - last_chance) * (1.0 - hp)
}

/// Apply the modifier set (with UI conditions) to a ship build.
#[must_use]
pub fn apply_modifiers(
    build: &ShipBuild,
    ship_class: &str,
    mods: &ModifierSet,
    hp_fraction: f64,
) -> AdjustedStats {
    let main = build.main_battery.as_ref();
    let torps = build.torpedoes.as_ref();
    let secondaries = build.secondaries.as_ref();
    let hull = build.hull.as_ref();

    // Low-HP reload reduction (Adrenaline Rush) affects every armament type.
    let low_hp = low_hp_multiplier(mods, ship_class, hp_fraction);

    let gun_reload = main
        .and_then(|g| g.guns.first())
        .map_or(0.0, |g| g.reload)
        * mods.multiply(ship_class, "GMShotDelay")
        * mods.multiply(ship_class, "reloadFactor")
        * mods.multiply(ship_class, "activeManeuveringReloadCoeff")
        * low_hp;
    let gun_range = main.map_or(0.0, |g| g.range_m)
        * build
            .fire_control
            .as_ref()
            .map_or(1.0, |fc| fc.max_dist_coef.max(0.0))
        * mods.multiply(ship_class, "GMMaxDist");
    let gun_rotation = main
        .map_or(0.0, GunStats::rotation_deg_s)
        * mods.multiply(ship_class, "GMRotationSpeed");

    let torp_reload = torps
        .and_then(|t| t.launchers.first())
        .map_or(0.0, |l| l.reload)
        * mods.multiply(ship_class, "GTShotDelay")
        * low_hp;
    let torp_rotation = torps
        .map_or(0.0, |t| {
            t.launchers
                .first()
                .map_or(0.0, |l| if l.rotation > 0.0 { 180.0 / l.rotation } else { 0.0 })
        })
        * mods.multiply(ship_class, "GTRotationSpeed");

    let secondary_reload = secondaries
        .and_then(|g| g.guns.first())
        .map_or(0.0, |g| g.reload)
        * mods.multiply(ship_class, "GSShotDelay")
        * low_hp;
    let secondary_range = secondaries.map_or(0.0, |g| g.range_m)
        * mods.multiply(ship_class, "GSMaxDist");

    let speed = hull.map_or(0.0, |h| h.mobility.speed) * mods.multiply(ship_class, "speedCoef");
    let rudder = hull.map_or(0.0, |h| h.mobility.rudder_time)
        * mods.multiply(ship_class, "SGRudderTime");
    let visibility = mods.multiply(ship_class, "visibilityDistCoeff")
        * mods.multiply(ship_class, "visibilityFactor");
    let concealment_sea = hull.map_or(0.0, |h| h.visibility.sea) * visibility;
    let concealment_air = hull.map_or(0.0, |h| h.visibility.plane) * visibility;

    let aa_dps = build
        .air_defense
        .as_ref()
        .map(|aa| {
            aa.near
                .iter()
                .chain(&aa.medium)
                .chain(&aa.far)
                .map(|aura| aura.dps)
                .sum::<f64>()
        })
        .unwrap_or(0.0)
        * mods.multiply(ship_class, "AAAuraDamage");

    let battery = hull.as_ref().and_then(|h| h.submarine_battery.as_ref());
    let battery_capacity = battery.map_or(0.0, |b| b.capacity as f64)
        * mods.multiply(ship_class, "batteryCapacityCoeff");
    let battery_regen = battery.map_or(0.0, |b| b.regen) * mods.multiply(ship_class, "batteryRegenCoeff");
    let pinger = build.pinger.as_ref();
    let pinger_reload = pinger.map_or(0.0, |p| p.reload)
        * mods.multiply(ship_class, "pingerReloadCoeff");
    let pinger_speed = pinger.map_or(0.0, |p| p.speed)
        * mods.multiply(ship_class, "pingerWaveSpeedCoeff")
        * mods.multiply(ship_class, "hydrophoneWaveSpeedCoeff");

    AdjustedStats {
        health: hull.map_or(0.0, |h| h.health) * mods.multiply(ship_class, "healthHullCoeff"),
        gun_reload_s: gun_reload,
        gun_range_m: gun_range,
        gun_rotation_deg_s: gun_rotation,
        torp_reload_s: torp_reload,
        torp_rotation_deg_s: torp_rotation,
        secondary_reload_s: secondary_reload,
        secondary_range_m: secondary_range,
        speed,
        rudder_time: rudder,
        concealment_sea,
        concealment_air,
        aa_dps,
        battery_capacity,
        battery_regen,
        consumable_reload_mult: mods.multiply(ship_class, "ConsumableReloadTime"),
        consumable_work_mult: mods.multiply(ship_class, "ConsumablesWorkTime"),
        consumable_charges_extra: mods.sum(ship_class, "additionalConsumables"),
        consumable_capacity_mult: mods.multiply(ship_class, "consumableCapacityCoeff"),
        pinger_reload_s: pinger_reload,
        pinger_speed,
    }
}

