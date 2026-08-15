//! Clan tests.

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::super::*;

use crate::models::{PlayerInfo, ShipStats};

#[test]
fn clan_id_and_tag_parse_from_accountinfo() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {"clan": {"id": 99, "tag": "ABC"}}}
    });
    assert_eq!(parse_clan_id(&json, 42), Some(99));
    assert_eq!(parse_clan_tag(&json, 42), "ABC");
    assert_eq!(parse_clan_id(&json, 1), None);
}

#[test]
fn clan_info_parses_members() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"99": {
            "clan_id": 99,
            "tag": "ABC",
            "name": "Alpha",
            "description": "desc",
            "members_count": 2,
            "created_at": 123,
            "updated_at": 456,
            "leader_id": 1,
            "leader_name": "Lead",
            "creator_id": 1,
            "creator_name": "Lead",
            "old_name": "",
            "old_tag": "",
            "renamed_at": 0,
            "is_clan_disbanded": false,
            "members": {
                "1": {"account_name": "Lead", "role": "commander", "joined_at": 100},
                "2": {"account_name": "Dude", "role": "executive_officer", "joined_at": 200}
            }
        }}
    });
    let clan = parse_clan_info(&json, 99).expect("clan info");
    assert_eq!(clan.tag, "ABC");
    assert_eq!(clan.name, "Alpha");
    assert_eq!(clan.members_count, 2);
    assert_eq!(clan.members.len(), 2);
    let leader = clan
        .members
        .iter()
        .find(|m| m.account_id == 1)
        .expect("leader");
    assert_eq!(leader.role, "commander");
    assert_eq!(leader.account_name, "Lead");
    assert!(parse_clan_info(&json, 1).is_none());
}

#[test]
fn clan_search_parses_results() {
    let json = serde_json::json!({
        "status": "ok",
        "data": [{"clan_id": 1, "tag": "ABC"}, {"clan_id": 2, "tag": "XYZ"}]
    });
    let results = parse_clan_search(&json);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].clan_id, 1);
    assert_eq!(results[1].tag, "XYZ");
    assert!(parse_clan_search(&serde_json::json!({"data": {}})).is_empty());
}
