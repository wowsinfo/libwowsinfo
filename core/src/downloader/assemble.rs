//! Player-view assembly (rating, charts, stats).

use std::collections::HashMap;

use crate::models::{Achievement, ClanInfo, EncyclopediaShip, PlayerInfo, PlayerStatistics, PlayerView, PrEntry, RankPlayerInfo, RankShipStat, RecentOverview, ShipStatLine, ShipStats};
use crate::rating::{get_ap, get_colour, get_comment, get_overall_rating};


/// Assemble the player view shown by the stats screen: compute ratings from
/// ship stats + PR table and attach wiki names.
#[must_use]
pub fn assemble_player(
    player: PlayerInfo,
    mut ships: Vec<ShipStats>,
    pr: &HashMap<u64, PrEntry>,
    warship: &HashMap<u64, EncyclopediaShip>,
    server: crate::data::Server,
    clan_tag: String,
    achievements: Vec<Achievement>,
    recent: Option<RecentOverview>,
    rank: Option<RankPlayerInfo>,
    rank_ships: Vec<RankShipStat>,
    clan: Option<ClanInfo>,
) -> PlayerView {
    let rating = get_overall_rating(&mut ships, pr);
    let total_battles: i64 = ships
        .iter()
        .filter_map(|s| s.pvp.as_ref())
        .map(|pvp| pvp.battles)
        .sum();

    // Keep parity with the stats screen: order by rating desc, unknown last.
    ships.sort_by(|a, b| {
        b.rating
            .partial_cmp(&a.rating)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let ship_lines = ships
        .into_iter()
        .filter(|s| s.pvp.is_some())
        .map(|s| {
            let wiki = warship.get(&s.ship_id);
            ShipStatLine {
                ship_id: s.ship_id,
                name: wiki
                    .map(|w| w.name.clone())
                    .unwrap_or_else(|| s.ship_id.to_string()),
                index: wiki.map(|w| w.index.clone()).unwrap_or_default(),
                tier: wiki.map(|w| w.tier).unwrap_or(0),
                r#type: wiki.map(|w| w.r#type.clone()).unwrap_or_default(),
                nation: wiki.map(|w| w.nation.clone()).unwrap_or_default(),
                icon: wiki.map(|w| w.icon.clone()).unwrap_or_default(),
                premium: wiki.map(|w| w.premium).unwrap_or(false),
                battles: s.battles,
                avg_dmg: s.avg_dmg,
                avg_winrate: s.avg_winrate,
                avg_frags: s.avg_frags,
                rating: s.rating,
                rating_colour: get_colour(Some(s.rating)).to_string(),
                rating_comment: get_comment(s.rating),
                ap: s.ap,
                statistics: PlayerStatistics {
                    battles: s.battles,
                    pvp: s.pvp.clone(),
                    solo: s.solo.clone(),
                    div2: s.div2.clone(),
                    div3: s.div3.clone(),
                    pve: s.pve.clone(),
                    rank_solo: s.rank_solo.clone(),
                    ..Default::default()
                },
                expected_dmg: pr
                    .get(&s.ship_id)
                    .map(|e| e.average_damage_dealt)
                    .unwrap_or_default(),
                expected_winrate: pr
                    .get(&s.ship_id)
                    .map(|e| e.win_rate)
                    .unwrap_or_default(),
                expected_frags: pr
                    .get(&s.ship_id)
                    .map(|e| e.average_frags)
                    .unwrap_or_default(),
                last_battle_time: s.last_battle_time,
            }
        })
        .collect();

    PlayerView {
        account_id: player.account_id,
        nickname: player.nickname,
        server: server.domain().to_string(),
        rating,
        rating_colour: get_colour(Some(rating)).to_string(),
        rating_comment: get_comment(rating),
        ap: get_ap(rating, total_battles),
        hidden_profile: player.hidden_profile.unwrap_or(false),
        ships: ship_lines,
        statistics: player.statistics.unwrap_or_default(),
        achievements,
        created_at: player.created_at,
        last_battle_time: player.last_battle_time,
        leveling_tier: player.leveling_tier,
        logout_at: player.logout_at,
        clan_tag,
        recent,
        rank,
        rank_ships,
        clan,
    }
}

