//! Clan parsing (`/wows/clans/accountinfo/`, `/wows/clans/list/`, `/wows/clans/info/`).

use serde_json::{Map, Value};

use super::guard;
use crate::models::{ClanInfo, ClanMember, ClanSearchResult};

/// Clan id from `/wows/clans/accountinfo/` (`clan.id`).
#[must_use]
pub fn parse_clan_id(json: &Value, account_id: u64) -> Option<u64> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    data.get(account_id.to_string())
        .and_then(|entry| entry.get("clan"))
        .and_then(|clan| clan.get("id"))
        .and_then(Value::as_u64)
}

/// Parse `/wows/clans/info/` into full clan info with members.
#[must_use]
pub fn parse_clan_info(json: &Value, clan_id: u64) -> Option<ClanInfo> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let clan = data.get(clan_id.to_string())?;
    let str_field = |key: &str| {
        clan
            .get(key)
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let int_field = |key: &str| clan.get(key).and_then(Value::as_i64).unwrap_or(0);
    let mut members = Vec::new();
    if let Some(map) = clan.get("members").and_then(Value::as_object) {
        for (id, member) in map {
            members.push(ClanMember {
                account_id: id.parse().unwrap_or(0),
                account_name: member
                    .get("account_name")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                role: member
                    .get("role")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                joined_at: member.get("joined_at").and_then(Value::as_i64).unwrap_or(0),
            });
        }
    }
    Some(ClanInfo {
        clan_id: clan.get("clan_id").and_then(Value::as_u64).unwrap_or(clan_id),
        tag: str_field("tag"),
        name: str_field("name"),
        description: str_field("description"),
        members_count: int_field("members_count"),
        created_at: int_field("created_at"),
        updated_at: int_field("updated_at"),
        leader_id: int_field("leader_id"),
        leader_name: str_field("leader_name"),
        creator_id: int_field("creator_id"),
        creator_name: str_field("creator_name"),
        old_name: str_field("old_name"),
        old_tag: str_field("old_tag"),
        renamed_at: int_field("renamed_at"),
        is_clan_disbanded: clan
            .get("is_clan_disbanded")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        members,
    })
}

/// Parse `/wows/clans/list/` search results.
#[must_use]
pub fn parse_clan_search(json: &Value) -> Vec<ClanSearchResult> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    data.as_array()
        .map(|list| {
            list.iter()
                .filter_map(|entry| {
                    serde_json::from_value::<ClanSearchResult>(entry.clone()).ok()
                })
                .collect()
        })
        .unwrap_or_default()
}
