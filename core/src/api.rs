//! Port of `src/value/api.ts`: Wargaming/Wiki endpoint templates as builders.

use crate::data::Server;

/// Map the app's game-data language codes to the codes accepted by the WG API.
///
/// The bundled `lang.json` ships `en`/`ja`/`zh_sg`/`zh_tw`, but the
/// encyclopedia API only accepts `zh-cn`/`zh-tw` (and rejects `zh_sg` with
/// `INVALID_LANGUAGE`). Local parsing keeps the original codes; this is the
/// boundary where they are translated for outgoing URLs.
#[must_use]
pub fn api_language(language: &str) -> &str {
    match language {
        "zh_sg" => "zh-cn",
        "zh_tw" => "zh-tw",
        other => other,
    }
}

/// Player search results: `/wows/account/list/`.
#[must_use]
pub fn player_search(server: Server, api_key: &str, query: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/account/list/?application_id={api_key}&search={query}",
        server.domain()
    )
}

/// Player profile: `/wows/account/info/`.
#[must_use]
pub fn player_info(server: Server, api_key: &str, account_id: u64) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/account/info/?application_id={api_key}&account_id={account_id}&extra=statistics.pvp_div2%2Cstatistics.pvp_div3%2Cstatistics.pvp_solo%2Cstatistics.pve%2Cstatistics.rank_solo",
        server.domain()
    )
}

/// Players currently online: `/wgn/servers/info/`.
#[must_use]
pub fn player_online(server: Server, api_key: &str) -> String {
    format!(
        "https://api.worldoftanks.{}/wgn/servers/info/?application_id={api_key}&fields=players_online&game=wows",
        server.domain()
    )
}

/// Ship stats for a player: `/wows/ships/stats/`.
#[must_use]
pub fn ship_info(server: Server, api_key: &str, account_id: u64) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/ships/stats/?application_id={api_key}&account_id={account_id}",
        server.domain()
    )
}

/// One ship's wiki entry: `/wows/encyclopedia/ships/`.
#[must_use]
pub fn ship_wiki(server: Server, api_key: &str, ship_id: u64, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/ships/?application_id={api_key}&ship_id={ship_id}&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Current game version: `/wows/encyclopedia/info/` (`game_version`).
#[must_use]
pub fn game_version(server: Server, api_key: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/info/?application_id={api_key}&fields=game_version",
        server.domain()
    )
}

/// Supported languages: `/wows/encyclopedia/info/` (`languages`).
#[must_use]
pub fn language(server: Server, api_key: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/info/?application_id={api_key}&fields=languages",
        server.domain()
    )
}

/// Ship nations/types/modules: `/wows/encyclopedia/info/`.
#[must_use]
pub fn encyclopedia(server: Server, api_key: &str, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/info/?application_id={api_key}&fields=ship_nations%2Cship_modules%2Cship_types&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Paginated wiki ship list (subset used by the app).
#[must_use]
pub fn warship(server: Server, api_key: &str, page_no: u64, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/ships/?application_id={api_key}&fields=name%2Cnation%2Ctype%2Ctier%2Cship_id%2Cship_id_str%2Cimages.small%2Cis_premium%2Cis_special&page_no={page_no}&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Player achievements: `/wows/account/achievements/`.
#[must_use]
pub fn player_achievement(server: Server, api_key: &str, account_id: u64) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/account/achievements/?application_id={api_key}&language=en&fields=battle&account_id={account_id}",
        server.domain()
    )
}

/// Achievements encyclopedia (`name`/`image` per achievement).
#[must_use]
pub fn achievements_wiki(server: Server, api_key: &str, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/achievements/?application_id={api_key}&fields=battle.achievement_id%2Cbattle.name%2Cbattle.image&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Player stats by date (last 10 days) for the recent charts.
#[must_use]
pub fn stats_by_date(server: Server, api_key: &str, account_id: u64, dates: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/account/statsbydate/?application_id={api_key}&account_id={account_id}&dates={dates}",
        server.domain()
    )
}

/// Player clan: `/wows/clans/accountinfo/`.
#[must_use]
pub fn player_clan(server: Server, api_key: &str, account_id: u64) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/clans/accountinfo/?application_id={api_key}&extra=clan&fields=clan.id%2Cclan.tag&account_id={account_id}",
        server.domain()
    )
}

/// Clan search: `/wows/clans/list/`.
#[must_use]
pub fn clan_search(server: Server, api_key: &str, query: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/clans/list/?application_id={api_key}&fields=clan_id%2Ctag&search={query}",
        server.domain()
    )
}

/// Full clan info with members: `/wows/clans/info/`.
#[must_use]
pub fn clan_info(server: Server, api_key: &str, clan_id: u64) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/clans/info/?application_id={api_key}&extra=members&fields=-members_ids&clan_id={clan_id}",
        server.domain()
    )
}

/// Ranked stats: `/wows/seasons/accountinfo/`.
#[must_use]
pub fn rank_info(server: Server, api_key: &str, account_id: u64) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/seasons/accountinfo/?application_id={api_key}&account_id={account_id}",
        server.domain()
    )
}

/// Ranked ship stats: `/wows/seasons/shipstats/`.
#[must_use]
pub fn rank_ship_info(server: Server, api_key: &str, account_id: u64) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/seasons/shipstats/?application_id={api_key}&account_id={account_id}",
        server.domain()
    )
}

/// Wiki collections: `/wows/encyclopedia/collections/`.
#[must_use]
pub fn collections(server: Server, api_key: &str, page_no: u64, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/collections/?application_id={api_key}&fields=-card_cost%2C-tag&page_no={page_no}&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Wiki collection cards: `/wows/encyclopedia/collectioncards/`.
#[must_use]
pub fn collection_cards(server: Server, api_key: &str, page_no: u64, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/collectioncards/?application_id={api_key}&fields=images.small%2Ccard_id%2Ccollection_id%2Cdescription%2Cname&page_no={page_no}&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Wiki consumables: `/wows/encyclopedia/consumables/`.
#[must_use]
pub fn consumables(server: Server, api_key: &str, page_no: u64, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/consumables/?application_id={api_key}&fields=type%2Cconsumable_id%2Cdescription%2Cname%2Cimage%2Cprice_credit%2Cprice_gold%2Cprofile.description&page_no={page_no}&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Commander skills: `/wows/encyclopedia/crewskills/`.
#[must_use]
pub fn commander_skills(server: Server, api_key: &str, page_no: u64, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/crewskills/?application_id={api_key}&page_no={page_no}&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Wiki battle arenas / maps: `/wows/encyclopedia/battlearenas/`.
#[must_use]
pub fn battle_arenas(server: Server, api_key: &str, page_no: u64, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/battlearenas/?application_id={api_key}&fields=name%2Cicon%2Cdescription&page_no={page_no}&language={}",
        server.domain(),
        api_language(language)
    )
}

/// Remote personal-rating table (mirrored on GitHub).
pub const PERSONAL_RATING: &str =
    "https://raw.githubusercontent.com/HenryQuan/WoWs-Info-Origin/API/json/personal_rating.json";

/// Ship model data (GitHub).
pub const SHIP_MODEL: &str =
    "https://raw.githubusercontent.com/HenryQuan/WoWs-Info-Ultra/API/json/model.json";

/// App version metadata (GitHub).
pub const GITHUB_APP_VERSION: &str =
    "https://raw.githubusercontent.com/HenryQuan/WoWs-Info-Origin/API/json/app.json";

#[cfg(test)]
mod tests;
