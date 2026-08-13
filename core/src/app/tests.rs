#![allow(deprecated)]

use super::*;
use crux_core::testing::AppTester;
use crux_http::HttpResponse;
use crux_http::protocol::HttpResult;

fn config() -> Config {
    Config {
        server: Server::Asia,
        language: "en".to_string(),
        api_key: "TEST-KEY".to_string(),
    }
}

fn http_ok(body: serde_json::Value) -> HttpResult {
    HttpResult::Ok(HttpResponse::ok().json(&body).build())
}

fn resolve_http_matching(
    app: &AppTester<App>,
    update: crux_core::testing::Update<Effect, Event>,
    url_contains: &str,
    body: serde_json::Value,
) -> Event {
    let mut update = update;
    let request = update
        .effects_mut()
        .find_map(|effect| match effect {
            Effect::Http(request) if request.operation.url.contains(url_contains) => {
                Some(request)
            }
            _ => None,
        })
        .expect("expected an HTTP effect");
    let update = app.resolve(request, http_ok(body)).expect("resolve");
    update
        .events
        .into_iter()
        .next()
        .expect("expected one event")
}

#[test]
fn missing_key_surfaces_error_phase() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let config = Config {
        api_key: String::new(),
        ..config()
    };
    let update = app.update(Event::Init(config), &mut model);
    if APP_KEY.is_empty() {
        assert_eq!(update.effects.len(), 1);
        assert!(matches!(update.effects[0], Effect::Render(_)));
        assert!(matches!(model.phase, Phase::Error(_)));
    } else {
        // A key is embedded in this build, so an empty override falls back
        // to it and init proceeds normally.
        assert!(!matches!(model.phase, Phase::Error(_)));
        assert!(update.effects.len() > 1);
    }
    assert_eq!(app.view(&model).phase, model.phase);
}

#[test]
fn init_requests_caches_version_and_render() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let update = app.update(Event::Init(config()), &mut model);
    let http = update
        .effects()
        .filter(|e| matches!(e, Effect::Http(_)))
        .count();
    let kv = update
        .effects()
        .filter(|e| matches!(e, Effect::KeyValue(_)))
        .count();
    let renders = update
        .effects()
        .filter(|e| matches!(e, Effect::Render(_)))
        .count();
    assert_eq!(http, 2, "game version + players online");
    assert_eq!(kv, 5);
    assert_eq!(renders, 1);
}

#[test]
fn init_loads_cached_pr_from_kv() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);

    // Feed a cached PR table back through the same event a shell would.
    let event = Event::KvLoaded {
        key: data::saved::PR.to_string(),
        value: KvOutcome::Ok {
            value: Some(serde_json::to_string(&downloader::local_pr()).unwrap()),
        },
    };
    let _ = app.update(event, &mut model);

    assert!(model.pr.len() > 10, "local PR fallback should be loaded");
    assert_eq!(app.view(&model).phase, Phase::Idle);
}

#[test]
fn init_loads_players_online() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let update = app.update(Event::Init(config()), &mut model);
    let event = resolve_http_matching(
        &app,
        update,
        "servers/info",
        serde_json::json!({"status": "ok", "data": {"wows": [{"players_online": 4321}]}}),
    );
    let _ = app.update(event, &mut model);
    assert_eq!(app.view(&model).online, 4321);
}

#[test]
fn select_clan_loads_clan_info_into_view() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    let update = app.update(Event::SelectClan { clan_id: 99 }, &mut model);
    let event = resolve_http_matching(
        &app,
        update,
        "clans/info",
        serde_json::json!({"status": "ok", "data": {"99": {
            "clan_id": 99,
            "tag": "ABC",
            "name": "Alpha",
            "members_count": 1,
            "members": {"7": {"account_name": "Bob", "role": "commander", "joined_at": 100}}
        }}}),
    );
    let _ = app.update(event, &mut model);
    let clan = app.view(&model).selected_clan.expect("selected clan");
    assert_eq!(clan.name, "Alpha");
    assert_eq!(clan.members.len(), 1);
    assert_eq!(clan.members[0].account_name, "Bob");
}

#[test]
fn load_wiki_collections_populates_view() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    let update = app.update(
        Event::LoadWiki {
            dataset: WikiDataset::Collections,
        },
        &mut model,
    );
    let event = resolve_http_matching(
        &app,
        update,
        "encyclopedia/collections",
        serde_json::json!({
            "status": "ok",
            "meta": {"page": 1, "page_total": 1},
            "data": {"1": {"collection_id": 1, "name": "C1", "description": "d", "image": "i"}}
        }),
    );
    let _ = app.update(event, &mut model);
    let collections = app.view(&model).wiki_collections;
    assert_eq!(collections.len(), 1);
    assert_eq!(collections.get(&1).map(|c| c.name.as_str()), Some("C1"));
    assert!(!model.downloading_wiki.contains(&WikiDataset::Collections));
}

#[test]
fn load_ship_wiki_populates_view() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    let update = app.update(
        Event::LoadShipWiki {
            ship_id: 3542005744,
        },
        &mut model,
    );
    let event = resolve_http_matching(
        &app,
        update,
        "encyclopedia/ships",
        serde_json::json!({
            "status": "ok",
            "data": {"3542005744": {
                "ship_id": 3542005744u64,
                "name": "Hermelin",
                "type": "dd",
                "tier": 1,
                "default_profile": {
                    "armour": {"total": 51, "health": 37500},
                    "weaponry": {"artillery": 72},
                    "mobility": {"total": 55, "max_speed": 32.5}
                }
            }}
        }),
    );
    let _ = app.update(event, &mut model);
    let ship = app.view(&model).selected_ship_wiki.expect("ship wiki");
    assert_eq!(ship.name, "Hermelin");
    assert_eq!(ship.profile.armour.total, 51);
    assert_eq!(ship.profile.weaponry.artillery, 72);
}

#[test]
fn local_data_drives_ship_wiki_and_warship_list() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    let ships = serde_json::json!({
        "ships": {
            "1": {
                "id": 1, "index": "PASC014", "name": "IDS_NAME", "description": "IDS_D",
                "year": "IDS_Y", "paperShip": false, "tier": 8, "region": "USA",
                "type": "Cruiser", "regionID": "IDS_USA", "typeID": "IDS_CRUISER",
                "group": "normal", "costXP": 0, "costGold": 0, "costCR": 100,
                "consumables": [], "nextShips": [],
                "modules": {
                    "_Hull": [{
                        "index": 0, "name": "IDS_HULL", "cost": {"costXP": 0, "costCR": 0},
                        "components": {"hull": ["A_Hull"], "artillery": ["A1_Guns"]}
                    }]
                },
                "components": {
                    "A_Hull": {"health": 30000.0, "protection": 4.0,
                               "mobility": {"speed": 32.0},
                               "visibility": {"sea": 11.5}},
                    "A1_Guns": {"range": 14699.0, "sigma": 2.0, "guns": [{
                        "reload": 15.0, "rotation": 25.7, "each": 3,
                        "ammo": ["PAPA_AP"], "vertSector": 41.0, "count": 3}]}
                }
            }
        },
        "projectiles": {
            "PAPA_AP": {
                "type": "Artillery", "nation": "USA", "name": "IDS_AP",
                "ammoType": "AP", "speed": 800.0, "weight": 120.0, "damage": 5000.0,
                "diameter": 0.203,
                "ap": {"diameter": 0.203, "weight": 120.0, "drag": 0.3,
                       "velocity": 800.0, "krupp": 2400.0}
            }
        }
    })
    .to_string();
    let lang = serde_json::json!({
        "en": {
            "IDS_NAME": "New Orleans", "IDS_HULL": "Hull A", "IDS_AP": "203 mm AP",
            "IDS_CRUISER": "Cruiser", "IDS_USA": "U.S.A."
        }
    })
    .to_string();

    let _ = app.update(
        Event::SetLocalData {
            ships: ships.clone(),
            lang,
        },
        &mut model,
    );
    assert!(app.view(&model).local_data_ready);
    let _ = app.update(Event::LoadLocalWarships, &mut model);
    assert_eq!(app.view(&model).warship.len(), 1);
    assert_eq!(app.view(&model).warship[&1].name, "PASC014");

    let _ = app.update(Event::LoadLocalShipWiki { ship_id: 1 }, &mut model);
    let wiki = app.view(&model).local_ship.expect("local ship");
    assert_eq!(wiki.name, "New Orleans");
    assert_eq!(wiki.main_battery.as_ref().map(|m| m.configuration.as_str()), Some("3 x 3"));
    assert_eq!(wiki.penetration_curves.len(), 1);

    let _ = app.update(
        Event::SelectLocalShipModule {
            slot: "artillery".to_string(),
            index: 0,
        },
        &mut model,
    );
    assert!(app.view(&model).local_ship.is_some());

    let _ = app.update(Event::LoadLocalCompare { ship_ids: vec![1] }, &mut model);
    let compare = app.view(&model).local_compare.expect("local compare");
    assert_eq!(compare.ships.len(), 1);
    assert_eq!(compare.rows.len(), 15);
    assert_eq!(compare.rows[0].label, "Tier");
}


mod player;
mod search;
