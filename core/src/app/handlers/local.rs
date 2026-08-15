//! Local (bundled data) entry points.

use super::super::*;

pub(crate) fn set_local_data(
    model: &mut Model,
    ships: Vec<u8>,
    lang: Vec<u8>,
) -> Command<Effect, Event> {
    let ships_text = wiki::decompress_zstd(&ships).unwrap_or_default();
    let lang_text = wiki::decompress_zstd(&lang).unwrap_or_default();
    let ships_json = serde_json::from_str(&ships_text).unwrap_or_default();
    let lang_json = serde_json::from_str(&lang_text).unwrap_or_default();
    model.local_data = Some(wiki::parse_game_data(&ships_json));
    model.local_lang = wiki::parse_lang(&lang_json, &model.api_language);
    model.raw_lang_json = Some(lang_text);
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
        model.local_achievements = wiki::all_achievement_views(data, &model.local_lang);
        model.local_upgrades_wiki = wiki::all_upgrade_views(data, &model.local_lang);
        model.local_flags_wiki = wiki::all_flag_views(data, &model.local_lang);
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
                        index: ship.index.clone(),
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

pub(crate) fn set_language(model: &mut Model, language: String) -> Command<Effect, Event> {
    model.api_language = if language.is_empty() {
        data::DEFAULT_USER_LANGUAGE.to_string()
    } else {
        language
    };
    model.language_overridden = true;
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

pub(crate) fn load_local_warships(model: &mut Model) -> Command<Effect, Event> {
    if !model.warship.is_empty() {
        return render::render();
    }
    fill_local_warships(model);
    render::render()
}

/// Build the local wiki entry for one ship from `wowsinfo.json`.

pub(crate) fn load_local_ship_wiki(model: &mut Model, ship_id: u64) -> Command<Effect, Event> {
    model.local_ship_id = Some(ship_id);
    rebuild_local_ship(model);
    render::render()
}

/// Apply a module slot selection and rebuild the selected local ship.

pub(crate) fn select_local_ship_module(
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

pub(crate) fn load_local_compare(
    model: &mut Model,
    ship_ids: Vec<u64>,
) -> Command<Effect, Event> {
    if let Some(data) = &model.local_data {
        model.local_compare = wiki::build_local_compare(data, &model.local_lang, &ship_ids);
    }
    render::render()
}

/// Toggle a selected commander skill and rebuild the ship stats.

pub(crate) fn toggle_local_skill(model: &mut Model, key: String) -> Command<Effect, Event> {
    if !model.local_skills.remove(&key) {
        model.local_skills.insert(key);
    }
    rebuild_local_ship(model);
    render::render()
}

/// Toggle a selected module upgrade and rebuild the ship stats.

pub(crate) fn toggle_local_upgrade(model: &mut Model, key: String) -> Command<Effect, Event> {
    if !model.local_upgrades.remove(&key) {
        model.local_upgrades.insert(key);
    }
    rebuild_local_ship(model);
    render::render()
}

/// Toggle a selected signal flag and rebuild the ship stats.

pub(crate) fn toggle_local_flag(model: &mut Model, key: String) -> Command<Effect, Event> {
    if !model.local_flags.remove(&key) {
        model.local_flags.insert(key);
    }
    rebuild_local_ship(model);
    render::render()
}

/// Set the simulated HP level (0..1) and rebuild conditional stats.

pub(crate) fn set_local_hp(model: &mut Model, fraction: f64) -> Command<Effect, Event> {
    model.local_hp = fraction.clamp(0.0, 1.0);
    rebuild_local_ship(model);
    render::render()
}

/// Set whether the ship is considered spotted and rebuild trigger skills.

pub(crate) fn set_local_spotted(model: &mut Model, spotted: bool) -> Command<Effect, Event> {
    model.local_spotted = spotted;
    rebuild_local_ship(model);
    render::render()
}


