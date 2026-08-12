//! The Crux app: event/model/view-model mapping and orchestration for the
//! player search -> stats flow, ported from the React Native app.

use std::collections::HashMap;

use crux_core::{
    App as AppTrait, Command,
    macros::effect,
    render::{self, RenderOperation},
};
use crux_http::{HttpError, HttpRequest, Response};
use crux_kv::{KeyValueError, KeyValueOperation};
use crux_time::TimeRequest;
use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::{
    APP_KEY, api,
    data::{self, Server},
    downloader, models,
};

type HttpCap = crux_http::Http<Effect, Event>;
type KeyValueCap = crux_kv::KeyValue<Effect, Event>;
type TimeCap = crux_time::Time<Effect, Event>;

/// Startup configuration supplied by the shell.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Facet, PartialEq)]
pub struct Config {
    pub server: Server,
    #[serde(default = "default_language")]
    pub language: String,
    /// Optional override; falls back to the key embedded by `build.rs`.
    #[serde(default)]
    pub api_key: String,
}

fn default_language() -> String {
    data::DEFAULT_API_LANGUAGE.to_string()
}

/// HTTP result crossing the FFI boundary (plain types so every shell language
/// can represent it without extra dependencies).
#[derive(Debug, Clone, Serialize, Deserialize, Facet, PartialEq)]
#[repr(C)]
pub enum HttpOutcome {
    Ok { body: String },
    Err { message: String },
}

/// Key-value result crossing the FFI boundary.
#[derive(Debug, Clone, Serialize, Deserialize, Facet, PartialEq)]
#[repr(C)]
pub enum KvOutcome {
    Ok { value: Option<String> },
    Err { message: String },
}

/// Events the shell can send to the core, plus capability responses.
#[derive(Debug, Clone, Serialize, Deserialize, Facet, PartialEq)]
#[repr(C)]
pub enum Event {
    Init(Config),
    SetServer(Server),
    SearchPlayer {
        query: String,
    },
    SelectPlayer {
        account_id: u64,
    },
    Refresh,
    /// Persisted the server preference.
    ServerSaved,
    /// Response to `Time::now`.
    NowLoaded(i64),
    GameVersionLoaded(HttpOutcome),
    SearchLoaded(HttpOutcome),
    PlayerLoaded(HttpOutcome),
    ShipsLoaded(HttpOutcome),
    WarshipLoaded(HttpOutcome),
    PrLoaded(HttpOutcome),
    KvLoaded {
        key: String,
        value: KvOutcome,
    },
}

/// Side effects the core can request from the shell.
#[derive(Debug)]
#[effect(facet_typegen)]
pub enum Effect {
    Render(RenderOperation),
    Http(HttpRequest),
    KeyValue(KeyValueOperation),
    Time(TimeRequest),
}

/// UI state as seen by the shells.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Facet, PartialEq)]
pub struct ViewModel {
    pub phase: Phase,
    #[serde(default)]
    pub search_results: Vec<SearchResult>,
    #[serde(default)]
    pub player: Option<models::PlayerView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Facet, PartialEq)]
#[repr(C)]
pub enum Phase {
    Idle,
    Searching,
    LoadingPlayer,
    Player,
    Error(String),
}

impl Default for Phase {
    fn default() -> Self {
        Self::Idle
    }
}

/// One search hit shown in the UI.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Facet, PartialEq)]
pub struct SearchResult {
    pub account_id: u64,
    pub nickname: String,
}

/// Internal state of the app.
#[derive(Default)]
pub struct Model {
    config: Option<Config>,
    server: Server,
    api_language: String,
    game_version: Option<String>,
    warship: HashMap<u64, models::EncyclopediaShip>,
    pr: HashMap<u64, models::PrEntry>,
    phase: Phase,
    search_results: Vec<models::AccountListEntry>,
    pending_account_id: Option<u64>,
    pending_player: Option<models::PlayerInfo>,
    pending_ships: Option<Vec<models::ShipStats>>,
    selected: Option<models::PlayerView>,
    /// True while the paginated ship encyclopedia download is in progress.
    downloading_warship: bool,
}

/// The application.
#[derive(Default)]
pub struct App;

impl AppTrait for App {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::Init(config) => init(model, config),
            Event::SetServer(server) => set_server(model, server),
            Event::SearchPlayer { query } => search(model, query),
            Event::SelectPlayer { account_id } => select(model, account_id),
            Event::Refresh => refresh(model),
            Event::ServerSaved => render::render(),
            Event::NowLoaded(_) => render::render(),
            Event::GameVersionLoaded(outcome) => on_game_version_loaded(model, outcome),
            Event::SearchLoaded(outcome) => on_search_loaded(model, outcome),
            Event::PlayerLoaded(outcome) => on_player_loaded(model, outcome),
            Event::ShipsLoaded(outcome) => on_ships_loaded(model, outcome),
            Event::WarshipLoaded(outcome) => on_warship_loaded(model, outcome),
            Event::PrLoaded(outcome) => on_pr_loaded(model, outcome),
            Event::KvLoaded { key, value } => on_kv_loaded(model, key, value),
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            phase: model.phase.clone(),
            search_results: model
                .search_results
                .iter()
                .map(|r| SearchResult {
                    account_id: r.account_id,
                    nickname: r.nickname.clone(),
                })
                .collect(),
            player: model.selected.clone(),
        }
    }
}

const MISSING_KEY_MESSAGE: &str =
    "Missing Wargaming API key. Add one to keys.toml or set WOWSINFO_APP_KEY.";

fn api_key(config: &Config) -> String {
    if config.api_key.is_empty() {
        APP_KEY.to_string()
    } else {
        config.api_key.clone()
    }
}

fn http_outcome(result: Result<Response<String>, HttpError>) -> HttpOutcome {
    match result {
        Ok(response) => HttpOutcome::Ok {
            body: response.body().cloned().unwrap_or_default(),
        },
        Err(error) => HttpOutcome::Err {
            message: error.to_string(),
        },
    }
}

fn kv_outcome(result: Result<Option<Vec<u8>>, KeyValueError>) -> KvOutcome {
    match result {
        Ok(value) => KvOutcome::Ok {
            value: value.map(|bytes| String::from_utf8_lossy(&bytes).into_owned()),
        },
        Err(error) => KvOutcome::Err {
            message: error.to_string(),
        },
    }
}

fn kv_get_event(key: &'static str) -> Command<Effect, Event> {
    KeyValueCap::get(key).then_send(|result| Event::KvLoaded {
        key: key.to_string(),
        value: kv_outcome(result),
    })
}

fn kv_set_event(key: &'static str, value: String) -> Command<Effect, Event> {
    KeyValueCap::set(key, value.into_bytes()).then_send(|_| Event::ServerSaved)
}

fn init(model: &mut Model, config: Config) -> Command<Effect, Event> {
    model.config = Some(config.clone());
    model.server = config.server;
    model.api_language = if config.language.is_empty() {
        data::DEFAULT_API_LANGUAGE.to_string()
    } else {
        config.language.clone()
    };

    let key = api_key(&config);
    if key.is_empty() {
        model.phase = Phase::Error(MISSING_KEY_MESSAGE.to_string());
        return render::render();
    }

    Command::all([
        kv_get_event(data::local::USER_SERVER),
        kv_get_event(data::local::USER_LANGUAGE),
        kv_get_event(data::saved::WARSHIP),
        kv_get_event(data::saved::PR),
        HttpCap::get(api::game_version(model.server, &key))
            .expect_string()
            .build()
            .then_send(|result| Event::GameVersionLoaded(http_outcome(result))),
        render::render(),
    ])
}

fn set_server(model: &mut Model, server: Server) -> Command<Effect, Event> {
    model.server = server;
    if let Some(config) = model.config.as_mut() {
        config.server = server;
    }
    let value = serde_json::to_string(&(server as u8)).unwrap_or_default();
    Command::all([
        kv_set_event(data::local::USER_SERVER, value),
        render::render(),
    ])
}

fn search(model: &mut Model, query: String) -> Command<Effect, Event> {
    model.search_results.clear();
    if query.trim().is_empty() {
        model.phase = Phase::Idle;
        return render::render();
    }
    let Some(config) = model.config.clone() else {
        model.phase = Phase::Error("App not initialised".to_string());
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        model.phase = Phase::Error(MISSING_KEY_MESSAGE.to_string());
        return render::render();
    }
    model.phase = Phase::Searching;
    HttpCap::get(api::player_search(model.server, &key, &query))
        .expect_string()
        .build()
        .then_send(|result| Event::SearchLoaded(http_outcome(result)))
}

fn select(model: &mut Model, account_id: u64) -> Command<Effect, Event> {
    model.phase = Phase::LoadingPlayer;
    model.pending_account_id = Some(account_id);
    model.pending_player = None;
    model.pending_ships = None;

    let Some(config) = model.config.clone() else {
        model.phase = Phase::Error("App not initialised".to_string());
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        model.phase = Phase::Error(MISSING_KEY_MESSAGE.to_string());
        return render::render();
    }
    Command::all(player_commands(model, &key, account_id))
}

fn player_commands(model: &mut Model, key: &str, account_id: u64) -> Vec<Command<Effect, Event>> {
    let mut commands = vec![
        HttpCap::get(api::player_info(model.server, key, account_id))
            .expect_string()
            .build()
            .then_send(|result| Event::PlayerLoaded(http_outcome(result))),
        HttpCap::get(api::ship_info(model.server, key, account_id))
            .expect_string()
            .build()
            .then_send(|result| Event::ShipsLoaded(http_outcome(result))),
    ];
    if model.warship.is_empty() && !model.downloading_warship {
        model.downloading_warship = true;
        commands.push(
            HttpCap::get(api::warship(model.server, key, 1, &model.api_language))
                .expect_string()
                .build()
                .then_send(|result| Event::WarshipLoaded(http_outcome(result))),
        );
    }
    commands.push(render::render());
    commands
}

fn refresh(model: &mut Model) -> Command<Effect, Event> {
    let Some(config) = model.config.clone() else {
        model.phase = Phase::Error("App not initialised".to_string());
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        model.phase = Phase::Error(MISSING_KEY_MESSAGE.to_string());
        return render::render();
    }

    let mut commands: Vec<Command<Effect, Event>> = vec![
        HttpCap::get(api::PERSONAL_RATING)
            .expect_string()
            .build()
            .then_send(|result| Event::PrLoaded(http_outcome(result))),
        TimeCap::now().then_send(|now| {
            Event::NowLoaded(
                now.duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0),
            )
        }),
    ];
    if model.warship.is_empty() && !model.downloading_warship {
        model.downloading_warship = true;
        commands.push(
            HttpCap::get(api::warship(model.server, &key, 1, &model.api_language))
                .expect_string()
                .build()
                .then_send(|result| Event::WarshipLoaded(http_outcome(result))),
        );
    }
    if let Some(account_id) = model.pending_account_id {
        commands.extend(player_commands(model, &key, account_id));
    }
    commands.push(render::render());
    Command::all(commands)
}

fn on_search_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            model.search_results = downloader::parse_search_results(&json);
            model.phase = Phase::Idle;
        }
        HttpOutcome::Err { message } => {
            model.phase = Phase::Error(format!("Search failed: {message}"));
        }
    }
    render::render()
}

fn on_player_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(account_id) = model.pending_account_id {
                if let Some(player) = downloader::parse_player_info(&json, account_id) {
                    model.pending_player = Some(player);
                } else {
                    model.phase = Phase::Error("Player profile not found".to_string());
                }
            }
        }
        HttpOutcome::Err { message } => {
            model.phase = Phase::Error(format!("Player lookup failed: {message}"));
        }
    }
    try_assemble(model)
}

fn on_ships_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(account_id) = model.pending_account_id {
                model.pending_ships = Some(downloader::parse_ship_stats(&json, account_id));
            }
        }
        HttpOutcome::Err { message } => {
            model.phase = Phase::Error(format!("Ship stats failed: {message}"));
        }
    }
    try_assemble(model)
}

fn on_warship_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    let mut commands = Vec::new();
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            let empty = serde_json::Value::Object(Default::default());
            let data = downloader::guard(&json, "data", &empty);
            if let Some(map) = data.as_object() {
                for value in map.values() {
                    if let Ok(raw) =
                        serde_json::from_value::<models::RawEncyclopediaShip>(value.clone())
                    {
                        let processed = downloader::process_warship_entry(raw, false);
                        model.warship.insert(processed.ship_id, processed);
                    }
                }
            }

            // The encyclopedia is paginated (`meta.page` / `meta.page_total`).
            // Keep fetching until the last page, then cache and assemble so the
            // player view is built from the complete ship encyclopedia.
            let meta = downloader::guard(&json, "meta", &empty);
            let page = meta.get("page").and_then(|v| v.as_u64()).unwrap_or(1);
            let page_total = meta.get("page_total").and_then(|v| v.as_u64()).unwrap_or(1);
            let can_fetch = model
                .config
                .as_ref()
                .is_some_and(|config| !api_key(config).is_empty());
            if page < page_total && can_fetch {
                let config = model.config.clone().expect("checked above");
                let key = api_key(&config);
                commands.push(
                    HttpCap::get(api::warship(model.server, &key, page + 1, &model.api_language))
                        .expect_string()
                        .build()
                        .then_send(|result| Event::WarshipLoaded(http_outcome(result))),
                );
            } else {
                model.downloading_warship = false;
                if let Ok(json) = serde_json::to_string(&model.warship) {
                    commands.push(kv_set_event(data::saved::WARSHIP, json));
                }
                commands.push(try_assemble(model));
            }
        }
        HttpOutcome::Err { message } => {
            model.downloading_warship = false;
            model.phase = Phase::Error(format!("Wiki download failed: {message}"));
            commands.push(try_assemble(model));
        }
    }
    Command::all(commands)
}

fn on_pr_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            let pr = downloader::parse_pr(&json);
            if !pr.is_empty() {
                model.pr = pr;
                if let Ok(json) = serde_json::to_string(&model.pr) {
                    return Command::all([
                        kv_set_event(data::saved::PR, json),
                        try_assemble(model),
                    ]);
                }
            }
        }
        HttpOutcome::Err { .. } => {}
    }
    try_assemble(model)
}

fn on_game_version_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            let empty = serde_json::Value::String(String::new());
            let version = downloader::guard(&json, "data.game_version", &empty)
                .as_str()
                .unwrap_or("")
                .to_string();
            if !version.is_empty() {
                model.game_version = Some(version);
            }
        }
        HttpOutcome::Err { .. } => {}
    }
    render::render()
}

fn on_kv_loaded(model: &mut Model, key: String, value: KvOutcome) -> Command<Effect, Event> {
    match value {
        KvOutcome::Ok { value: Some(json) } => match key.as_str() {
            data::local::USER_SERVER => {
                if let Ok(index) = serde_json::from_str::<u8>(&json) {
                    if let Some(server) = Server::from_index(index as usize) {
                        model.server = server;
                    }
                }
            }
            data::local::USER_LANGUAGE => {
                if let Ok(language) = serde_json::from_str::<String>(&json) {
                    model.api_language = language;
                }
            }
            data::saved::WARSHIP => {
                if let Ok(map) =
                    serde_json::from_str::<HashMap<u64, models::EncyclopediaShip>>(&json)
                {
                    model.warship = map;
                }
            }
            data::saved::PR => {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json) {
                    model.pr = downloader::parse_pr(&value);
                }
            }
            _ => {}
        },
        KvOutcome::Ok { value: None } | KvOutcome::Err { .. } => {
            if key == data::saved::PR && model.pr.is_empty() {
                // Bundled table is the fallback when nothing is cached yet.
                model.pr = downloader::local_pr();
            }
        }
    }
    render::render()
}

fn try_assemble(model: &mut Model) -> Command<Effect, Event> {
    if model.downloading_warship {
        // The ship encyclopedia is still downloading; keep the loading phase
        // until the final page so the view isn't assembled from partial data.
        return render::render();
    }
    if let (Some(player), Some(ships)) = (model.pending_player.clone(), model.pending_ships.clone())
    {
        let view =
            downloader::assemble_player(player, ships, &model.pr, &model.warship, model.server);
        model.selected = Some(view);
        model.phase = Phase::Player;
    }
    render::render()
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
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
        assert_eq!(kv, 4);
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
    fn search_flow_returns_results() {
        let app = AppTester::<App>::default();
        let mut model = Model::default();
        let _ = app.update(Event::Init(config()), &mut model);

        let update = app.update(
            Event::SearchPlayer {
                query: "henry".to_string(),
            },
            &mut model,
        );
        assert!(matches!(model.phase, Phase::Searching));

        let body = serde_json::json!({
            "status": "ok",
            "data": [{"account_id": 1, "nickname": "HenryQuan"}]
        });
        let event = resolve_http_matching(&app, update, "/wows/account/list/", body);
        let Event::SearchLoaded(outcome) = event else {
            panic!("expected SearchLoaded, got {event:?}");
        };
        let _ = app.update(Event::SearchLoaded(outcome), &mut model);

        assert!(matches!(model.phase, Phase::Idle));
        assert_eq!(model.search_results.len(), 1);
        assert_eq!(app.view(&model).search_results[0].nickname, "HenryQuan");
    }

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
    fn search_error_sets_phase() {
        let app = AppTester::<App>::default();
        let mut model = Model::default();
        let _ = app.update(Event::Init(config()), &mut model);
        let update = app.update(
            Event::SearchPlayer {
                query: "nobody".to_string(),
            },
            &mut model,
        );
        let mut update = update;
        let effect = update
            .effects_mut()
            .find(|e| matches!(e, Effect::Http(_)))
            .expect("http effect");
        let Effect::Http(request) = effect else {
            unreachable!()
        };
        let update = app
            .resolve(
                request,
                HttpResult::Err(crux_http::HttpError::from(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "network down",
                ))),
            )
            .expect("resolve");
        let event = update.events.into_iter().next().expect("event");
        let _ = app.update(event, &mut model);
        assert!(matches!(model.phase, Phase::Error(_)));
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
}
