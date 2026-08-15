//! Player / server / search entry points.

use super::super::*;

pub(crate) fn init(model: &mut Model, config: Config) -> Command<Effect, Event> {
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


pub(crate) fn set_server(model: &mut Model, server: Server) -> Command<Effect, Event> {
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


pub(crate) fn search(model: &mut Model, query: String) -> Command<Effect, Event> {
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

pub(crate) fn search_clan(model: &mut Model, query: String) -> Command<Effect, Event> {
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

pub(crate) fn select_clan(model: &mut Model, clan_id: u64) -> Command<Effect, Event> {
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

pub(crate) fn select(model: &mut Model, account_id: u64) -> Command<Effect, Event> {
    model.phase = Phase::LoadingPlayer;
    model.pending_account_id = Some(account_id);
    model.pending_player = None;
    model.pending_ships = None;
    model.achievements.clear();
    model.clan_tag.clear();
    model.clan_id = None;
    model.clan = None;
    model.recent = None;
    model.rank = None;
    model.rank_ships.clear();

    let Some(config) = model.config.clone() else {
        model.phase = Phase::Error("App not initialised".to_string());
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        model.phase = Phase::Error(MISSING_KEY_MESSAGE.to_string());
        return render::render();
    }
    Command::all(player_commands(model, &key, account_id))
}


fn player_commands(model: &mut Model, key: &str, account_id: u64) -> Vec<Command<Effect, Event>> {
    let mut commands = vec![
        HttpCap::get(api::player_info(model.server, key, account_id))
            .expect_string()
            .build()
            .then_send(|result| Event::PlayerLoaded(http_outcome(result))),
        HttpCap::get(api::ship_info(model.server, key, account_id))
            .expect_string()
            .build()
            .then_send(|result| Event::ShipsLoaded(http_outcome(result))),
    ];
    if model.warship.is_empty() && !model.downloading_warship {
        model.downloading_warship = true;
        commands.push(
            HttpCap::get(api::warship(model.server, key, 1, &model.api_language))
                .expect_string()
                .build()
                .then_send(|result| Event::WarshipLoaded(http_outcome(result))),
        );
    }
    commands.push(
        HttpCap::get(api::rank_info(model.server, key, account_id))
            .expect_string()
            .build()
            .then_send(|result| Event::RankLoaded(http_outcome(result))),
    );
    commands.push(
        HttpCap::get(api::rank_ship_info(model.server, key, account_id))
            .expect_string()
            .build()
            .then_send(|result| Event::RankShipsLoaded(http_outcome(result))),
    );
    commands.push(
        HttpCap::get(api::player_achievement(model.server, key, account_id))
            .expect_string()
            .build()
            .then_send(|result| Event::AchievementsLoaded(http_outcome(result))),
    );
    commands.push(
        HttpCap::get(api::player_clan(model.server, key, account_id))
            .expect_string()
            .build()
            .then_send(|result| Event::ClanLoaded(http_outcome(result))),
    );
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    commands.push(
        HttpCap::get(api::stats_by_date(
            model.server,
            key,
            account_id,
            &downloader::recent_dates(now),
        ))
        .expect_string()
        .build()
        .then_send(|result| Event::RecentLoaded(http_outcome(result))),
    );
    commands.push(render::render());
    commands
}


pub(crate) fn refresh(model: &mut Model) -> Command<Effect, Event> {
    let Some(config) = model.config.clone() else {
        model.phase = Phase::Error("App not initialised".to_string());
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        model.phase = Phase::Error(MISSING_KEY_MESSAGE.to_string());
        return render::render();
    }

    let mut commands: Vec<Command<Effect, Event>> = vec![
        HttpCap::get(api::PERSONAL_RATING)
            .expect_string()
            .build()
            .then_send(|result| Event::PrLoaded(http_outcome(result))),
        TimeCap::now().then_send(|now| {
            Event::NowLoaded(
                now.duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0),
            )
        }),
    ];
    if model.warship.is_empty() && !model.downloading_warship {
        model.downloading_warship = true;
        commands.push(
            HttpCap::get(api::warship(model.server, &key, 1, &model.api_language))
                .expect_string()
                .build()
                .then_send(|result| Event::WarshipLoaded(http_outcome(result))),
        );
    }
    if let Some(account_id) = model.pending_account_id {
        commands.extend(player_commands(model, &key, account_id));
    }
    commands.push(render::render());
    Command::all(commands)
}

