//! Player / server / search entry points.

use super::super::*;

pub(super) fn init(model: &mut Model, config: Config) -> Command<Effect, Event> {
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
        kv_get_event(data::saved::ACHIEVEMENT),
        HttpCap::get(api::game_version(model.server, &key))
            .expect_string()
            .build()
            .then_send(|result| Event::GameVersionLoaded(http_outcome(result))),
        HttpCap::get(api::player_online(model.server, &key))
            .expect_string()
            .build()
            .then_send(|result| Event::OnlineLoaded(http_outcome(result))),
        render::render(),
    ])
}


pub(super) fn set_server(model: &mut Model, server: Server) -> Command<Effect, Event> {
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


pub(super) fn search(model: &mut Model, query: String) -> Command<Effect, Event> {
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

/// Clan search: `/wows/clans/list/`.

pub(super) fn search_clan(model: &mut Model, query: String) -> Command<Effect, Event> {
    model.clan_search_results.clear();
    if query.trim().is_empty() {
        return render::render();
    }
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    HttpCap::get(api::clan_search(model.server, &key, &query))
        .expect_string()
        .build()
        .then_send(|result| Event::ClanSearchLoaded(http_outcome(result)))
}

/// Open clan info: `/wows/clans/info/`.

pub(super) fn select_clan(model: &mut Model, clan_id: u64) -> Command<Effect, Event> {
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    HttpCap::get(api::clan_info(model.server, &key, clan_id))
        .expect_string()
        .build()
        .then_send(|result| Event::ClanSelectedLoaded(http_outcome(result)))
}

/// Load a wiki dataset (`/wows/encyclopedia/*`), skipping when it is already
/// loaded or a download is in flight.

pub(super) fn toggle_local_flag(model: &mut Model, key: String) -> Command<Effect, Event> {
    if !model.local_flags.remove(&key) {
        model.local_flags.insert(key);
    }
    rebuild_local_ship(model);
    render::render()
}

/// Set the simulated HP level (0..1) and rebuild conditional stats.

pub(super) fn set_local_hp(model: &mut Model, fraction: f64) -> Command<Effect, Event> {
    model.local_hp = fraction.clamp(0.0, 1.0);
    rebuild_local_ship(model);
    render::render()
}

/// Set whether the ship is considered spotted and rebuild trigger skills.

pub(super) fn set_local_spotted(model: &mut Model, spotted: bool) -> Command<Effect, Event> {
    model.local_spotted = spotted;
    rebuild_local_ship(model);
    render::render()
}


