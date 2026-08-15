//! Ship-stat processing.

use serde_json::{Map, Value};

use super::guard::guard;
use crate::models::ShipStats;


/// Parse `/wows/ships/stats/` into the ship list for one account.
///
/// The API returns `data.<account_id>` either as an object keyed by ship id
/// (older responses) or as an array of per-ship entries (current format).
#[must_use]
pub fn parse_ship_stats(json: &Value, account_id: u64) -> Vec<ShipStats> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let Some(account) = data.get(account_id.to_string()) else {
        return vec![];
    };

    let mut ships = Vec::new();
    if let Some(entries) = account.as_array() {
        for entry in entries {
            if let Ok(mut stats) = serde_json::from_value::<ShipStats>(entry.clone()) {
                stats.ship_id = entry
                    .get("ship_id")
                    .and_then(Value::as_u64)
                    .unwrap_or_default();
                if stats.ship_id != 0 {
                    ships.push(stats);
                }
            }
        }
    } else if let Some(map) = account.as_object() {
        for (key, value) in map {
            if key == "pvp" {
                continue;
            }
            if let Ok(mut stats) = serde_json::from_value::<ShipStats>(value.clone()) {
                stats.ship_id = value
                    .get("ship_id")
                    .and_then(Value::as_u64)
                    .or_else(|| key.parse().ok())
                    .unwrap_or_default();
                if stats.ship_id != 0 {
                    ships.push(stats);
                }
            }
        }
    }
    ships
}
