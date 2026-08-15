//! Projectile JSON parsing helpers.

use serde_json::Value;

pub(super) fn f64_field(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
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

pub(super) fn str_field(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

/// Parse a list of `[number, number]` pairs (e.g. `pointsOfDamage`).
pub(super) fn f64_pairs(json: &Value, key: &str) -> Vec<(f64, f64)> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    let pair = v.as_array()?;
                    let first = pair.first()?.as_f64()?;
                    let second = pair.get(1)?.as_f64()?;
                    Some((first, second))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Parse a string list (e.g. `ignoreClasses`).
pub(super) fn str_list(json: &Value, key: &str) -> Vec<String> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

/// Parse a `{key: number}` map into sorted pairs (e.g. `buoyancyToDamageCoeff`).
pub(super) fn str_f64_map(json: &Value, key: &str) -> Vec<(String, f64)> {
    json.get(key)
        .and_then(Value::as_object)
        .map(|map| {
            let mut pairs: Vec<(String, f64)> = map
                .iter()
                .filter_map(|(k, v)| v.as_f64().map(|value| (k.clone(), value)))
                .collect();
            pairs.sort_by(|a, b| a.0.cmp(&b.0));
            pairs
        })
        .unwrap_or_default()
}

/// Parse a list of numbers (e.g. `distParams`).
pub(super) fn f64_list(json: &Value, key: &str) -> Vec<f64> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_f64).collect())
        .unwrap_or_default()
}
