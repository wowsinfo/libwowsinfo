//! Ranked-season parsing (`/wows/seasons/accountinfo/` and `shipstats/`).

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::guard;
use crate::models::{RankPlayerInfo, RankSeason, RankShipStat};

/// Parse `/wows/seasons/accountinfo/` into typed rank data.
#[must_use]
pub fn parse_rank_info(json: &Value, account_id: u64) -> Option<RankPlayerInfo> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let account = data.get(account_id.to_string())?;
    let seasons = account.get("seasons")?.as_object()?;
    let mut parsed = HashMap::new();
    for (id, season) in seasons {
        if let Ok(season) = serde_json::from_value::<RankSeason>(season.clone()) {
            parsed.insert(id.clone(), season);
        }
    }
    Some(RankPlayerInfo { account_id, seasons: parsed })
}

/// Parse `/wows/seasons/shipstats/` into per-ship ranked stats. The API
/// returns the ships as either a map (ship_id -> entry) or a list of entries.
#[must_use]
pub fn parse_rank_ship_stats(json: &Value, account_id: u64) -> Vec<RankShipStat> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let Some(account) = data.get(account_id.to_string()) else {
        return Vec::new();
    };
    let entries: Vec<&Value> = match account {
        Value::Array(list) => list.iter().collect(),
        Value::Object(map) => map.values().collect(),
        _ => return Vec::new(),
    };
    entries
        .into_iter()
        .filter_map(|entry| serde_json::from_value::<RankShipStat>(entry.clone()).ok())
        .collect()
}
