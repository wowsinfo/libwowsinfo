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


mod player;
mod search;
