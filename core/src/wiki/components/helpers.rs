//! Shared parsing helpers for ship components.

use serde_json::Value;

pub(super) fn as_f64(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

pub(super) fn as_i64(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}


pub(super) fn depth_value(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

/// Parse a `{key: number}` map into sorted pairs (e.g. `bySubmarineDepth`).
pub(super) fn str_f64_map_sorted(json: &Value, key: &str) -> Vec<(String, f64)> {
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

/// Parse a numeric-keyed `{depth: coefficient}` table into sorted pairs.
pub(super) fn num_key_f64_table(json: &Value, key: &str) -> Vec<(f64, f64)> {
    json.get(key)
        .and_then(Value::as_object)
        .map(|map| {
            let mut pairs: Vec<(f64, f64)> = map
                .iter()
                .filter_map(|(k, v)| {
                    let depth = k.parse::<f64>().ok()?;
                    let coeff = v.as_f64()?;
                    Some((depth, coeff))
                })
                .collect();
            pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
            pairs
        })
        .unwrap_or_default()
}

/// Parse one `guns`/`launchers` entry.
pub(super) fn bool_field(json: &Value, key: &str) -> bool {
    json.get(key).and_then(Value::as_bool).unwrap_or(false)
}
