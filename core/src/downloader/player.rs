//! Player / achievement processing.

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::guard::guard;
use crate::models::{Achievement, EncyclopediaAchievement, PlayerInfo};


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

/// Players online for the game, matching the app's
/// `data.wows.0.players_online` guard (first server entry, -1 when missing).
#[must_use]
pub fn parse_online_count(json: &Value) -> i64 {
    let empty = Value::Object(Map::new());
    let wows = guard(json, "data.wows", &empty);
    wows.as_array()
        .and_then(|servers| servers.first())
        .and_then(|server| server.get("players_online"))
        .and_then(Value::as_i64)
        .unwrap_or(-1)
}
