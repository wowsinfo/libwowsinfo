//! Personal-rating processing.

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::guard::guard;
use crate::models::PrEntry;


/// Parse a PR table (`{data: {<ship_id>: {...}|[]}}`) into typed entries.
#[must_use]
pub fn parse_pr(json: &Value) -> HashMap<u64, PrEntry> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    // The cached form (`SafeStorage` under `SAVED.pr`) is the inner map
    // without the `data` envelope, so fall back to the whole object.
    let data = if data
        .as_object()
        .is_some_and(|map| map.is_empty() && !json.get("data").is_some())
    {
        json
    } else {
        data
    };
    let Some(data) = data.as_object() else {
        return HashMap::new();
    };
    data.iter()
        .filter_map(|(id, v)| {
            let Ok(ship_id) = id.parse::<u64>() else {
                return None;
            };
            // Empty entries (`[]`) are dropped like `getPR`/`readLocalPR` do.
            if v.is_array() {
                return None;
            }
            let Ok(entry) = serde_json::from_value::<PrEntry>(v.clone()) else {
                return None;
            };
            Some((ship_id, entry))
        })
        .collect()
}

/// `readLocalPR` in `Downloader.ts`: parse the bundled rating table.
#[must_use]
pub fn local_pr() -> HashMap<u64, PrEntry> {
    let json: Value =
        serde_json::from_str(include_str!("../../assets/personal_rating.json")).unwrap_or_default();
    parse_pr(&json)
}
