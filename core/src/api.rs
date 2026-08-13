//! Port of `src/value/api.ts`: Wargaming/Wiki endpoint templates as builders.

use crate::data::Server;

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
        "https://api.worldofwarships.{}/wows/encyclopedia/ships/?application_id={api_key}&ship_id={ship_id}&language={language}",
        server.domain()
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
        "https://api.worldofwarships.{}/wows/encyclopedia/info/?application_id={api_key}&fields=ship_nations%2Cship_modules%2Cship_types&language={language}",
        server.domain()
    )
}

/// Paginated wiki ship list (subset used by the app).
#[must_use]
pub fn warship(server: Server, api_key: &str, page_no: u64, language: &str) -> String {
    format!(
        "https://api.worldofwarships.{}/wows/encyclopedia/ships/?application_id={api_key}&fields=name%2Cnation%2Ctype%2Ctier%2Cship_id%2Cship_id_str%2Cimages.small%2Cis_premium%2Cis_special&page_no={page_no}&language={language}",
        server.domain()
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
        "https://api.worldofwarships.{}/wows/encyclopedia/achievements/?application_id={api_key}&fields=battle.achievement_id%2Cbattle.name%2Cbattle.image&language={language}",
        server.domain()
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
        "https://api.worldofwarships.{}/wows/clans/accountinfo/?application_id={api_key}&extra=clan&fields=clan.tag&account_id={account_id}",
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
mod tests {
    use super::*;

    #[test]
    fn search_url_includes_server_and_key() {
        let url = player_search(Server::Asia, "KEY123", "henry");
        assert!(url.starts_with("https://api.worldofwarships.asia/wows/account/list/"));
        assert!(url.contains("application_id=KEY123"));
        assert!(url.ends_with("search=henry"));
    }

    #[test]
    fn ship_wiki_appends_language() {
        let url = ship_wiki(Server::Eu, "KEY", 1234, "zh-cn");
        assert!(url.contains("ship_id=1234"));
        assert!(url.ends_with("language=zh-cn"));
    }

    #[test]
    fn player_info_uses_com_domain() {
        let url = player_info(Server::Com, "KEY", 7);
        assert!(url.contains("worldofwarships.com"));
        assert!(url.contains("extra=statistics.pvp_div2"));
        assert!(url.contains("statistics.pvp_solo"));
    }

    #[test]
    fn rank_and_clan_endpoints_match_templates() {
        assert!(rank_info(Server::Ru, "K", 1).contains("/wows/seasons/accountinfo/"));
        assert!(player_clan(Server::Ru, "K", 1).contains("/wows/clans/accountinfo/"));
    }
}
