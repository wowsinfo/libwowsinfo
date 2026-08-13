//! Generic modifier engine for commander skills, module upgrades and flags.
//!
//! Wargaming attaches a free-form modifier map to skills, modernizations and
//! exteriors: each key maps to either a plain number (all ship classes) or a
//! per-class dict. Values are multiplicative on top of the base stats, and a
//! few conditional keys (spotted/unspotted triggers, low-HP reload) are
//! resolved with the UI state. This mirrors the Flutter two `Modifiers` model
//! without hard-coding every key.

use std::collections::HashMap;

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ship_builder::ShipBuild;

/// A modifier value: common to all classes or per ship class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Facet)]
#[repr(C)]
pub enum ModifierValue {
    Number(f64),
    PerShipType(HashMap<String, f64>),
}

impl ModifierValue {
    /// Resolve the value for one ship class (defaults to 1.0 = no effect).
    #[must_use]
    pub fn for_class(&self, ship_class: &str) -> f64 {
        match self {
            ModifierValue::Number(value) => *value,
            ModifierValue::PerShipType(map) => map.get(ship_class).copied().unwrap_or(1.0),
        }
    }
}

/// An ordered modifier set (order matters for the merge rule).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ModifierSet {
    pub entries: Vec<(String, ModifierValue)>,
}

impl ModifierSet {
    /// Merge another set into this one. Numbers multiply; additive keys
    /// (`Additional`, `Extra`) are summed so counts like "+1 consumable" work.
    #[must_use]
    pub fn merged(mut self, other: &ModifierSet) -> Self {
        for (key, value) in &other.entries {
            if let Some((_, existing)) = self.entries.iter_mut().find(|(k, _)| k == key) {
                *existing = merge_values(existing, value, is_additive(key));
                continue;
            }
            self.entries.push((key.clone(), value.clone()));
        }
        self
    }

    /// Product of every entry with `key`, resolved for `ship_class`.
    #[must_use]
    pub fn multiply(&self, ship_class: &str, key: &str) -> f64 {
        self.entries
            .iter()
            .filter(|(k, _)| k == key)
            .map(|(_, value)| value.for_class(ship_class))
            .product::<f64>()
    }

    /// True when the set contains at least one entry for `key`.
    #[must_use]
    pub fn has(&self, key: &str) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }
}

fn is_additive(key: &str) -> bool {
    key.contains("Additional") || key.contains("Extra") || key == "additionalConsumables"
}

fn merge_values(a: &ModifierValue, b: &ModifierValue, additive: bool) -> ModifierValue {
    match (a, b) {
        (ModifierValue::Number(a), ModifierValue::Number(b)) => {
            ModifierValue::Number(if additive { a + b } else { a * b })
        }
        (ModifierValue::PerShipType(a), ModifierValue::PerShipType(b)) => {
            let mut out = a.clone();
            for (class, value) in b {
                let entry = out.entry(class.clone()).or_insert(1.0);
                *entry = if additive { *entry + *value } else { *entry * *value };
            }
            ModifierValue::PerShipType(out)
        }
        _ => a.clone(),
    }
}

/// Parse a game-data `modifiers` map into a set.
#[must_use]
pub fn parse_modifiers(json: &Value) -> ModifierSet {
    let mut entries = Vec::new();
    let Some(map) = json.as_object() else {
        return ModifierSet { entries };
    };
    for (key, value) in map {
        let parsed = if let Some(number) = value.as_f64() {
            ModifierValue::Number(number)
        } else if let Some(classes) = value.as_object() {
            let per_class: HashMap<String, f64> = classes
                .iter()
                .filter_map(|(class, v)| v.as_f64().map(|number| (class.clone(), number)))
                .collect();
            ModifierValue::PerShipType(per_class)
        } else {
            continue;
        };
        entries.push((key.clone(), parsed));
    }
    ModifierSet { entries }
}

/// Stats after applying the selected modifiers, conditions and HP level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Facet)]
pub struct AdjustedStats {
    pub health: f64,
    pub gun_reload_s: f64,
    pub gun_range_m: f64,
    pub gun_rotation_deg_s: f64,
    pub torp_reload_s: f64,
    pub torp_rotation_deg_s: f64,
    pub secondary_reload_s: f64,
    pub secondary_range_m: f64,
    pub speed: f64,
    pub rudder_time: f64,
    pub concealment_sea: f64,
    pub concealment_air: f64,
    pub aa_dps: f64,
    pub battery_capacity: f64,
    pub battery_regen: f64,
}

impl Default for AdjustedStats {
    fn default() -> Self {
        Self {
            health: 0.0,
            gun_reload_s: 0.0,
            gun_range_m: 0.0,
            gun_rotation_deg_s: 0.0,
            torp_reload_s: 0.0,
            torp_rotation_deg_s: 0.0,
            secondary_reload_s: 0.0,
            secondary_range_m: 0.0,
            speed: 0.0,
            rudder_time: 0.0,
            concealment_sea: 0.0,
            concealment_air: 0.0,
            aa_dps: 0.0,
            battery_capacity: 0.0,
            battery_regen: 0.0,
        }
    }
}

/// Effective low-HP reload multiplier: at full HP it is 1.0, at 0 HP it
/// reaches the `lastChanceReloadCoefficient` value (e.g. 0.2 = -80% reload).
fn low_hp_multiplier(mods: &ModifierSet, ship_class: &str, hp_fraction: f64) -> f64 {
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
    let hull = &build.hull;
    let main = &build.main_battery;
    let torps = &build.torpedoes;
    let secondaries = &build.secondaries;

    let main = build.main_battery.as_ref();
    let torps = build.torpedoes.as_ref();
    let secondaries = build.secondaries.as_ref();
    let hull = build.hull.as_ref();

    let gun_reload = main
        .and_then(|g| g.guns.first())
        .map_or(0.0, |g| g.reload)
        * mods.multiply(ship_class, "GMShotDelay")
        * mods.multiply(ship_class, "reloadFactor")
        * mods.multiply(ship_class, "activeManeuveringReloadCoeff")
        * low_hp_multiplier(mods, ship_class, hp_fraction);
    let gun_range = main.map_or(0.0, |g| g.range_m)
        * build
            .fire_control
            .as_ref()
            .map_or(1.0, |fc| fc.max_dist_coef.max(0.0))
        * mods.multiply(ship_class, "GMMaxDist");
    let gun_rotation = main
        .and_then(|g| g.guns.first())
        .map_or(0.0, |g| g.rotation)
        * mods.multiply(ship_class, "GMRotationSpeed");

    let torp_reload = torps
        .and_then(|t| t.launchers.first())
        .map_or(0.0, |l| l.reload)
        * mods.multiply(ship_class, "GTShotDelay");
    let torp_rotation = torps
        .and_then(|t| t.launchers.first())
        .map_or(0.0, |l| l.rotation)
        * mods.multiply(ship_class, "GTRotationSpeed");

    let secondary_reload = secondaries
        .and_then(|g| g.guns.first())
        .map_or(0.0, |g| g.reload)
        * mods.multiply(ship_class, "GSShotDelay");
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_scalar_and_per_class_values() {
        let set = parse_modifiers(&json!({
            "speedCoef": 1.05,
            "GMShotDelay": {"Cruiser": 0.9, "Battleship": 0.95}
        }));
        assert_eq!(set.multiply("Cruiser", "speedCoef"), 1.05);
        assert_eq!(set.multiply("Cruiser", "GMShotDelay"), 0.9);
        assert_eq!(set.multiply("Destroyer", "GMShotDelay"), 1.0);
        assert_eq!(set.multiply("Cruiser", "missing"), 1.0);
    }

    #[test]
    fn merge_multiplies_and_sums() {
        let a = parse_modifiers(&json!({"GMShotDelay": 0.9, "additionalConsumables": 1}));
        let b = parse_modifiers(&json!({"GMShotDelay": 0.95, "additionalConsumables": 1}));
        let merged = a.merged(&b);
        assert!((merged.multiply("Cruiser", "GMShotDelay") - 0.855).abs() < 1e-9);
        assert!((merged.multiply("Cruiser", "additionalConsumables") - 2.0).abs() < 1e-9);
    }

    #[test]
    fn low_hp_reload_scales() {
        let mods = parse_modifiers(&json!({"lastChanceReloadCoefficient": 0.2}));
        assert!((low_hp_multiplier(&mods, "Cruiser", 1.0) - 1.0).abs() < 1e-9);
        assert!((low_hp_multiplier(&mods, "Cruiser", 0.0) - 0.2).abs() < 1e-9);
        assert!((low_hp_multiplier(&mods, "Cruiser", 0.5) - 0.6).abs() < 1e-9);
    }

    #[test]
    fn applies_to_build() {
        let json = serde_json::json!({
            "modules": {
                "_Hull": [{
                    "index": 0, "name": "H", "cost": {"costXP": 0, "costCR": 0},
                    "components": {"hull": ["H1"], "artillery": ["G1"], "airDefense": ["AA1"]}
                }]
            },
            "components": {
                "H1": {"health": 30000.0, "protection": 4.0,
                       "mobility": {"speed": 32.0, "turningRadius": 660.0, "rudderTime": 9.0},
                       "visibility": {"sea": 11.5, "plane": 6.0}},
                "G1": {"range": 14699.0, "sigma": 2.0, "guns": [{
                    "reload": 15.0, "rotation": 25.7, "each": 3,
                    "ammo": ["PAPA_AP"], "vertSector": 41.0, "count": 3}]},
                "AA1": {"near": [{"minRange": 0.1, "maxRange": 2.0, "hitChance": 0.9,
                                  "damage": 60.0, "rof": 0.29, "dps": 200.0, "guns": []}]}
            }
        });
        let ship = super::super::gamedata::ShipInfo {
            modules: json.get("modules").cloned().unwrap_or_default(),
            components: json.get("components").cloned().unwrap_or_default(),
            ..Default::default()
        };
        let build = super::super::ship_builder::build_ship_build(
            &ship,
            super::super::ship_builder::ModuleSelection::default(),
        );
        let mods = parse_modifiers(&json!({
            "GMShotDelay": 0.9, "speedCoef": 1.05, "visibilityFactor": 0.9
        }));
        let adjusted = apply_modifiers(&build, "Cruiser", &mods, 1.0);
        assert!((adjusted.gun_reload_s - 13.5).abs() < 1e-9);
        assert!((adjusted.speed - 33.6).abs() < 1e-9);
        assert!((adjusted.concealment_sea - 10.35).abs() < 1e-9);
        assert!((adjusted.aa_dps - 200.0).abs() < 1e-9);
    }
}
