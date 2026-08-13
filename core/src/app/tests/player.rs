use super::super::*;
use super::*;

#[test]
fn select_player_assembles_stats() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    model.pr = downloader::local_pr();

    let mut update = app.update(Event::SelectPlayer { account_id: 42 }, &mut model);
    assert!(matches!(model.phase, Phase::LoadingPlayer));
    assert_eq!(update.effects.len(), 4, "player + ships + warship + render");

    let player_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {"account_id": 42, "nickname": "Bob", "hidden_profile": false}}
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
        } else {
            warship_body.clone()
        };
        let update = app.resolve(request, http_ok(body)).expect("resolve http");
        events.push(update.events.into_iter().next().expect("one event"));
    }
    for event in events {
        let _ = app.update(event, &mut model);
    }

    assert!(matches!(model.phase, Phase::Player));
    let player = app.view(&model).player.expect("player view");
    assert_eq!(player.nickname, "Bob");
    assert!(model.warship.contains_key(&3542005744));
    assert_eq!(player.ships.len(), 1);
    assert_eq!(player.ships[0].name, "Hermelin");
    assert!(player.rating > 0.0);
}
#[test]
fn warship_download_paginates_until_last_page() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    model.pr = downloader::local_pr();

    let mut update = app.update(Event::SelectPlayer { account_id: 42 }, &mut model);
    assert_eq!(update.effects.len(), 4, "player + ships + warship + render");

    let player_body = serde_json::json!({
        "status": "ok",
        "data": {"42": {"account_id": 42, "nickname": "Bob", "hidden_profile": false}}
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
    let warship_page1 = serde_json::json!({
        "status": "ok",
        "meta": {"page": 1, "page_total": 2},
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
    let warship_page2 = serde_json::json!({
        "status": "ok",
        "meta": {"page": 2, "page_total": 2},
        "data": {
            "3542005745": {
                "ship_id": 3542005745u64,
                "name": "Erie",
                "nation": "usa",
                "type": "cruiser",
                "tier": 1,
                "images": {"small": "http://example.com/erie.png"},
                "is_premium": false,
                "is_special": false
            }
        }
    });

    let requests: Vec<&mut crux_core::Request<crux_http::HttpRequest>> = update
        .effects_mut()
        .filter_map(|effect| match effect {
            Effect::Http(request) => Some(request),
            _ => None,
        })
        .collect();
    let mut events = Vec::new();
    let mut warship_request = None;
    for request in requests {
        let url = request.operation.url.clone();
        if url.contains("/wows/encyclopedia/ships/") {
            warship_request = Some(request);
            continue;
        }
        let body = if url.contains("/wows/account/info/") {
            player_body.clone()
        } else {
            ships_body.clone()
        };
        let update = app.resolve(request, http_ok(body)).expect("resolve http");
        events.push(update.events.into_iter().next().expect("one event"));
    }
    for event in events {
        let _ = app.update(event, &mut model);
    }

    // Page 1: the ship is stored, the next page is requested, and the
    // player is not assembled from a partial encyclopedia yet.
    let resolved = app
        .resolve(warship_request.expect("warship request"), http_ok(warship_page1))
        .expect("resolve page 1");
    let event = resolved.events.into_iter().next().expect("warship event");
    let mut update = app.update(event, &mut model);
    assert_eq!(model.warship.len(), 1);
    assert!(matches!(model.phase, Phase::LoadingPlayer));

    let page2 = update
        .effects_mut()
        .find_map(|effect| match effect {
            Effect::Http(request) if request.operation.url.contains("page_no=2") => Some(request),
            _ => None,
        })
        .expect("expected page 2 request");
    let resolved = app
        .resolve(page2, http_ok(warship_page2))
        .expect("resolve page 2");
    let event = resolved.events.into_iter().next().expect("warship event");
    let _ = app.update(event, &mut model);

    assert_eq!(model.warship.len(), 2);
    assert!(matches!(model.phase, Phase::Player));
    let player = app.view(&model).player.expect("player view");
    assert_eq!(player.ships.len(), 1);
    assert_eq!(player.ships[0].name, "Hermelin");
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
    // PR + time + player + ships + render
    assert_eq!(update.effects.len(), 6);
    assert!(update.effects.iter().any(|e| matches!(e, Effect::Time(_))));
    assert!(update.effects.iter().any(|e| matches!(e, Effect::Http(_))));
}
