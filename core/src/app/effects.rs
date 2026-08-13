//! Handlers for capability responses (HTTP, KV, time) and player assembly.

use super::*;

pub(super) fn on_search_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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

pub(super) fn on_player_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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

pub(super) fn on_ships_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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

pub(super) fn on_warship_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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

pub(super) fn on_pr_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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

pub(super) fn on_achievements_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(account_id) = model.pending_account_id {
                model.achievements =
                    downloader::parse_achievements(&json, account_id, &model.achievements_wiki);
            }
        }
        HttpOutcome::Err { message } => {
            model.phase = Phase::Error(format!("Achievements failed: {message}"));
        }
    }
    try_assemble(model)
}

pub(super) fn on_achievements_wiki_loaded(
    model: &mut Model,
    outcome: HttpOutcome,
) -> Command<Effect, Event> {
    model.downloading_achievements = false;
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            model.achievements_wiki = downloader::parse_achievements_wiki(&json);
            if !model.achievements_wiki.is_empty() {
                if let Ok(json) = serde_json::to_string(&model.achievements_wiki) {
                    return Command::all([
                        kv_set_event(data::saved::ACHIEVEMENT, json),
                        try_assemble(model),
                    ]);
                }
            }
        }
        HttpOutcome::Err { message } => {
            model.phase = Phase::Error(format!("Achievements wiki failed: {message}"));
        }
    }
    try_assemble(model)
}

pub(super) fn on_clan_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    let mut commands = Vec::new();
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(account_id) = model.pending_account_id {
                model.clan_tag = downloader::parse_clan_tag(&json, account_id);
                model.clan_id = downloader::parse_clan_id(&json, account_id);
            }
        }
        HttpOutcome::Err { .. } => {}
    }
    // Full clan info needs the clan id from the account lookup, mirroring the
    // app's tag-first-then-info flow.
    if model.clan.is_none() {
        if let (Some(clan_id), Some(config)) = (model.clan_id, model.config.clone()) {
            let key = api_key(&config);
            if !key.is_empty() {
                commands.push(
                    HttpCap::get(api::clan_info(model.server, &key, clan_id))
                        .expect_string()
                        .build()
                        .then_send(|result| Event::ClanInfoLoaded(http_outcome(result))),
                );
            }
        }
    }
    commands.push(try_assemble(model));
    Command::all(commands)
}

pub(super) fn on_clan_info_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            if let Some(clan_id) = model.clan_id {
                model.clan = downloader::parse_clan_info(&json, clan_id);
            }
        }
        HttpOutcome::Err { .. } => {}
    }
    try_assemble(model)
}

pub(super) fn on_clan_search_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            model.clan_search_results = downloader::parse_clan_search(&json);
        }
        HttpOutcome::Err { .. } => {}
    }
    render::render()
}

pub(super) fn on_clan_selected_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            let clan_id = model
                .selected_clan
                .as_ref()
                .map(|clan| clan.clan_id)
                .unwrap_or(0);
            let parsed = downloader::parse_clan_info(&json, clan_id);
            // The response carries the id; fall back to the requested one.
            let parsed = parsed.or_else(|| {
                json.get("data")
                    .and_then(serde_json::Value::as_object)
                    .and_then(|data| data.values().next())
                    .and_then(|first| {
                        first
                            .get("clan_id")
                            .and_then(serde_json::Value::as_u64)
                            .map(|id| downloader::parse_clan_info(&json, id))
                            .flatten()
                    })
            });
            if let Some(clan) = parsed {
                model.selected_clan = Some(clan);
            }
        }
        HttpOutcome::Err { .. } => {}
    }
    render::render()
}

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

pub(super) fn on_game_version_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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
fn fetch_achievements_wiki(model: &mut Model) -> Command<Effect, Event> {
    if model.downloading_achievements {
        return render::render();
    }
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    model.downloading_achievements = true;
    HttpCap::get(api::achievements_wiki(model.server, &key, &model.api_language))
        .expect_string()
        .build()
        .then_send(|result| Event::AchievementsWikiLoaded(http_outcome(result)))
}

fn try_assemble(model: &mut Model) -> Command<Effect, Event> {
    if model.downloading_warship {
        // The ship encyclopedia is still downloading; keep the loading phase
        // until the final page so the view isn't assembled from partial data.
        return render::render();
    }
    if let (Some(player), Some(ships)) = (model.pending_player.clone(), model.pending_ships.clone())
    {
        let view = downloader::assemble_player(
            player,
            ships,
            &model.pr,
            &model.warship,
            model.server,
            model.clan_tag.clone(),
            model.achievements.clone(),
            model.recent.clone(),
            model.rank.clone(),
            model.rank_ships.clone(),
            model.clan.clone(),
        );
        model.selected = Some(view);
        model.phase = Phase::Player;
    }
    render::render()
}
