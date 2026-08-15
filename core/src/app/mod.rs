//! The Crux app: event/model/view-model mapping and orchestration for the
//! player search -> stats flow, ported from the React Native app.
//! Split into domain submodules so every file stays small.

pub(crate) use std::collections::HashMap;
pub(crate) use crux_core::Command;
pub(crate) use crux_core::render;
pub(crate) use crate::{
    api, data::{self as data, Server}, downloader, models, wiki,
};
#[cfg(test)]
pub(crate) use crate::APP_KEY;

pub(crate) type HttpCap = crux_http::Http<Effect, Event>;
pub(crate) type KeyValueCap = crux_kv::KeyValue<Effect, Event>;
pub(crate) type TimeCap = crux_time::Time<Effect, Event>;

mod config;
mod core;
mod datasets;
mod effects;
mod events;
mod handlers;
mod helpers;
mod model;
#[cfg(test)]
mod tests;
mod view;

pub use config::Config;
pub use core::App;
pub use datasets::{SearchResult, WikiDataset};
pub use events::{Effect, Event, HttpOutcome, KvOutcome};
pub(crate) use helpers::{
    api_key, http_outcome, kv_get_event, kv_set_event, MISSING_KEY_MESSAGE,
};
pub use model::Model;
pub use view::{Phase, ViewModel};

pub(crate) use effects::{
    on_achievements_loaded, on_achievements_wiki_loaded, on_clan_info_loaded, on_clan_loaded,
    on_clan_search_loaded, on_clan_selected_loaded, on_game_version_loaded, on_kv_loaded,
    on_online_loaded, on_player_loaded, on_pr_loaded, on_rank_loaded, on_rank_ships_loaded,
    on_recent_loaded, on_search_loaded, on_ship_wiki_loaded, on_ships_loaded, on_warship_loaded,
    on_wiki_loaded,
};
pub(crate) use handlers::{
    init, load_local_compare, load_local_ship_wiki, load_local_warships, load_ship_wiki,
    load_warship, load_wiki, refresh, search, search_clan, select, select_clan,
    select_local_ship_module, set_language, set_local_data, set_local_hp, set_local_spotted,
    set_server, toggle_local_flag, toggle_local_skill, toggle_local_upgrade,
};

