use super::*;

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
fn pr_cleanup_drops_empty_arrays() {
    let json = serde_json::json!({
        "data": {
            "1": {"average_damage_dealt": 1.0, "average_frags": 0.5, "win_rate": 50.0},
            "2": []
        }
    });
    let parsed = parse_pr(&json);
    assert_eq!(parsed.len(), 1);
    assert!(parsed.contains_key(&1));
    assert!(!parsed.contains_key(&2));
}

#[test]
fn local_pr_has_data() {
    let pr = local_pr();
    assert!(
        pr.len() > 10,
        "bundled personal_rating.json should be usable"
    );
    let entry = pr.get(&3542005744).expect("known ship");
    assert!(entry.average_damage_dealt > 0.0);
}

#[test]
fn search_results_parse() {
    let json = serde_json::json!({
        "status": "ok",
        "data": [{"account_id": 123, "nickname": "HenryQuan"}, {"account_id": 456, "nickname": "x"}]
    });
    let results = parse_search_results(&json);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].nickname, "HenryQuan");
}

#[test]
fn player_info_parse_handles_keyed_data() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {"account_id": 42, "nickname": "Bob", "hidden_profile": true}}
    });
    let info = parse_player_info(&json, 42).expect("player");
    assert_eq!(info.nickname, "Bob");
    assert_eq!(info.hidden_profile, Some(true));
}

#[test]
fn ship_stats_parse_skips_pvp_rollup() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "1": {"ship_id": 1, "battles": 10, "wins": 5, "damage_dealt": 100, "frags": 2,
                  "pvp": {"battles": 10, "wins": 5, "damage_dealt": 100, "frags": 2}},
            "pvp": {"battles": 10}
        }}
    });
    let ships = parse_ship_stats(&json, 42);
    assert_eq!(ships.len(), 1);
    assert_eq!(ships[0].ship_id, 1);
}

#[test]
fn ship_stats_parse_handles_array_format() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": [
            {"account_id": 42, "ship_id": 1, "battles": 10,
             "pvp": {"battles": 10, "wins": 5, "damage_dealt": 100, "frags": 2}},
            {"account_id": 42, "ship_id": 2, "battles": 0, "pvp": null}
        ]}
    });
    let ships = parse_ship_stats(&json, 42);
    assert_eq!(ships.len(), 2);
    assert_eq!(ships[0].ship_id, 1);
    assert_eq!(ships[1].ship_id, 2);
}

#[test]
fn version_check_is_simple_inequality() {
    assert!(check_version_update("12.7.0.0", "12.8.0.0"));
    assert!(!check_version_update("12.7.0.0", "12.7.0.0"));
}

#[test]
fn assemble_player_builds_view() {
    let pr = local_pr();
    let ships = vec![ShipStats {
        ship_id: 3542005744,
        battles: 100,
        wins: 50,
        damage_dealt: 5_000_000,
        frags: 80,
        pvp: Some(crate::models::PvpStats {
            battles: 100,
            wins: 50,
            damage_dealt: 5_000_000,
            frags: 80,
            ..Default::default()
        }),
        ..Default::default()
    }];

    let view = assemble_player(
        PlayerInfo {
            account_id: 1,
            nickname: "HenryQuan".to_string(),
            ..Default::default()
        },
        ships,
            &pr,
            &HashMap::new(),
            crate::data::Server::Asia,
            String::new(),
            Vec::new(),
        );
    assert_eq!(view.nickname, "HenryQuan");
    assert_eq!(view.server, "asia");
    assert_eq!(view.ships.len(), 1);
    assert!(view.ships[0].rating > 0.0);
    assert!(view.rating > 0.0);
    assert!(view.ap > 0.0, "player AP derives from rating and total battles");
}

#[test]
fn parse_achievements_sorts_by_count() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {"battle": {"2": 1, "1": 3, "3": 2}}}
    });
    let achievements = parse_achievements(&json, 42, &HashMap::new());
    let ids: Vec<&str> = achievements.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, vec!["1", "3", "2"]);
    assert_eq!(achievements[0].count, 3);
}

#[test]
fn achievements_wiki_parses_and_enriches() {
    let wiki_json = serde_json::json!({
        "status": "ok",
        "data": {"battle": {
            "1": {"achievement_id": "1", "name": "First Blood", "image": "http://example.com/1.png"},
            "2": {"achievement_id": "2", "name": "Dreadnought", "image": "http://example.com/2.png"}
        }}
    });
    let wiki = parse_achievements_wiki(&wiki_json);
    assert_eq!(wiki.len(), 2);
    assert_eq!(wiki.get("1").map(|e| e.name.as_str()), Some("First Blood"));

    let battle_json = serde_json::json!({
        "status": "ok",
        "data": {"42": {"battle": {"2": 1, "1": 3}}}
    });
    let achievements = parse_achievements(&battle_json, 42, &wiki);
    assert_eq!(achievements[0].name, "First Blood");
    assert_eq!(achievements[0].icon, "http://example.com/1.png");
    assert_eq!(achievements[1].name, "Dreadnought");
}

#[test]
fn clan_tag_extracts_tag() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {"clan": {"tag": "ABC"}}}
    });
    assert_eq!(parse_clan_tag(&json, 42), "ABC");
    assert_eq!(parse_clan_tag(&json, 99), "");
}

#[test]
fn statistics_parses_all_modes() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "account_id": 42,
            "nickname": "Bob",
            "statistics": {
                "pvp": {"battles": 10},
                "pvp_solo": {"battles": 5},
                "pvp_div2": {"battles": 3},
                "pvp_div3": {"battles": 2},
                "pve": {"battles": 7},
                "rank_solo": {"battles": 4}
            }
        }}
    });
    let player = parse_player_info(&json, 42).expect("player");
    let stats = player.statistics.expect("statistics");
    assert_eq!(stats.pvp.as_ref().map(|p| p.battles), Some(10));
    assert_eq!(stats.solo.as_ref().map(|p| p.battles), Some(5));
    assert_eq!(stats.div2.as_ref().map(|p| p.battles), Some(3));
    assert_eq!(stats.div3.as_ref().map(|p| p.battles), Some(2));
    assert_eq!(stats.pve.as_ref().map(|p| p.battles), Some(7));
    assert_eq!(stats.rank_solo.as_ref().map(|p| p.battles), Some(4));
}
