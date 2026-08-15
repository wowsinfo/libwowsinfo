//! Rank / online / wiki / version / KV response handlers.

use super::super::*;

pub(super) fn on_rank_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(account_id) = model.pending_account_id {
                model.rank = downloader::parse_rank_info(&json, account_id);
            }
        }
        HttpOutcome::Err { .. } => {}
    }
    try_assemble(model)
}


pub(super) fn on_rank_ships_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(account_id) = model.pending_account_id {
                model.rank_ships = downloader::parse_rank_ship_stats(&json, account_id);
            }
        }
        HttpOutcome::Err { .. } => {}
    }
    try_assemble(model)
}


pub(super) fn on_online_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            model.online = downloader::parse_online_count(&json);
        }
        HttpOutcome::Err { .. } => {}
    }
    render::render()
}


pub(super) fn on_wiki_loaded(
    model: &mut Model,
    dataset: WikiDataset,
    outcome: HttpOutcome,
) -> Command<Effect, Event> {
    let mut commands = Vec::new();
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            match dataset {
                WikiDataset::Collections => {
                    model
                        .wiki_collections
                        .extend(downloader::parse_collections(&json));
                }
                WikiDataset::CollectionCards => {
                    model
                        .wiki_collection_cards
                        .extend(downloader::parse_collection_cards(&json));
                }
                WikiDataset::Consumables => {
                    model
                        .wiki_consumables
                        .extend(downloader::parse_consumables(&json));
                }
                WikiDataset::CommanderSkills => {
                    model
                        .wiki_commander_skills
                        .extend(downloader::parse_commander_skills(&json));
                }
                WikiDataset::Maps => {
                    model.wiki_maps.extend(downloader::parse_maps(&json));
                }
            }

            // The encyclopedia is paginated; keep fetching until the last page.
            let empty = serde_json::Value::Object(Default::default());
            let meta = downloader::guard(&json, "meta", &empty);
            let page = meta
                .get("page")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(1);
            let page_total = meta
                .get("page_total")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(1);
            let can_fetch = model
                .config
                .as_ref()
                .is_some_and(|config| !api_key(config).is_empty());
            if page < page_total && can_fetch {
                let config = model.config.clone().expect("checked above");
                let key = api_key(&config);
                commands.push(
                    HttpCap::get(super::handlers::wiki_url(
                        dataset,
                        model.server,
                        &key,
                        page + 1,
                        &model.api_language,
                    ))
                    .expect_string()
                    .build()
                    .then_send(move |result| Event::WikiLoaded {
                        dataset,
                        outcome: http_outcome(result),
                    }),
                );
            } else {
                model.downloading_wiki.remove(&dataset);
            }
        }
        HttpOutcome::Err { .. } => {
            model.downloading_wiki.remove(&dataset);
        }
    }
    commands.push(render::render());
    Command::all(commands)
}


pub(super) fn on_ship_wiki_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(ship_id) = model.pending_ship_wiki_id {
                model.selected_ship_wiki = downloader::parse_ship_wiki(&json, ship_id);
            }
        }
        HttpOutcome::Err { .. } => {}
    }
    render::render()
}


pub(super) fn on_recent_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(account_id) = model.pending_account_id {
                model.recent = downloader::parse_recent_overview(&json, account_id);
            }
        }
        // The `statsbydate` endpoint is no longer served by the API; recent
        // charts simply stay hidden until a data source is available.
        HttpOutcome::Err { .. } => {}
    }
    try_assemble(model)
}


pub(super) fn on_kv_loaded(model: &mut Model, key: String, value: KvOutcome) -> Command<Effect, Event> {
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
                if !model.language_overridden
                    && let Ok(language) = serde_json::from_str::<String>(&json)
                {
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
            data::saved::ACHIEVEMENT => {
                if let Ok(wiki) =
                    serde_json::from_str::<HashMap<String, models::EncyclopediaAchievement>>(&json)
                {
                    model.achievements_wiki = wiki;
                }
            }
            _ => {}
        },
        KvOutcome::Ok { value: None } | KvOutcome::Err { .. } => {
            if key == data::saved::PR && model.pr.is_empty() {
                // Bundled table is the fallback when nothing is cached yet.
                model.pr = downloader::local_pr();
            }
            if key == data::saved::ACHIEVEMENT && model.achievements_wiki.is_empty() {
                return fetch_achievements_wiki(model);
            }
        }
    }
    render::render()
}

/// Fetch (and cache) the achievements encyclopedia when nothing is cached yet.

