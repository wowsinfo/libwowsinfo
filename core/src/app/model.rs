//! Internal state of the app.

use std::collections::{HashMap, HashSet};

use super::config::Config;
use super::datasets::WikiDataset;
use super::view::Phase;
use crate::data::Server;
use crate::models;
use crate::wiki;

/// Internal state of the app.
#[derive(Default)]
pub struct Model {
    pub(crate) config: Option<Config>,
    pub(crate) server: Server,
    pub(crate) api_language: String,
    pub(crate) game_version: Option<String>,
    pub(crate) warship: HashMap<u64, models::EncyclopediaShip>,
    pub(crate) pr: HashMap<u64, models::PrEntry>,
    pub(crate) phase: Phase,
    pub(crate) search_results: Vec<models::AccountListEntry>,
    pub(crate) pending_account_id: Option<u64>,
    pub(crate) pending_player: Option<models::PlayerInfo>,
    pub(crate) pending_ships: Option<Vec<models::ShipStats>>,
    pub(crate) selected: Option<models::PlayerView>,
    pub(crate) achievements: Vec<models::Achievement>,
    pub(crate) achievements_wiki: HashMap<String, models::EncyclopediaAchievement>,
    pub(crate) clan_tag: String,
    pub(crate) recent: Option<models::RecentOverview>,
    pub(crate) rank: Option<models::RankPlayerInfo>,
    pub(crate) rank_ships: Vec<models::RankShipStat>,
    pub(crate) clan: Option<models::ClanInfo>,
    pub(crate) clan_id: Option<u64>,
    pub(crate) clan_search_results: Vec<models::ClanSearchResult>,
    pub(crate) selected_clan: Option<models::ClanInfo>,
    pub(crate) online: i64,
    pub(crate) wiki_collections: HashMap<u64, models::WikiCollection>,
    pub(crate) wiki_collection_cards: HashMap<u64, models::CollectionCard>,
    pub(crate) wiki_consumables: HashMap<u64, models::Consumable>,
    pub(crate) wiki_commander_skills: HashMap<u64, models::CommanderSkill>,
    pub(crate) wiki_maps: HashMap<u64, models::WikiMap>,
    pub(crate) downloading_wiki: HashSet<WikiDataset>,
    pub(crate) selected_ship_wiki: Option<models::ShipWiki>,
    pub(crate) pending_ship_wiki_id: Option<u64>,
    pub(crate) local_data: Option<wiki::GameData>,
    pub(crate) local_lang: wiki::LangMap,
    pub(crate) local_selection: wiki::ModuleSelection,
    pub(crate) local_ship: Option<wiki::LocalShipWiki>,
    pub(crate) local_ship_id: Option<u64>,
    pub(crate) local_compare: Option<wiki::LocalCompare>,
    pub(crate) local_skills: HashSet<String>,
    pub(crate) local_upgrades: HashSet<String>,
    pub(crate) local_flags: HashSet<String>,
    pub(crate) local_hp: f64,
    pub(crate) local_spotted: bool,
    pub(crate) local_consumables: Vec<wiki::ConsumableView>,
    pub(crate) local_skills_wiki: Vec<wiki::LocalSkillWikiEntry>,
    pub(crate) local_achievements: Vec<wiki::LocalAchievementEntry>,
    pub(crate) local_upgrades_wiki: Vec<wiki::LocalUpgradeEntry>,
    pub(crate) local_flags_wiki: Vec<wiki::LocalFlagEntry>,
    /// Raw `lang.json` string so the language can be re-parsed on change.
    pub(crate) raw_lang_json: Option<String>,
    /// True once the user picked a language this session; the init restore
    /// must not overwrite it.
    pub(crate) language_overridden: bool,
    pub(crate) downloading_achievements: bool,
    /// True while the paginated ship encyclopedia download is in progress.
    pub(crate) downloading_warship: bool,
}
