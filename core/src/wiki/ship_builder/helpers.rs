//! Module-tree parsing helpers.

use std::collections::HashMap;

use serde_json::Value;

use super::types::ModuleOption;
use crate::wiki::gamedata::ShipInfo;

pub(super) fn as_f64(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

pub(super) fn as_i64(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}

pub(super) fn as_str(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

pub(super) fn first_component<'a>(
    components: &'a HashMap<String, Vec<String>>,
    key: &str,
) -> Option<&'a String> {
    components.get(key).and_then(|ids| ids.first())
}

/// Parse a `[min, max]` pair into `(min, max)` (e.g. `horizSector`).
pub(super) fn sector(json: &Value, key: &str) -> (f64, f64) {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            (
                arr.first().and_then(Value::as_f64).unwrap_or(0.0),
                arr.get(1).and_then(Value::as_f64).unwrap_or(0.0),
            )
        })
        .unwrap_or((0.0, 0.0))
}

pub(super) fn parse_module_options(ship: &ShipInfo, slot_key: &str) -> Vec<ModuleOption> {
    ship.modules
        .get(slot_key)
        .and_then(Value::as_array)
        .map(|options| {
            options
                .iter()
                .map(|option| {
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
                    ModuleOption {
                        index: as_i64(option, "index"),
                        name: as_str(option, "name"),
                        cost_xp: as_i64(cost, "costXP"),
                        cost_cr: as_i64(cost, "costCR"),
                        components,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn component<'a>(ship: &'a ShipInfo, id: &str) -> Option<&'a Value> {
    ship.components.get(id)
}
