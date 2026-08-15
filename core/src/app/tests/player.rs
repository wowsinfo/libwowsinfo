//! Player-flow tests.

use super::*;

#[test]
fn select_player_assembles_stats() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    model.pr = downloader::local_pr();

    let mut update = app.update(Event::SelectPlayer { account_id: 42 }, &mut model);
    assert!(matches!(model.phase, Phase::LoadingPlayer));
    assert_eq!(update.effects.len(), 9, "player + ships + warship + rank + rank ships + achievements + clan + recent + render");

    let player_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "account_id": 42,
            "nickname": "Bob",
            "hidden_profile": false,
            "statistics": {
                "pvp": {"battles": 100, "wins": 50, "damage_dealt": 5_000_000, "frags": 80}
            }
        }}
    });
    let ships_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "3542005744": {
                "ship_id": 3542005744u64,
                "battles": 100,
                "wins": 50,
                "damage_dealt": 5_000_000,
                "frags": 80,
                "pvp": {"battles": 100, "wins": 50, "damage_dealt": 5_000_000, "frags": 80}
            }
        }}
    });
    let warship_body = serde_json::json!({
        "status": "ok",
        "meta": {"page_total": 1},
        "data": {
            "3542005744": {
                "ship_id": 3542005744u64,
                "name": "Hermelin",
                "nation": "pan_europe",
                "type": "dd",
                "tier": 1,
                "images": {"small": "http://example.com/hermelin.png"},
                "is_premium": false,
                "is_special": false
            }
        }
    });
    let achievements_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {"battle": {"1": 3, "2": 1}}}
    });
    let clan_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {"clan": {"id": 99, "tag": "ABC"}}}
    });
    let clan_info_body = serde_json::json!({
        "status": "ok",
        "data": {"99": {
            "clan_id": 99,
            "tag": "ABC",
            "name": "Alpha",
            "members_count": 1,
            "members": {
                "7": {"account_name": "Bob", "role": "commander", "joined_at": 100}
            }
        }}
    });
    let rank_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "account_id": 42,
            "seasons": {"24": {
                "-1": {
                    "rank_solo": {"battles": 50, "wins": 30},
                    "rank_div2": null,
                    "rank_div3": null
                }
            }},
            "rank_info": {
                "24": {"3": {"3": {"rank": 23, "rank_best": 23, "stars": 5, "stage": 4, "sprint_number": 3}}}
            }
        }}
    });
    let rank_ships_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "3542005744": {
                "ship_id": 3542005744u64,
                "seasons": {"24": {"-1": {"rank_solo": {"battles": 10, "wins": 6}}}}
            }
        }}
    });
    let recent_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {
            "20260801": {"pvp": {"battles": 10, "wins": 5, "damage_dealt": 100000}},
            "20260802": {"pvp": {"battles": 14, "wins": 7, "damage_dealt": 150000}}
        }}
    });

    // Resolve every pending HTTP request against its endpoint.
    let requests: Vec<&mut crux_core::Request<crux_http::HttpRequest>> = update
        .effects_mut()
        .filter_map(|effect| match effect {
            Effect::Http(request) => Some(request),
            _ => None,
        })
        .collect();
    let mut events = Vec::new();
    for request in requests {
        let url = request.operation.url.clone();
        let body = if url.contains("/wows/account/info/") {
            player_body.clone()
        } else if url.contains("/wows/ships/stats/") {
            ships_body.clone()
        } else if url.contains("/wows/account/achievements/") {
            achievements_body.clone()
        } else if url.contains("/wows/clans/accountinfo/") {
            clan_body.clone()
        } else if url.contains("/wows/clans/info/") {
            clan_info_body.clone()
        } else if url.contains("/wows/seasons/accountinfo/") {
            rank_body.clone()
        } else if url.contains("/wows/seasons/shipstats/") {
            rank_ships_body.clone()
        } else if url.contains("/wows/account/statsbydate/") {
            recent_body.clone()
        } else {
            warship_body.clone()
        };
        let update = app.resolve(request, http_ok(body)).expect("resolve http");
        events.push(update.events.into_iter().next().expect("one event"));
    }
    for event in events {
        let mut update = app.update(event, &mut model);
        // `ClanLoaded` chains a `clans/info` request once the clan id is known.
        for request in update.effects_mut().filter_map(|effect| match effect {
            Effect::Http(request) if request.operation.url.contains("clans/info/") => Some(request),
            _ => None,
        }) {
            let update = app
                .resolve(request, http_ok(clan_info_body.clone()))
                .expect("resolve clan info");
            let _ = app.update(
                update.events.into_iter().next().expect("one event"),
                &mut model,
            );
        }
    }

    assert!(matches!(model.phase, Phase::Player));
    let player = app.view(&model).player.expect("player view");
    assert_eq!(player.nickname, "Bob");
        assert!(model.warship.contains_key(&3542005744));
        assert_eq!(player.ships.len(), 1);
        assert_eq!(player.ships[0].name, "Hermelin");
        assert!(player.rating > 0.0);
        assert_eq!(player.statistics.pvp.as_ref().map(|p| p.battles), Some(100));
        assert_eq!(player.achievements.len(), 2);
        assert_eq!(player.clan_tag, "ABC");
        let clan = player.clan.as_ref().expect("clan info");
        assert_eq!(clan.name, "Alpha");
        assert_eq!(clan.members.len(), 1);
        let rank = player.rank.as_ref().expect("rank info");
        assert!(rank.seasons.contains_key("24"));
        assert_eq!(
            rank.seasons["24"]
                .ranks
                .get("-1")
                .and_then(|m| m.rank_solo.as_ref())
                .map(|p| p.battles),
            Some(50)
        );
        assert_eq!(player.rank_ships.len(), 1);
        assert_eq!(player.rank_ships[0].ship_id, 3542005744u64);
        assert_eq!(player.created_at, None);
        let recent = player.recent.expect("recent overview");
        assert_eq!(recent.total_battles, 4);
        assert_eq!(recent.days.len(), 1);
}

#[test]
fn refresh_reloads_pr_and_reloads_player() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    model.pr = downloader::local_pr();
    model.warship.insert(
        3542005744,
        models::EncyclopediaShip {
            ship_id: 3542005744,
            name: "Hermelin".to_string(),
            ..Default::default()
        },
    );
    model.pending_account_id = Some(42);

    let update = app.update(Event::Refresh, &mut model);
    // PR + time + player + ships + rank + rank ships + achievements + clan + recent + render
    assert_eq!(update.effects.len(), 11);
    assert!(update.effects.iter().any(|e| matches!(e, Effect::Time(_))));
    assert!(update.effects.iter().any(|e| matches!(e, Effect::Http(_))));
}
