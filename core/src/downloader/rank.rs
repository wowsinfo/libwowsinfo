//! Ranked-season parsing (`/wows/seasons/accountinfo/` and `shipstats/`).

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::guard::guard;
use crate::models::{RankInfo, RankPlayerInfo, RankSeason, RankSeasonMode, RankShipStat};

/// Parse `/wows/seasons/accountinfo/` into typed rank data.
#[must_use]
pub fn parse_rank_info(json: &Value, account_id: u64) -> Option<RankPlayerInfo> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let account = data.get(account_id.to_string())?;
    let seasons = account.get("seasons")?.as_object()?;
    let rank_infos = account.get("rank_info").and_then(Value::as_object);
    let mut parsed = HashMap::new();
    for (id, season) in seasons {
        let Some(season_map) = season.as_object() else {
            continue;
        };
        let ranks = season_map
            .iter()
            .filter_map(|(rank_key, mode)| {
                serde_json::from_value::<RankSeasonMode>(mode.clone())
                    .ok()
                    .map(|mode| (rank_key.clone(), mode))
            })
            .collect();
        let rank_info = rank_infos
            .and_then(|infos| infos.get(id))
            .and_then(Value::as_object)
            .and_then(best_rank_info);
        parsed.insert(
            id.clone(),
            RankSeason {
                ranks,
                rank_info,
            },
        );
    }
    Some(RankPlayerInfo { account_id, seasons: parsed })
}

/// Pick the sprint with the highest `sprint_number` from the account-level
/// `rank_info.<season_id>` map. Each sprint is itself keyed by division
/// (`rank_info.<season>.<sprint>.<division>`), so the parser descends one
/// extra level before reading `rank`/`stars`/`stage`/`rank_best`.
fn best_rank_info(sprints: &Map<String, Value>) -> Option<RankInfo> {
    let mut best: Option<(i64, RankInfo)> = None;
    for value in sprints.values() {
        let entries: Vec<&Value> = match value {
            Value::Object(map) => map.values().collect(),
            _ => vec![value],
        };
        for entry in entries {
            let sprint = entry
                .get("sprint_number")
                .and_then(Value::as_i64)
                .unwrap_or(0);
            let info = RankInfo {
                rank: entry.get("rank").and_then(Value::as_i64).unwrap_or(0),
                max_rank: entry
                    .get("rank_best")
                    .and_then(Value::as_i64)
                    .unwrap_or(0),
                stars: entry.get("stars").and_then(Value::as_i64).unwrap_or(0),
                stage: entry.get("stage").and_then(Value::as_i64).unwrap_or(0),
                start_rank: 0,
            };
            if best.as_ref().is_none_or(|(best_sprint, _)| sprint > *best_sprint) {
                best = Some((sprint, info));
            }
        }
    }
    best.map(|(_, info)| info)
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
        .filter_map(|entry| parse_rank_ship_entry(entry))
        .collect()
}

fn parse_rank_ship_entry(entry: &Value) -> Option<RankShipStat> {
    let ship_id = entry.get("ship_id").and_then(Value::as_u64)?;
    let seasons = entry
        .get("seasons")
        .and_then(Value::as_object)
        .map(|seasons| {
            seasons
                .iter()
                .filter_map(|(id, season)| {
                    let ranks = season
                        .as_object()?
                        .iter()
                        .filter_map(|(rank_key, mode)| {
                            serde_json::from_value::<RankSeasonMode>(mode.clone())
                                .ok()
                                .map(|mode| (rank_key.clone(), mode))
                        })
                        .collect();
                    Some((
                        id.clone(),
                        RankSeason {
                            ranks,
                            rank_info: None,
                        },
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    Some(RankShipStat { ship_id, seasons })
}
