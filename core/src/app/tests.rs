use super::*;
use crux_core::testing::AppTester;
use crux_http::HttpResponse;
use crux_http::protocol::HttpResult;
use crux_kv::{KeyValueResponse, KeyValueResult};

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

fn kv_ok(value: &str) -> KeyValueResult {
    KeyValueResult::Ok {
        response: KeyValueResponse::Get {
            value: value.as_bytes().to_vec().into(),
        },
    }
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
    assert_eq!(http, 1);
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


mod player;
mod search;
