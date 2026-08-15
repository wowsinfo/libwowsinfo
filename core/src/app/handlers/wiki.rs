//! Wiki (encyclopedia) request entry points.

use super::super::*;

pub(super) fn wiki_url(
    dataset: WikiDataset,
    server: Server,
    key: &str,
    page: u64,
    language: &str,
) -> String {
    match dataset {
        WikiDataset::Collections => api::collections(server, key, page, language),
        WikiDataset::CollectionCards => api::collection_cards(server, key, page, language),
        WikiDataset::Consumables => api::consumables(server, key, page, language),
        WikiDataset::CommanderSkills => api::commander_skills(server, key, page, language),
        WikiDataset::Maps => api::battle_arenas(server, key, page, language),
    }
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

