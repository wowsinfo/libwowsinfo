//! Port of `src/core/downloader/Downloader.ts` data processing, reduced to
//! pure functions (side effects are requested by the Crux app instead).

use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::{
    models::{
        AccountListEntry, ApiResponse, EncyclopediaShip, PlayerInfo, PlayerView, PrEntry,
        RawEncyclopediaShip, ShipStatLine, ShipStats,
    },
    rating::{get_colour, get_comment, get_overall_rating},
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

/// Parse `/wows/ships/stats/` into the ship list for one account.
#[must_use]
pub fn parse_ship_stats(json: &Value, account_id: u64) -> Vec<ShipStats> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let Some(account) = data.get(account_id.to_string()) else {
        return vec![];
    };
    let Some(account) = account.as_object() else {
        return vec![];
    };
    account
        .iter()
        .filter(|(k, _)| k.as_str() != "pvp")
        .filter_map(|(_, v)| {
            let mut stats: ShipStats = serde_json::from_value(v.clone()).ok()?;
            stats.ship_id = v.get("ship_id").and_then(Value::as_u64).unwrap_or_default();
            Some(stats)
        })
        .collect()
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
) -> PlayerView {
    let rating = get_overall_rating(&mut ships, pr);

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
        ap: 0.0,
        hidden_profile: player.hidden_profile.unwrap_or(false),
        ships: ship_lines,
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
mod tests {
    use super::*;

    #[test]
    fn guard_walks_paths_and_falls_back() {
        let json = serde_json::json!({"data": {"battle": {"wins": 5}}});
        let default = Value::Bool(false);
        assert_eq!(guard(&json, "data.battle.wins", &default), &Value::from(5));
        assert_eq!(
            guard(&json, "data.battle.losses", &default),
            &Value::Bool(false)
        );
        assert_eq!(guard(&json, "data.nope", &default), &Value::Bool(false));
        assert_eq!(guard(&json, "", &default), &json);
        assert_eq!(guard(&json, ".data", &default), &Value::Bool(false));
    }

    #[test]
    fn pr_cleanup_drops_empty_arrays() {
        let json = serde_json::json!({
            "data": {
                "1": {"average_damage_dealt": 1.0, "average_frags": 0.5, "win_rate": 50.0},
                "2": []
            }
        });
        let parsed = parse_pr(&json);
        assert_eq!(parsed.len(), 1);
        assert!(parsed.contains_key(&1));
        assert!(!parsed.contains_key(&2));
    }

    #[test]
    fn local_pr_has_data() {
        let pr = local_pr();
        assert!(
            pr.len() > 10,
            "bundled personal_rating.json should be usable"
        );
        let entry = pr.get(&3542005744).expect("known ship");
        assert!(entry.average_damage_dealt > 0.0);
    }

    #[test]
    fn search_results_parse() {
        let json = serde_json::json!({
            "status": "ok",
            "data": [{"account_id": 123, "nickname": "HenryQuan"}, {"account_id": 456, "nickname": "x"}]
        });
        let results = parse_search_results(&json);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].nickname, "HenryQuan");
    }

    #[test]
    fn player_info_parse_handles_keyed_data() {
        let json = serde_json::json!({
            "status": "ok",
            "data": {"42": {"account_id": 42, "nickname": "Bob", "hidden_profile": true}}
        });
        let info = parse_player_info(&json, 42).expect("player");
        assert_eq!(info.nickname, "Bob");
        assert_eq!(info.hidden_profile, Some(true));
    }

    #[test]
    fn ship_stats_parse_skips_pvp_rollup() {
        let json = serde_json::json!({
            "status": "ok",
            "data": {"42": {
                "1": {"ship_id": 1, "battles": 10, "wins": 5, "damage_dealt": 100, "frags": 2,
                      "pvp": {"battles": 10, "wins": 5, "damage_dealt": 100, "frags": 2}},
                "pvp": {"battles": 10}
            }}
        });
        let ships = parse_ship_stats(&json, 42);
        assert_eq!(ships.len(), 1);
        assert_eq!(ships[0].ship_id, 1);
    }

    #[test]
    fn version_check_is_simple_inequality() {
        assert!(check_version_update("12.7.0.0", "12.8.0.0"));
        assert!(!check_version_update("12.7.0.0", "12.7.0.0"));
    }

    #[test]
    fn assemble_player_builds_view() {
        let pr = local_pr();
        let ships = vec![ShipStats {
            ship_id: 3542005744,
            battles: 100,
            wins: 50,
            damage_dealt: 5_000_000,
            frags: 80,
            pvp: Some(crate::models::PvpStats {
                battles: 100,
                wins: 50,
                damage_dealt: 5_000_000,
                frags: 80,
            }),
            ..Default::default()
        }];

        let view = assemble_player(
            PlayerInfo {
                account_id: 1,
                nickname: "HenryQuan".to_string(),
                ..Default::default()
            },
            ships,
            &pr,
            &HashMap::new(),
            crate::data::Server::Asia,
        );
        assert_eq!(view.nickname, "HenryQuan");
        assert_eq!(view.server, "asia");
        assert_eq!(view.ships.len(), 1);
        assert!(view.ships[0].rating > 0.0);
        assert!(view.rating > 0.0);
    }
}
