//! Search / warship-entry processing.

use serde_json::Value;

use super::guard::guard;
use crate::models::{AccountListEntry, EncyclopediaShip, RawEncyclopediaShip};


/// `getWarship` post-processing for a single raw ship entry.
#[must_use]
pub fn process_warship_entry(raw: RawEncyclopediaShip, is_new_launch: bool) -> EncyclopediaShip {
    let mut ship: EncyclopediaShip = raw.into();
    if is_new_launch {
        ship.new = Some(true);
    }
    ship
}

/// Parse `/wows/account/list/` results.
#[must_use]
pub fn parse_search_results(json: &Value) -> Vec<AccountListEntry> {
    let empty = Value::Array(vec![]);
    let data = guard(json, "data", &empty);
    let Some(data) = data.as_array() else {
        return vec![];
    };
    data.iter()
        .filter_map(|v| serde_json::from_value::<AccountListEntry>(v.clone()).ok())
        .collect()
}
