//! Player / ship / achievements / recent response handlers.

use super::super::*;

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

