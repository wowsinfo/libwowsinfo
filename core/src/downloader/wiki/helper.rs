//! Paginated wiki-page merge helper.

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::super::guard::guard;

/// Merge one paginated wiki page (`data: {<id>: entry}`) into a map keyed by
/// the entry's own id field.
pub(super) fn parse_wiki_map<T: serde::de::DeserializeOwned>(
    json: &Value,
    id_field: &str,
) -> HashMap<u64, T> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let mut out = HashMap::new();
    if let Some(map) = data.as_object() {
        for (key, value) in map {
            let Some(id) = value
                .get(id_field)
                .and_then(Value::as_u64)
                .or_else(|| key.parse().ok())
            else {
                continue;
            };
            if let Ok(item) = serde_json::from_value::<T>(value.clone()) {
                out.insert(id, item);
            }
        }
    }
    out
}
