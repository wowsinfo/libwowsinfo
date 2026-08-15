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
                    "-1": {
                        "rank_solo": {"battles": 100, "wins": 60, "damage_dealt": 5_000_000, "frags": 80},
                        "rank_div2": {"battles": 10, "wins": 5},
                        "rank_div3": null
                    },
                    "3": {"rank_solo": {"battles": 20, "wins": 10}}
                },
                "23": {}
            },
            "rank_info": {
                "24": {
                    "2": {"3": {"rank": 23, "rank_best": 23, "stars": 5, "stage": 4, "sprint_number": 2}},
                    "3": {"3": {"rank": 23, "rank_best": 23, "stars": 5, "stage": 4, "sprint_number": 3}}
                }
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
    let solo = season
        .ranks
        .get("-1")
        .and_then(|mode| mode.rank_solo.as_ref())
        .expect("rank solo");
    assert_eq!(solo.battles, 100);
    assert_eq!(solo.wins, 60);
    assert!(season.ranks.get("-1").is_some_and(|m| m.rank_div2.is_some()));
    assert!(season.ranks.get("-1").is_some_and(|m| m.rank_div3.is_none()));
    assert_eq!(season.ranks.len(), 2);
    assert!(rank.seasons.get("23").is_some_and(|s| s.ranks.is_empty()));
    assert!(parse_rank_info(&json, 1).is_none());
}

#[test]
fn rank_ship_stats_parse_map_and_list() {
    let map_json = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "3542005744": {
                "ship_id": 3542005744u64,
                "seasons": {"24": {"-1": {"rank_solo": {"battles": 50, "wins": 30}}}}
            }
        }}
    });
    let list_json = serde_json::json!({
        "status": "ok",
        "data": {"42": [
            {"ship_id": 3542005744u64, "seasons": {"24": {"-1": {"rank_solo": {"battles": 50}}}}}
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
            .and_then(|s| s.ranks.get("-1"))
            .and_then(|m| m.rank_solo.as_ref())
            .map(|p| p.battles),
        Some(50)
    );
    assert_eq!(from_list.len(), 1);
    assert!(parse_rank_ship_stats(&map_json, 1).is_empty());
}
