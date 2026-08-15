//! URL-builder tests.

use super::*;

#[test]
fn api_language_maps_game_codes_to_wg_codes() {
    assert_eq!(api_language("en"), "en");
    assert_eq!(api_language("ja"), "ja");
    assert_eq!(api_language("zh_sg"), "zh-cn");
    assert_eq!(api_language("zh_tw"), "zh-tw");
}

#[test]
fn wiki_urls_use_wg_language_codes() {
    assert!(collections(Server::Eu, "K", 1, "zh_sg").ends_with("language=zh-cn"));
    assert!(battle_arenas(Server::Eu, "K", 1, "zh_tw").ends_with("language=zh-tw"));
    assert!(warship(Server::Eu, "K", 1, "zh_sg").contains("language=zh-cn"));
    assert!(ship_wiki(Server::Eu, "K", 1, "zh_sg").ends_with("language=zh-cn"));
}

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
    assert!(player_clan(Server::Ru, "K", 1).contains("clan.id"));
    assert!(clan_info(Server::Eu, "K", 5).contains("/wows/clans/info/"));
    assert!(clan_info(Server::Eu, "K", 5).contains("clan_id=5"));
}

#[test]
fn migrated_wiki_endpoints_match_templates() {
    assert!(collections(Server::Eu, "K", 1, "en").contains("/wows/encyclopedia/collections/"));
    assert!(collection_cards(Server::Eu, "K", 1, "en")
        .contains("/wows/encyclopedia/collectioncards/"));
    assert!(consumables(Server::Eu, "K", 1, "en")
        .contains("/wows/encyclopedia/consumables/"));
    assert!(commander_skills(Server::Eu, "K", 1, "en")
        .contains("/wows/encyclopedia/crewskills/"));
}
