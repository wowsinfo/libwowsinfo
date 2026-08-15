//! Rank tests.

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::super::*;

use crate::models::{PlayerInfo, ShipStats};

#[test]
fn rank_info_parses_seasons() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "account_id": 42,
            "seasons": {
                "24": {
                    "rank_info": {"max_rank": 23, "start_rank": 15, "stars": 5, "rank": 23, "stage": 4},
                    "rank_solo": {"battles": 100, "wins": 60, "damage_dealt": 5_000_000, "frags": 80},
                    "rank_div2": {"battles": 10, "wins": 5}
                },
                "23": {}
            }
        }}
    });
    let rank = parse_rank_info(&json, 42).expect("rank info");
    assert_eq!(rank.account_id, 42);
    let season = rank.seasons.get("24").expect("season 24");
    let info = season.rank_info.as_ref().expect("rank info");
    assert_eq!(info.max_rank, 23);
    assert_eq!(info.stars, 5);
    assert_eq!(info.stage, 4);
    let solo = season.rank_solo.as_ref().expect("rank solo");
    assert_eq!(solo.battles, 100);
    assert_eq!(solo.wins, 60);
    assert!(season.rank_div2.is_some());
    assert!(season.rank_div3.is_none());
    assert!(rank
        .seasons
        .get("23")
        .is_some_and(|s| s.rank_solo.is_none()));
    assert!(parse_rank_info(&json, 1).is_none());
}

#[test]
fn rank_ship_stats_parse_map_and_list() {
    let map_json = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "3542005744": {
                "ship_id": 3542005744u64,
                "seasons": {"24": {"rank_solo": {"battles": 50, "wins": 30}}}
            }
        }}
    });
    let list_json = serde_json::json!({
        "status": "ok",
        "data": {"42": [
            {"ship_id": 3542005744u64, "seasons": {"24": {"rank_solo": {"battles": 50}}}}
        ]}
    });
    let from_map = parse_rank_ship_stats(&map_json, 42);
    let from_list = parse_rank_ship_stats(&list_json, 42);
    assert_eq!(from_map.len(), 1);
    assert_eq!(from_map[0].ship_id, 3542005744u64);
    assert_eq!(
        from_map[0]
            .seasons
            .get("24")
            .and_then(|s| s.rank_solo.as_ref())
            .map(|p| p.battles),
        Some(50)
    );
    assert_eq!(from_list.len(), 1);
    assert!(parse_rank_ship_stats(&map_json, 1).is_empty());
}
