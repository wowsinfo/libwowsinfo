//! The Crux app: event/model/view-model mapping and orchestration for the
//! player search -> stats flow, ported from the React Native app.

use std::collections::{HashMap, HashSet};

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
    downloader, models, wiki,
};

type HttpCap = crux_http::Http<Effect, Event>;
type KeyValueCap = crux_kv::KeyValue<Effect, Event>;
type TimeCap = crux_time::Time<Effect, Event>;

use effects::{
    on_achievements_loaded, on_achievements_wiki_loaded, on_clan_info_loaded, on_clan_loaded,
    on_clan_search_loaded, on_clan_selected_loaded, on_game_version_loaded, on_kv_loaded,
    on_online_loaded, on_player_loaded, on_pr_loaded, on_rank_loaded, on_rank_ships_loaded,
    on_recent_loaded, on_search_loaded, on_ship_wiki_loaded, on_ships_loaded, on_warship_loaded,
    on_wiki_loaded,
};
use handlers::{
    init, load_local_compare, load_local_ship_wiki, load_local_warships, load_ship_wiki,
    load_warship, load_wiki, refresh, search, search_clan, select, select_clan,
    select_local_ship_module, set_language, set_local_data, set_local_hp, set_local_spotted,
    set_server, toggle_local_flag, toggle_local_skill, toggle_local_upgrade,
};

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
    /// Search clans by name/tag (`/wows/clans/list/`).
    SearchClan {
        query: String,
    },
    /// Open a clan's info screen (`/wows/clans/info/`).
    SelectClan {
        clan_id: u64,
    },
    /// Load a wiki dataset on demand (paginated encyclopedia endpoint).
    LoadWiki {
        dataset: WikiDataset,
    },
    /// Load the ship encyclopedia on demand (`/wows/encyclopedia/ships/`).
    LoadWarship,
    /// Load a ship's full wiki entry (`/wows/encyclopedia/ships/?ship_id=`).
    LoadShipWiki {
        ship_id: u64,
    },
    /// Load the bundled `wowsinfo.zst` and `lang.zst` for local wiki mode
    /// (zstd frames, decompressed in memory on the Rust side).
    SetLocalData {
        ships: Vec<u8>,
        lang: Vec<u8>,
    },
    /// Fill the warship encyclopedia from the local game data.
    LoadLocalWarships,
    /// Build the local wiki entry for one ship from `wowsinfo.json`.
    LoadLocalShipWiki {
        ship_id: u64,
    },
    /// Change a module slot of the currently selected local ship.
    SelectLocalShipModule {
        slot: String,
        index: i64,
    },
    /// Build a local comparison table for the given ships.
    LoadLocalCompare {
        ship_ids: Vec<u64>,
    },
    /// Toggle a commander skill in the local ship build.
    ToggleLocalSkill {
        key: String,
    },
    /// Toggle a module upgrade in the local ship build.
    ToggleLocalUpgrade {
        key: String,
    },
    /// Toggle a signal flag in the local ship build.
    ToggleLocalFlag {
        key: String,
    },
    /// Set the simulated HP fraction (0..1) for conditional skills.
    SetLocalHp {
        fraction: f64,
    },
    /// Set whether the ship is spotted (drives trigger skills).
    SetLocalSpotted {
        spotted: bool,
    },
    /// Change the interface/data language (persisted via key-value store).
    SetLanguage {
        language: String,
    },
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
    /// Full clan info (`/wows/clans/info/`), fetched after `ClanLoaded`.
    ClanInfoLoaded(HttpOutcome),
    ClanSearchLoaded(HttpOutcome),
    /// Full clan info for the clan screen, requested via `SelectClan`.
    ClanSelectedLoaded(HttpOutcome),
    RecentLoaded(HttpOutcome),
    /// Ranked seasons (`/wows/seasons/accountinfo/`).
    RankLoaded(HttpOutcome),
    /// Ranked ship stats (`/wows/seasons/shipstats/`).
    RankShipsLoaded(HttpOutcome),
    /// Players online (`/wgn/servers/info/`).
    OnlineLoaded(HttpOutcome),
    WikiLoaded {
        dataset: WikiDataset,
        outcome: HttpOutcome,
    },
    ShipWikiLoaded(HttpOutcome),
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
    #[serde(default)]
    pub clan_search_results: Vec<models::ClanSearchResult>,
    #[serde(default)]
    pub selected_clan: Option<models::ClanInfo>,
    /// Players online across the game; -1 when the request failed.
    #[serde(default)]
    pub online: i64,
    #[serde(default)]
    pub warship: HashMap<u64, models::EncyclopediaShip>,
    #[serde(default)]
    pub wiki_collections: HashMap<u64, models::WikiCollection>,
    #[serde(default)]
    pub wiki_collection_cards: HashMap<u64, models::CollectionCard>,
    #[serde(default)]
    pub wiki_consumables: HashMap<u64, models::Consumable>,
    #[serde(default)]
    pub wiki_commander_skills: HashMap<u64, models::CommanderSkill>,
    #[serde(default)]
    pub wiki_maps: HashMap<u64, models::WikiMap>,
    #[serde(default)]
    pub selected_ship_wiki: Option<models::ShipWiki>,
    #[serde(default)]
    pub local_ship: Option<wiki::LocalShipWiki>,
    #[serde(default)]
    pub local_compare: Option<wiki::LocalCompare>,
    #[serde(default)]
    pub local_consumables: Vec<wiki::ConsumableView>,
    #[serde(default)]
    pub local_skills_wiki: Vec<wiki::LocalSkillWikiEntry>,
    #[serde(default)]
    pub local_achievements: Vec<wiki::LocalAchievementEntry>,
    #[serde(default)]
    pub local_upgrades_wiki: Vec<wiki::LocalUpgradeEntry>,
    #[serde(default)]
    pub local_flags_wiki: Vec<wiki::LocalFlagEntry>,
    /// True once the bundled game data has been parsed successfully.
    #[serde(default)]
    pub local_data_ready: bool,
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

/// A wiki dataset that can be loaded on demand (`/wows/encyclopedia/*`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Facet)]
#[repr(C)]
pub enum WikiDataset {
    Collections,
    CollectionCards,
    Consumables,
    CommanderSkills,
    Maps,
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
    rank: Option<models::RankPlayerInfo>,
    rank_ships: Vec<models::RankShipStat>,
    clan: Option<models::ClanInfo>,
    clan_id: Option<u64>,
    clan_search_results: Vec<models::ClanSearchResult>,
    selected_clan: Option<models::ClanInfo>,
    online: i64,
    wiki_collections: HashMap<u64, models::WikiCollection>,
    wiki_collection_cards: HashMap<u64, models::CollectionCard>,
    wiki_consumables: HashMap<u64, models::Consumable>,
    wiki_commander_skills: HashMap<u64, models::CommanderSkill>,
    wiki_maps: HashMap<u64, models::WikiMap>,
    downloading_wiki: HashSet<WikiDataset>,
    selected_ship_wiki: Option<models::ShipWiki>,
    pending_ship_wiki_id: Option<u64>,
    local_data: Option<wiki::GameData>,
    local_lang: wiki::LangMap,
    local_selection: wiki::ModuleSelection,
    local_ship: Option<wiki::LocalShipWiki>,
    local_ship_id: Option<u64>,
    local_compare: Option<wiki::LocalCompare>,
    local_skills: HashSet<String>,
    local_upgrades: HashSet<String>,
    local_flags: HashSet<String>,
    local_hp: f64,
    local_spotted: bool,
    local_consumables: Vec<wiki::ConsumableView>,
    local_skills_wiki: Vec<wiki::LocalSkillWikiEntry>,
    local_achievements: Vec<wiki::LocalAchievementEntry>,
    local_upgrades_wiki: Vec<wiki::LocalUpgradeEntry>,
    local_flags_wiki: Vec<wiki::LocalFlagEntry>,
    /// Raw `lang.json` string so the language can be re-parsed on change.
    raw_lang_json: Option<String>,
    /// True once the user picked a language this session; the init restore
    /// must not overwrite it.
    language_overridden: bool,
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
            Event::SearchClan { query } => search_clan(model, query),
            Event::SelectClan { clan_id } => select_clan(model, clan_id),
            Event::LoadWiki { dataset } => load_wiki(model, dataset),
            Event::LoadWarship => load_warship(model),
            Event::LoadShipWiki { ship_id } => load_ship_wiki(model, ship_id),
            Event::SetLocalData { ships, lang } => set_local_data(model, ships, lang),
            Event::LoadLocalWarships => load_local_warships(model),
            Event::LoadLocalShipWiki { ship_id } => load_local_ship_wiki(model, ship_id),
            Event::SelectLocalShipModule { slot, index } => {
                select_local_ship_module(model, slot, index)
            }
            Event::LoadLocalCompare { ship_ids } => load_local_compare(model, ship_ids),
            Event::ToggleLocalSkill { key } => toggle_local_skill(model, key),
            Event::ToggleLocalUpgrade { key } => toggle_local_upgrade(model, key),
            Event::ToggleLocalFlag { key } => toggle_local_flag(model, key),
            Event::SetLocalHp { fraction } => set_local_hp(model, fraction),
            Event::SetLocalSpotted { spotted } => set_local_spotted(model, spotted),
            Event::SetLanguage { language } => set_language(model, language),
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
            Event::ClanInfoLoaded(outcome) => on_clan_info_loaded(model, outcome),
            Event::ClanSearchLoaded(outcome) => on_clan_search_loaded(model, outcome),
            Event::ClanSelectedLoaded(outcome) => on_clan_selected_loaded(model, outcome),
            Event::RecentLoaded(outcome) => on_recent_loaded(model, outcome),
            Event::RankLoaded(outcome) => on_rank_loaded(model, outcome),
            Event::RankShipsLoaded(outcome) => on_rank_ships_loaded(model, outcome),
            Event::OnlineLoaded(outcome) => on_online_loaded(model, outcome),
            Event::WikiLoaded { dataset, outcome } => on_wiki_loaded(model, dataset, outcome),
            Event::ShipWikiLoaded(outcome) => on_ship_wiki_loaded(model, outcome),
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
            clan_search_results: model.clan_search_results.clone(),
            selected_clan: model.selected_clan.clone(),
            online: model.online,
            warship: model.warship.clone(),
            wiki_collections: model.wiki_collections.clone(),
            wiki_collection_cards: model.wiki_collection_cards.clone(),
            wiki_consumables: model.wiki_consumables.clone(),
            wiki_commander_skills: model.wiki_commander_skills.clone(),
            wiki_maps: model.wiki_maps.clone(),
            selected_ship_wiki: model.selected_ship_wiki.clone(),
            local_ship: model.local_ship.clone(),
            local_compare: model.local_compare.clone(),
            local_consumables: model.local_consumables.clone(),
            local_skills_wiki: model.local_skills_wiki.clone(),
            local_flags_wiki: model.local_flags_wiki.clone(),
            local_achievements: model.local_achievements.clone(),
            local_upgrades_wiki: model.local_upgrades_wiki.clone(),
            local_data_ready: model.local_data.is_some(),
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
