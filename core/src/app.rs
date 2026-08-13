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

use effects::{
    on_achievements_loaded, on_achievements_wiki_loaded, on_clan_loaded, on_game_version_loaded,
    on_kv_loaded, on_player_loaded, on_pr_loaded, on_recent_loaded, on_search_loaded,
    on_ships_loaded, on_warship_loaded,
};
use handlers::{init, refresh, search, select, set_server};

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
    AchievementsLoaded(HttpOutcome),
    AchievementsWikiLoaded(HttpOutcome),
    ClanLoaded(HttpOutcome),
    RecentLoaded(HttpOutcome),
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
    achievements: Vec<models::Achievement>,
    achievements_wiki: HashMap<String, models::EncyclopediaAchievement>,
    clan_tag: String,
    recent: Option<models::RecentOverview>,
    downloading_achievements: bool,
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
            Event::AchievementsLoaded(outcome) => on_achievements_loaded(model, outcome),
            Event::AchievementsWikiLoaded(outcome) => on_achievements_wiki_loaded(model, outcome),
            Event::ClanLoaded(outcome) => on_clan_loaded(model, outcome),
            Event::RecentLoaded(outcome) => on_recent_loaded(model, outcome),
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


mod effects;
mod handlers;

#[cfg(test)]
mod tests;
