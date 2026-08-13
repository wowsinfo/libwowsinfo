//! Port of `src/core/downloader/Downloader.ts` data processing, reduced to
//! pure functions (side effects are requested by the Crux app instead).

use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::{
    models::{
        AccountListEntry, Achievement, ApiResponse, EncyclopediaAchievement, EncyclopediaShip,
        PlayerInfo, PlayerView, PrEntry, RawEncyclopediaShip, ShipStatLine, ShipStats,
    },
    rating::{get_ap, get_colour, get_comment, get_overall_rating},
};

/// `Guard` in `src/core/util/SafeGuard.js`: walk a dotted path, returning
/// `dval` when anything along the path is null/missing.
#[must_use]
pub fn guard<'a>(json: &'a Value, path: &str, dval: &'a Value) -> &'a Value {
    if path.is_empty() && !json.is_null() {
        return json;
    }
    if path.starts_with('.') || path.ends_with('.') {
        return dval;
    }
    let mut current = json;
    for part in path.split('.') {
        current = match current.get(part) {
            Some(v) if !v.is_null() => v,
            _ => return dval,
        };
    }
    current
}

/// Cleanup shared by `getPR`/`readLocalPR`: drop empty array entries.
#[must_use]
pub fn clean_pr_data(data: Map<String, Value>) -> Map<String, Value> {
    data.into_iter()
        .filter(|(_, v)| !(v.is_array() && v.as_array().is_some_and(|a| a.is_empty())))
        .collect()
}

/// Parse a PR table (`{data: {<ship_id>: {...}|[]}}`) into typed entries.
#[must_use]
pub fn parse_pr(json: &Value) -> HashMap<u64, PrEntry> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    // The cached form (`SafeStorage` under `SAVED.pr`) is the inner map
    // without the `data` envelope, so fall back to the whole object.
    let data = if data
        .as_object()
        .is_some_and(|map| map.is_empty() && !json.get("data").is_some())
    {
        json
    } else {
        data
    };
    let Some(data) = data.as_object() else {
        return HashMap::new();
    };
    data.iter()
        .filter_map(|(id, v)| {
            let Ok(ship_id) = id.parse::<u64>() else {
                return None;
            };
            // Empty entries (`[]`) are dropped like `getPR`/`readLocalPR` do.
            if v.is_array() {
                return None;
            }
            let Ok(entry) = serde_json::from_value::<PrEntry>(v.clone()) else {
                return None;
            };
            Some((ship_id, entry))
        })
        .collect()
}

/// `readLocalPR` in `Downloader.ts`: parse the bundled rating table.
#[must_use]
pub fn local_pr() -> HashMap<u64, PrEntry> {
    let json: Value =
        serde_json::from_str(include_str!("../assets/personal_rating.json")).unwrap_or_default();
    parse_pr(&json)
}

/// `getWarship` post-processing for a single raw ship entry.
#[must_use]
pub fn process_warship_entry(raw: RawEncyclopediaShip, is_new_launch: bool) -> EncyclopediaShip {
    let mut ship: EncyclopediaShip = raw.into();
    if is_new_launch {
        ship.new = Some(true);
    }
    ship
}

/// Parse `/wows/account/list/` results.
#[must_use]
pub fn parse_search_results(json: &Value) -> Vec<AccountListEntry> {
    let empty = Value::Array(vec![]);
    let data = guard(json, "data", &empty);
    let Some(data) = data.as_array() else {
        return vec![];
    };
    data.iter()
        .filter_map(|v| serde_json::from_value::<AccountListEntry>(v.clone()).ok())
        .collect()
}

/// Parse `/wows/account/info/` for one account.
#[must_use]
pub fn parse_player_info(json: &Value, account_id: u64) -> Option<PlayerInfo> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let entry = data.get(account_id.to_string()).unwrap_or(&empty);
    serde_json::from_value::<PlayerInfo>(entry.clone()).ok()
}

/// Parse the achievements encyclopedia (`data.battle`) into a wiki lookup.
#[must_use]
pub fn parse_achievements_wiki(json: &Value) -> HashMap<String, EncyclopediaAchievement> {
    let empty = Value::Object(Map::new());
    let battle = guard(json, "data.battle", &empty);
    let Some(battle) = battle.as_object() else {
        return HashMap::new();
    };
    battle
        .values()
        .filter_map(|entry| serde_json::from_value::<EncyclopediaAchievement>(entry.clone()).ok())
        .map(|entry| (entry.id.clone(), entry))
        .collect()
}

/// Parse `/wows/account/achievements/` into the player's unlocked
/// achievements (id -> count), enriched with wiki names/icons and sorted by
/// count descending.
#[must_use]
pub fn parse_achievements(
    json: &Value,
    account_id: u64,
    wiki: &HashMap<String, EncyclopediaAchievement>,
) -> Vec<Achievement> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let entry = data.get(account_id.to_string()).unwrap_or(&empty);
    let mut achievements: Vec<Achievement> = entry
        .get("battle")
        .and_then(Value::as_object)
        .map(|battle| {
            battle
                .iter()
                .filter_map(|(id, count)| {
                    count.as_u64().map(|count| Achievement {
                        id: id.clone(),
                        count,
                        name: wiki.get(id).map(|e| e.name.clone()).unwrap_or_default(),
                        icon: wiki.get(id).map(|e| e.icon.clone()).unwrap_or_default(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    achievements.sort_by(|a, b| b.count.cmp(&a.count));
    achievements
}

/// Parse the player's clan tag from `/wows/clans/accountinfo/` (empty when
/// the player is not in a clan).
#[must_use]
pub fn parse_clan_tag(json: &Value, account_id: u64) -> String {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    data.get(account_id.to_string())
        .and_then(|entry| entry.get("clan"))
        .and_then(|clan| clan.get("tag"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

/// Parse `/wows/ships/stats/` into the ship list for one account.
///
/// The API returns `data.<account_id>` either as an object keyed by ship id
/// (older responses) or as an array of per-ship entries (current format).
#[must_use]
pub fn parse_ship_stats(json: &Value, account_id: u64) -> Vec<ShipStats> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let Some(account) = data.get(account_id.to_string()) else {
        return vec![];
    };

    let mut ships = Vec::new();
    if let Some(entries) = account.as_array() {
        for entry in entries {
            if let Ok(mut stats) = serde_json::from_value::<ShipStats>(entry.clone()) {
                stats.ship_id = entry
                    .get("ship_id")
                    .and_then(Value::as_u64)
                    .unwrap_or_default();
                if stats.ship_id != 0 {
                    ships.push(stats);
                }
            }
        }
    } else if let Some(map) = account.as_object() {
        for (key, value) in map {
            if key == "pvp" {
                continue;
            }
            if let Ok(mut stats) = serde_json::from_value::<ShipStats>(value.clone()) {
                stats.ship_id = value
                    .get("ship_id")
                    .and_then(Value::as_u64)
                    .or_else(|| key.parse().ok())
                    .unwrap_or_default();
                if stats.ship_id != 0 {
                    ships.push(stats);
                }
            }
        }
    }
    ships
}

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
                ap: s.ap,
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
        clan_tag,
    }
}

/// `checkVersionUpdate` in `Downloader.ts`: strings simply differ.
#[must_use]
pub fn check_version_update(previous: &str, current: &str) -> bool {
    previous != current
}

/// `ApiResponse` helper: true when `status == "ok"` (SafeFetch's contract).
#[must_use]
pub fn is_ok<T>(response: &ApiResponse<T>) -> bool {
    response.status == "ok"
}

#[cfg(test)]
mod tests;
