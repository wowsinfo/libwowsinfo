//! Events and capability results crossing the FFI boundary.

use crux_core::{macros::effect, render::RenderOperation};
use crux_http::HttpRequest;
use crux_kv::KeyValueOperation;
use crux_time::TimeRequest;
use facet::Facet;
use serde::{Deserialize, Serialize};

use super::config::Config;
use super::datasets::WikiDataset;
use crate::data::Server;

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

