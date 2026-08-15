//! Clan response handlers.

use super::super::*;

use super::player::try_assemble;

pub(crate) fn on_clan_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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
    if model.clan.is_none()
        && let (Some(clan_id), Some(config)) = (model.clan_id, model.config.clone()) {
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
    commands.push(try_assemble(model));
    Command::all(commands)
}


pub(crate) fn on_clan_info_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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


pub(crate) fn on_clan_search_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
    match outcome {
        HttpOutcome::Ok { body } => {
            let json = serde_json::from_str(&body).unwrap_or_default();
            model.clan_search_results = downloader::parse_clan_search(&json);
        }
        HttpOutcome::Err { .. } => {}
    }
    render::render()
}


pub(crate) fn on_clan_selected_loaded(model: &mut Model, outcome: HttpOutcome) -> Command<Effect, Event> {
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
                            .and_then(|id| downloader::parse_clan_info(&json, id))
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


