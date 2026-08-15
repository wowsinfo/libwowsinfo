//! Aircraft JSON parsing helpers.

use serde_json::Value;

use super::types::PlaneConsumableSlot;

pub(super) fn f64_field(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

pub(super) fn i64_field(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}

pub(super) fn str_field(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

pub(super) fn opt_f64(json: &Value, key: &str) -> Option<f64> {
    json.get(key)
        .filter(|v| !v.is_null())
        .and_then(Value::as_f64)
}

pub(super) fn opt_i64(json: &Value, key: &str) -> Option<i64> {
    json.get(key)
        .filter(|v| !v.is_null())
        .and_then(Value::as_i64)
}

pub(super) fn speed_limits(json: &Value, key: &str) -> (Option<f64>, Option<f64>) {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            (
                arr.first().and_then(Value::as_f64),
                arr.get(1).and_then(Value::as_f64),
            )
        })
        .unwrap_or((None, None))
}

pub(super) fn plane_consumables(json: &Value) -> Vec<PlaneConsumableSlot> {
    let mut slots: Vec<PlaneConsumableSlot> = json
        .get("planeConsumables")
        .and_then(Value::as_object)
        .map(|map| {
            map.values()
                .filter_map(|v| {
                    let abils = v.get("abils").and_then(Value::as_array)?;
                    Some(PlaneConsumableSlot {
                        slot: v.get("slot").and_then(Value::as_i64).unwrap_or(0),
                        abilities: abils
                            .iter()
                            .filter_map(|entry| {
                                entry
                                    .as_array()
                                    .and_then(|pair| pair.first())
                                    .and_then(Value::as_str)
                                    .map(ToOwned::to_owned)
                            })
                            .collect(),
                        special: v
                            .get("isSpecial")
                            .and_then(Value::as_bool)
                            .unwrap_or(false),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    slots.sort_by_key(|slot| slot.slot);
    slots
}
