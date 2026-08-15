//! Game-data JSON parsing helpers.

use serde_json::Value;

pub(super) fn get<'a>(json: &'a Value, key: &str, default: &'a Value) -> &'a Value {
    json.get(key).filter(|v| !v.is_null()).unwrap_or(default)
}

pub(super) fn str_field(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

pub(super) fn i64_field(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}

pub(super) fn u64_list(json: &Value, key: &str) -> Vec<u64> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_u64).collect())
        .unwrap_or_default()
}

pub(super) fn str_list(json: &Value, key: &str) -> Vec<String> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_str).map(ToOwned::to_owned).collect())
        .unwrap_or_default()
}

pub(super) fn i64_list(json: &Value, key: &str) -> Vec<i64> {
    json.get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_i64).collect())
        .unwrap_or_default()
}
