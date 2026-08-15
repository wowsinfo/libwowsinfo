//! App flow tests (init, clan, wiki, player, warship).

//! Split by theme so every file stays small.

use super::*;
use crux_core::testing::AppTester;
use crux_http::HttpResponse;
use crux_http::protocol::HttpResult;

mod clan;
mod init;
mod player;
mod search;
mod warship;
mod wiki;

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

