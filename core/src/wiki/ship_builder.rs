//! Ship builder: module tree resolution and per-selection stats.
//!
//! The game data stores every ship as a module tree (`ships.<id>.modules`),
//! where each module option references component ids (`ships.<id>.components`).
//! This is a Rust port of the Flutter two app's `ShipModules` model, following
//! the wows-toolkit convention of one typed struct per component.

use std::collections::HashMap;

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::components::{
    parse_band, parse_guns, parse_hull, AirDefenseStats, AirSupportStats, DepthChargeStats,
    EngineStats, FireControlStats, GunStats, HullStats, PingerStats, SpecialStats,
    TorpedoStats,
};
use super::gamedata::ShipInfo;

/// One module option (`modules.<slot>[i]`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ModuleOption {
    pub index: i64,
    /// Localisation key of the module name (`IDS_...`).
    pub name: String,
    pub cost_xp: i64,
    pub cost_cr: i64,
    /// Slot key -> component ids (e.g. `artillery -> ["A1_203_55"]`).
    pub components: HashMap<String, Vec<String>>,
}

/// The computed stats for the currently selected modules.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ShipBuild {
    pub hull: Option<HullStats>,
    pub main_battery: Option<GunStats>,
    pub secondaries: Option<GunStats>,
    pub torpedoes: Option<TorpedoStats>,
    pub air_defense: Option<AirDefenseStats>,
    pub fire_control: Option<FireControlStats>,
    pub engine: Option<EngineStats>,
    pub depth_charges: Option<DepthChargeStats>,
    pub air_support: Option<AirSupportStats>,
    pub pinger: Option<PingerStats>,
    pub special: Option<SpecialStats>,
}

/// Per-module selection indices (defaults to the stock modules).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModuleSelection {
    pub hull: usize,
    pub artillery: usize,
    pub torpedoes: usize,
    pub fire_control: usize,
    pub engine: usize,
    pub fighter: usize,
    pub torpedo_bomber: usize,
    pub dive_bomber: usize,
    pub skip_bomber: usize,
}

fn as_f64(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn as_i64(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}

fn as_str(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn first_component<'a>(
    components: &'a HashMap<String, Vec<String>>,
    key: &str,
) -> Option<&'a String> {
    components.get(key).and_then(|ids| ids.first())
}

fn parse_module_options(ship: &ShipInfo, slot_key: &str) -> Vec<ModuleOption> {
    ship.modules
        .get(slot_key)
        .and_then(Value::as_array)
        .map(|options| {
            options
                .iter()
                .filter_map(|option| {
                    let components = option
                        .get("components")
                        .and_then(Value::as_object)
                        .map(|map| {
                            map.iter()
                                .map(|(k, v)| {
                                    (
                                        k.clone(),
                                        v.as_array()
                                            .map(|arr| {
                                                arr.iter()
                                                    .filter_map(Value::as_str)
                                                    .map(ToOwned::to_owned)
                                                    .collect()
                                            })
                                            .unwrap_or_default(),
                                    )
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    let cost = option.get("cost").unwrap_or(&Value::Null);
                    Some(ModuleOption {
                        index: as_i64(option, "index"),
                        name: as_str(option, "name"),
                        cost_xp: as_i64(cost, "costXP"),
                        cost_cr: as_i64(cost, "costCR"),
                        components,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn component<'a>(ship: &'a ShipInfo, id: &str) -> Option<&'a Value> {
    ship.components.get(id)
}

/// Resolve the typed stats for the selected modules.
#[must_use]
pub fn build_ship_build(ship: &ShipInfo, selection: ModuleSelection) -> ShipBuild {
    let hull_options = parse_module_options(ship, "_Hull");
    let artillery_options = parse_module_options(ship, "_Artillery");
    let torpedo_options = parse_module_options(ship, "_Torpedoes");
    let fire_control_options = parse_module_options(ship, "_Suo");
    let fire_control_options_cv = parse_module_options(ship, "_FlightControl");
    let engine_options = parse_module_options(ship, "_Engine");

    let hull_option = hull_options.get(selection.hull).cloned();
    let hull_id = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "hull"))
        .cloned();
    let hull = hull_id
        .as_deref()
        .and_then(|id| component(ship, id))
        .map(parse_hull);

    let artillery_ids = if let Some(art) = artillery_options.get(selection.artillery) {
        art.components.get("artillery").cloned()
    } else {
        hull_option
            .as_ref()
            .and_then(|o| o.components.get("artillery").cloned())
    };
    let main_battery = artillery_ids
        .as_ref()
        .and_then(|ids| ids.first())
        .and_then(|id| component(ship, id))
        .map(parse_guns);

    let torpedo_ids = if let Some(t) = torpedo_options.get(selection.torpedoes) {
        t.components.get("torpedoes").cloned()
    } else {
        hull_option
            .as_ref()
            .and_then(|o| o.components.get("torpedoes").cloned())
    };
    let torpedoes = torpedo_ids
        .as_ref()
        .and_then(|ids| ids.first())
        .and_then(|id| component(ship, id))
        .map(|json| TorpedoStats {
            single_shot: json
                .get("singleShot")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            launchers: json
                .get("launchers")
                .and_then(Value::as_array)
                .map(|arr| arr.iter().map(super::components::parse_weapon).collect())
                .unwrap_or_default(),
        });

    let fire_control = fire_control_options
        .get(selection.fire_control)
        .or_else(|| fire_control_options_cv.get(selection.fire_control))
        .and_then(|o| {
            o.components
                .get("fireControl")
                .or_else(|| o.components.get("flightControl"))
        })
        .and_then(|ids| ids.first())
        .and_then(|id| component(ship, id))
        .map(|json| FireControlStats {
            max_dist_coef: as_f64(json, "maxDistCoef"),
            sigma_count_coef: as_f64(json, "sigmaCountCoef"),
        });

    let engine = engine_options
        .get(selection.engine)
        .and_then(|o| first_component(&o.components, "engine"))
        .and_then(|id| component(ship, id))
        .map(|json| EngineStats {
            speed_coef: as_f64(json, "speedCoef"),
        })
        .or_else(|| {
            if engine_options.is_empty() {
                Some(EngineStats {
                    speed_coef: 1.0,
                })
            } else {
                None
            }
        });

    let air_defense = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "airDefense"))
        .and_then(|id| component(ship, id))
        .map(|json| AirDefenseStats {
            near: parse_band(json, "near"),
            medium: parse_band(json, "medium"),
            far: parse_band(json, "far"),
        });

    let depth_charges = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "depthCharges"))
        .and_then(|id| component(ship, id))
        .map(|json| DepthChargeStats {
            reload: as_f64(json, "reload"),
            ammo: as_str(json, "ammo"),
            bombs: as_i64(json, "bombs"),
            groups: as_i64(json, "groups"),
        });

    let air_support = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "airSupport"))
        .and_then(|id| component(ship, id))
        .map(|json| AirSupportStats {
            name: as_str(json, "name"),
            charges_num: as_i64(json, "chargesNum"),
            plane: as_str(json, "plane"),
            reload: as_f64(json, "reload"),
            range: as_f64(json, "range"),
        });

    let pinger = parse_module_options(ship, "_Sonar")
        .first()
        .and_then(|o| first_component(&o.components, "pinger"))
        .or_else(|| {
            hull_option
                .as_ref()
                .and_then(|o| first_component(&o.components, "pinger"))
        })
        .and_then(|id| component(ship, id))
        .map(|json| PingerStats {
            reload: as_f64(json, "reload"),
            range: as_f64(json, "range"),
            life_time1: as_f64(json, "lifeTime1"),
            life_time2: as_f64(json, "lifeTime2"),
            speed: as_f64(json, "speed"),
        });

    let special = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "specials"))
        .and_then(|id| component(ship, id))
        .and_then(|json| json.get("rageMode").filter(|v| !v.is_null()))
        .map(|r| SpecialStats {
            boost_duration: as_f64(r, "boostDuration"),
            decrement_count: as_i64(r, "decrementCount"),
            decrement_delay: as_f64(r, "decrementDelay"),
            decrement_period: as_f64(r, "decrementPeriod"),
            guns_for_salvo: as_i64(r, "gunsForSalvo"),
            radius: as_f64(r, "radius"),
            rage_mode_name: as_str(r, "rageModeName"),
            required_hits: as_i64(r, "requiredHits"),
        });

    let secondaries = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "atba"))
        .and_then(|id| component(ship, id))
        .map(parse_guns);

    ShipBuild {
        hull,
        main_battery,
        secondaries,
        torpedoes,
        air_defense,
        fire_control,
        engine,
        depth_charges,
        air_support,
        pinger,
        special,
    }
}

/// The module slots that can be changed, with their options (for the UI).
#[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wiki::gamedata::GameData;

    #[test]
    fn module_options_parse_and_build() {
        let json = serde_json::json!({
            "modules": {
                "_Hull": [
                    {"index": 0, "name": "IDS_A", "cost": {"costXP": 0, "costCR": 100},
                     "components": {"hull": ["A_Hull"], "artillery": ["A1"]}},
                    {"index": 1, "name": "IDS_B", "cost": {"costXP": 1000, "costCR": 200},
                     "components": {"hull": ["B_Hull"], "artillery": ["B1"]}}
                ]
            },
            "components": {
                "A_Hull": {"health": 100.0, "protection": 4.0, "mobility": {"speed": 30.0},
                           "visibility": {"sea": 10.0}},
                "B_Hull": {"health": 120.0, "protection": 4.0, "mobility": {"speed": 31.0},
                           "visibility": {"sea": 10.0}}
            }
        });
        let ship = ShipInfo {
            modules: json.get("modules").cloned().unwrap_or_default(),
            components: json.get("components").cloned().unwrap_or_default(),
            ..Default::default()
        };
        let options = parse_module_options(&ship, "_Hull");
        assert_eq!(options.len(), 2);
        assert_eq!(options[1].cost_xp, 1000);
        assert_eq!(options[1].components["hull"], vec!["B_Hull".to_string()]);
        let build = build_ship_build(
            &ship,
            ModuleSelection {
                hull: 1,
                ..ModuleSelection::default()
            },
        );
        let hull = build.hull.expect("hull");
        assert_eq!(hull.health, 120.0);
        assert_eq!(hull.mobility.speed, 31.0);
        let _ = GameData::default();
    }
}
