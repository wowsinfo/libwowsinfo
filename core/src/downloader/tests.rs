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
                 "pvp": {"battles": 10, "wins": 5, "damage_dealt": 100, "frags": 2},
                 "pvp_solo": {"battles": 4, "wins": 2, "damage_dealt": 40, "frags": 1},
                 "pve": {"battles": 2, "wins": 2, "damage_dealt": 20, "frags": 0}},
                {"account_id": 42, "ship_id": 2, "battles": 0, "pvp": null}
            ]}
        });
        let ships = parse_ship_stats(&json, 42);
        assert_eq!(ships.len(), 2);
        assert_eq!(ships[0].ship_id, 1);
        assert_eq!(ships[0].solo.as_ref().map(|p| p.battles), Some(4));
        assert_eq!(ships[0].pve.as_ref().map(|p| p.battles), Some(2));
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
        None,
        None,
        Vec::new(),
        None,
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
fn recent_dates_cover_last_ten_days() {
    // Epoch 0 -> 1970-01-01, so day -1 is 1969-12-31 and the list descends.
    let dates = recent_dates(0);
    let parts: Vec<&str> = dates.split(',').collect();
    assert_eq!(parts.len(), 10);
    assert_eq!(parts[0], "19691231");
    assert!(parts.iter().all(|d| d.len() == 8 && d.chars().all(|c| c.is_ascii_digit())));
}

#[test]
fn recent_overview_derives_daily_deltas() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "20260801": {"pvp": {"battles": 10, "wins": 5, "damage_dealt": 100000}},
            "20260802": {"pvp": {"battles": 14, "wins": 7, "damage_dealt": 150000}}
        }}
    });
    let overview = parse_recent_overview(&json, 42).expect("overview");
    assert_eq!(overview.days.len(), 1);
    assert_eq!(overview.days[0].battles, 4);
    assert_eq!(overview.days[0].winrate, 50.0);
    assert_eq!(overview.days[0].avg_damage, 12_500.0);
    assert_eq!(overview.total_battles, 4);
    assert_eq!(overview.avg_winrate, 50.0);
    assert_eq!(overview.avg_damage, 12_500.0);
}

#[test]
fn statistics_parses_all_modes() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "account_id": 42,
            "nickname": "Bob",
            "statistics": {
                "battles": 31,
                "distance": 12345,
                "pvp": {
                    "battles": 10,
                    "main_battery": {"shots": 100, "hits": 40, "frags": 5, "max_frags_battle": 2}
                },
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
        assert_eq!(stats.battles, 31);
        assert_eq!(stats.distance, 12345);
        assert_eq!(stats.pvp.as_ref().map(|p| p.battles), Some(10));
        let main_battery = stats.pvp.as_ref().unwrap().main_battery.as_ref().unwrap();
        assert_eq!(main_battery.shots, 100);
        assert_eq!(main_battery.hits, 40);
    assert_eq!(stats.solo.as_ref().map(|p| p.battles), Some(5));
    assert_eq!(stats.div2.as_ref().map(|p| p.battles), Some(3));
    assert_eq!(stats.div3.as_ref().map(|p| p.battles), Some(2));
    assert_eq!(stats.pve.as_ref().map(|p| p.battles), Some(7));
    assert_eq!(stats.rank_solo.as_ref().map(|p| p.battles), Some(4));
}

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

#[test]
fn wiki_pages_parse_and_key_by_id() {
    let collections = serde_json::json!({
        "status": "ok",
        "data": {"1": {"collection_id": 1, "name": "C1", "description": "d", "image": "i"}}
    });
    let cards = serde_json::json!({
        "status": "ok",
        "data": {"7": {
            "card_id": 7,
            "collection_id": 1,
            "name": "card",
            "description": "d",
            "images": {"small": "s"}
        }}
    });
    let consumables = serde_json::json!({
        "status": "ok",
        "data": {"3": {
            "consumable_id": 3,
            "name": "Repair",
            "description": "fix",
            "image": "i",
            "type": "repair",
            "price_credit": 100,
            "price_gold": 0,
            "profile": {"p1": {"description": "heals"}}
        }}
    });
    let skills = serde_json::json!({
        "status": "ok",
        "data": {"5": {
            "name": "Expert",
            "icon": "ic",
            "tier": 3,
            "type_id": 1,
            "type_name": "skill",
            "perks": [{"perk_id": 9, "description": "bonus"}]
        }}
    });
    let cols = parse_collections(&collections);
    assert_eq!(cols.get(&1).map(|c| c.name.as_str()), Some("C1"));
    let card_list = parse_collection_cards(&cards);
    assert_eq!(card_list.get(&7).map(|c| c.collection_id), Some(1));
    assert_eq!(card_list.get(&7).map(|c| c.image.as_str()), Some("s"));
    let cons = parse_consumables(&consumables);
    let c = cons.get(&3).expect("consumable");
    assert_eq!(c.name, "Repair");
    assert_eq!(c.price_credit, 100);
    assert_eq!(c.profile.len(), 1);
    assert_eq!(c.profile[0].description, "heals");
    let sk = parse_commander_skills(&skills);
    let s = sk.get(&5).expect("skill");
    assert_eq!(s.skill_id, 5);
    assert_eq!(s.tier, 3);
    assert_eq!(s.perks.len(), 1);
    assert_eq!(s.perks[0].perk_id, 9);
}
