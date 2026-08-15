//! The Crux `App` type: event dispatch and view projection.

use crux_core::{render, App as AppTrait, Command};

use super::*;

/// The application.
#[derive(Default)]
pub struct App;

impl AppTrait for App {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::Init(config) => init(model, config),
            Event::SetServer(server) => set_server(model, server),
            Event::SearchPlayer { query } => search(model, query),
            Event::SelectPlayer { account_id } => select(model, account_id),
            Event::Refresh => refresh(model),
            Event::SearchClan { query } => search_clan(model, query),
            Event::SelectClan { clan_id } => select_clan(model, clan_id),
            Event::LoadWiki { dataset } => load_wiki(model, dataset),
            Event::LoadWarship => load_warship(model),
            Event::LoadShipWiki { ship_id } => load_ship_wiki(model, ship_id),
            Event::SetLocalData { ships, lang } => set_local_data(model, ships, lang),
            Event::LoadLocalWarships => load_local_warships(model),
            Event::LoadLocalShipWiki { ship_id } => load_local_ship_wiki(model, ship_id),
            Event::SelectLocalShipModule { slot, index } => {
                select_local_ship_module(model, slot, index)
            }
            Event::LoadLocalCompare { ship_ids } => load_local_compare(model, ship_ids),
            Event::ToggleLocalSkill { key } => toggle_local_skill(model, key),
            Event::ToggleLocalUpgrade { key } => toggle_local_upgrade(model, key),
            Event::ToggleLocalFlag { key } => toggle_local_flag(model, key),
            Event::SetLocalHp { fraction } => set_local_hp(model, fraction),
            Event::SetLocalSpotted { spotted } => set_local_spotted(model, spotted),
            Event::SetLanguage { language } => set_language(model, language),
            Event::ServerSaved => render::render(),
            Event::NowLoaded(_) => render::render(),
            Event::GameVersionLoaded(outcome) => on_game_version_loaded(model, outcome),
            Event::SearchLoaded(outcome) => on_search_loaded(model, outcome),
            Event::PlayerLoaded(outcome) => on_player_loaded(model, outcome),
            Event::ShipsLoaded(outcome) => on_ships_loaded(model, outcome),
            Event::WarshipLoaded(outcome) => on_warship_loaded(model, outcome),
            Event::PrLoaded(outcome) => on_pr_loaded(model, outcome),
            Event::AchievementsLoaded(outcome) => on_achievements_loaded(model, outcome),
            Event::AchievementsWikiLoaded(outcome) => on_achievements_wiki_loaded(model, outcome),
            Event::ClanLoaded(outcome) => on_clan_loaded(model, outcome),
            Event::ClanInfoLoaded(outcome) => on_clan_info_loaded(model, outcome),
            Event::ClanSearchLoaded(outcome) => on_clan_search_loaded(model, outcome),
            Event::ClanSelectedLoaded(outcome) => on_clan_selected_loaded(model, outcome),
            Event::RecentLoaded(outcome) => on_recent_loaded(model, outcome),
            Event::RankLoaded(outcome) => on_rank_loaded(model, outcome),
            Event::RankShipsLoaded(outcome) => on_rank_ships_loaded(model, outcome),
            Event::OnlineLoaded(outcome) => on_online_loaded(model, outcome),
            Event::WikiLoaded { dataset, outcome } => on_wiki_loaded(model, dataset, outcome),
            Event::ShipWikiLoaded(outcome) => on_ship_wiki_loaded(model, outcome),
            Event::KvLoaded { key, value } => on_kv_loaded(model, key, value),
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            phase: model.phase.clone(),
            search_results: model
                .search_results
                .iter()
                .map(|r| SearchResult {
                    account_id: r.account_id,
                    nickname: r.nickname.clone(),
                })
                .collect(),
            player: model.selected.clone(),
            clan_search_results: model.clan_search_results.clone(),
            selected_clan: model.selected_clan.clone(),
            online: model.online,
            warship: model.warship.clone(),
            wiki_collections: model.wiki_collections.clone(),
            wiki_collection_cards: model.wiki_collection_cards.clone(),
            wiki_consumables: model.wiki_consumables.clone(),
            wiki_commander_skills: model.wiki_commander_skills.clone(),
            wiki_maps: model.wiki_maps.clone(),
            selected_ship_wiki: model.selected_ship_wiki.clone(),
            local_ship: model.local_ship.clone(),
            local_compare: model.local_compare.clone(),
            local_consumables: model.local_consumables.clone(),
            local_skills_wiki: model.local_skills_wiki.clone(),
            local_flags_wiki: model.local_flags_wiki.clone(),
            local_achievements: model.local_achievements.clone(),
            local_upgrades_wiki: model.local_upgrades_wiki.clone(),
            local_data_ready: model.local_data.is_some(),
            units: wiki::LocalizedUnits::from_lang(&model.local_lang),
        }
    }
}

