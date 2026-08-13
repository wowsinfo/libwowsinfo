//! Event entry points: initialisation, server selection, search and refresh.

use super::*;

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
    }
}

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
pub(super) fn load_wiki(model: &mut Model, dataset: WikiDataset) -> Command<Effect, Event> {
    let loaded = match dataset {
        WikiDataset::Collections => !model.wiki_collections.is_empty(),
        WikiDataset::CollectionCards => !model.wiki_collection_cards.is_empty(),
        WikiDataset::Consumables => !model.wiki_consumables.is_empty(),
        WikiDataset::CommanderSkills => !model.wiki_commander_skills.is_empty(),
    };
    if loaded || model.downloading_wiki.contains(&dataset) {
        return render::render();
    }
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    model.downloading_wiki.insert(dataset);
    HttpCap::get(wiki_url(dataset, model.server, &key, 1, &model.api_language))
        .expect_string()
        .build()
        .then_send(move |result| Event::WikiLoaded {
            dataset,
            outcome: http_outcome(result),
        })
}

/// Load the paginated ship encyclopedia, skipping when already loaded or a
/// download is in flight (used by the wiki ships tab).
pub(super) fn load_warship(model: &mut Model) -> Command<Effect, Event> {
    if !model.warship.is_empty() || model.downloading_warship {
        return render::render();
    }
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    model.downloading_warship = true;
    HttpCap::get(api::warship(model.server, &key, 1, &model.api_language))
        .expect_string()
        .build()
        .then_send(|result| Event::WarshipLoaded(http_outcome(result)))
}

/// Load one ship's full wiki entry (`/wows/encyclopedia/ships/?ship_id=`).
pub(super) fn load_ship_wiki(model: &mut Model, ship_id: u64) -> Command<Effect, Event> {
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    model.pending_ship_wiki_id = Some(ship_id);
    HttpCap::get(api::ship_wiki(model.server, &key, ship_id, &model.api_language))
        .expect_string()
        .build()
        .then_send(|result| Event::ShipWikiLoaded(http_outcome(result)))
}

/// Parse the bundled `wowsinfo.json` and `lang.json` into memory (local mode).
pub(super) fn set_local_data(
    model: &mut Model,
    ships: String,
    lang: String,
) -> Command<Effect, Event> {
    let ships_json = serde_json::from_str(&ships).unwrap_or_default();
    let lang_json = serde_json::from_str(&lang).unwrap_or_default();
    model.local_data = Some(wiki::parse_game_data(&ships_json));
    model.local_lang = wiki::parse_lang(&lang_json, &model.api_language);
    model.raw_lang_json = Some(lang);
    refresh_local_lang(model);
    model.local_selection = wiki::ModuleSelection::default();
    model.local_skills.clear();
    model.local_upgrades.clear();
    model.local_flags.clear();
    model.local_hp = 1.0;
    model.local_spotted = false;
    model.local_ship_id = None;
    model.local_ship = None;
    render::render()
}

/// Rebuild everything that depends on the language map after it changes.
fn refresh_local_lang(model: &mut Model) {
    if let Some(raw) = &model.raw_lang_json {
        let lang_json = serde_json::from_str(raw).unwrap_or_default();
        model.local_lang = wiki::parse_lang(&lang_json, &model.api_language);
    }
    if let Some(data) = &model.local_data {
        model.local_consumables = wiki::all_consumable_views(data, &model.local_lang);
        model.local_skills_wiki = wiki::all_skill_views(data, &model.local_lang);
    }
    fill_local_warships(model);
    rebuild_local_ship(model);
}

/// Fill the warship encyclopedia from the local game data (offline ship list).
fn fill_local_warships(model: &mut Model) {
    if let Some(data) = &model.local_data {
        model.warship = data
            .ships
            .iter()
            .map(|(id, ship)| {
                (
                    *id,
                    models::EncyclopediaShip {
                        ship_id: *id,
                        name: model.local_lang.get(&ship.name),
                        nation: ship.region.clone(),
                        r#type: ship.r#type.clone(),
                        tier: ship.tier,
                        premium: ship.group == "special",
                        icon: String::new(),
                        new: None,
                        model: None,
                    },
                )
            })
            .collect();
    }
}

/// Change the interface/data language and persist it.
pub(super) fn set_language(model: &mut Model, language: String) -> Command<Effect, Event> {
    model.api_language = if language.is_empty() {
        data::DEFAULT_USER_LANGUAGE.to_string()
    } else {
        language
    };
    if let Some(config) = model.config.as_mut() {
        config.language = model.api_language.clone();
    }
    refresh_local_lang(model);
    let value = serde_json::to_string(&model.api_language).unwrap_or_default();
    Command::all([
        kv_set_event(data::local::USER_LANGUAGE, value),
        render::render(),
    ])
}

fn local_build_config(model: &Model) -> wiki::LocalBuildConfig {
    wiki::LocalBuildConfig {
        skills: model.local_skills.clone(),
        upgrades: model.local_upgrades.clone(),
        flags: model.local_flags.clone(),
        hp_fraction: model.local_hp,
        spotted: model.local_spotted,
    }
}

fn rebuild_local_ship(model: &mut Model) {
    if let (Some(data), Some(ship_id)) = (&model.local_data, model.local_ship_id) {
        model.local_ship = wiki::build_local_ship_wiki(
            data,
            &model.local_lang,
            ship_id,
            model.local_selection,
            &local_build_config(model),
        );
    }
}

/// Fill the warship encyclopedia from the local game data (offline ship list).
pub(super) fn load_local_warships(model: &mut Model) -> Command<Effect, Event> {
    if !model.warship.is_empty() {
        return render::render();
    }
    fill_local_warships(model);
    render::render()
}

/// Build the local wiki entry for one ship from `wowsinfo.json`.
pub(super) fn load_local_ship_wiki(model: &mut Model, ship_id: u64) -> Command<Effect, Event> {
    model.local_ship_id = Some(ship_id);
    rebuild_local_ship(model);
    render::render()
}

/// Apply a module slot selection and rebuild the selected local ship.
pub(super) fn select_local_ship_module(
    model: &mut Model,
    slot: String,
    index: i64,
) -> Command<Effect, Event> {
    let index = index.max(0) as usize;
    match slot.as_str() {
        "hull" => model.local_selection.hull = index,
        "artillery" => model.local_selection.artillery = index,
        "torpedoes" => model.local_selection.torpedoes = index,
        "fire_control" | "flight_control" => model.local_selection.fire_control = index,
        "engine" => model.local_selection.engine = index,
        "fighter" => model.local_selection.fighter = index,
        "torpedo_bomber" => model.local_selection.torpedo_bomber = index,
        "dive_bomber" => model.local_selection.dive_bomber = index,
        "skip_bomber" => model.local_selection.skip_bomber = index,
        _ => return render::render(),
    }
    rebuild_local_ship(model);
    render::render()
}

/// Build the local comparison table for a list of ship ids.
pub(super) fn load_local_compare(
    model: &mut Model,
    ship_ids: Vec<u64>,
) -> Command<Effect, Event> {
    if let Some(data) = &model.local_data {
        model.local_compare = wiki::build_local_compare(data, &model.local_lang, &ship_ids);
    }
    render::render()
}

/// Toggle a selected commander skill and rebuild the ship stats.
pub(super) fn toggle_local_skill(model: &mut Model, key: String) -> Command<Effect, Event> {
    if !model.local_skills.remove(&key) {
        model.local_skills.insert(key);
    }
    rebuild_local_ship(model);
    render::render()
}

/// Toggle a selected module upgrade and rebuild the ship stats.
pub(super) fn toggle_local_upgrade(model: &mut Model, key: String) -> Command<Effect, Event> {
    if !model.local_upgrades.remove(&key) {
        model.local_upgrades.insert(key);
    }
    rebuild_local_ship(model);
    render::render()
}

/// Toggle a selected signal flag and rebuild the ship stats.
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

pub(super) fn select(model: &mut Model, account_id: u64) -> Command<Effect, Event> {
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

pub(super) fn refresh(model: &mut Model) -> Command<Effect, Event> {
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
