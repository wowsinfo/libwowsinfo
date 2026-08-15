//! Guard / online-count tests.

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::super::*;

use crate::models::{PlayerInfo, ShipStats};

#[test]
fn guard_walks_paths_and_falls_back() {
    let json = serde_json::json!({"data": {"battle": {"wins": 5}}});
    let default = Value::Bool(false);
    assert_eq!(guard(&json, "data.battle.wins", &default), &Value::from(5));
    assert_eq!(
        guard(&json, "data.battle.losses", &default),
        &Value::Bool(false)
    );
    assert_eq!(guard(&json, "data.nope", &default), &Value::Bool(false));
    assert_eq!(guard(&json, "", &default), &json);
    assert_eq!(guard(&json, ".data", &default), &Value::Bool(false));
}

#[test]
fn online_count_reads_first_server() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"wows": [{"players_online": 12345}, {"players_online": 6789}]}
    });
    assert_eq!(parse_online_count(&json), 12345);
    assert_eq!(
        parse_online_count(&serde_json::json!({"data": {"wows": []}})),
        -1
    );
    assert_eq!(parse_online_count(&serde_json::json!({"data": {}})), -1);
}
