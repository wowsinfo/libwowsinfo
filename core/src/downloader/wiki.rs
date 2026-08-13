//! Wiki parsing (`/wows/encyclopedia/collections|collectioncards|consumables|crewskills`).

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::guard;
use crate::models::{
    CollectionCard, CommanderSkill, Consumable, ConsumableProfile, Perk, WikiCollection,
};

/// Merge one paginated wiki page (`data: {<id>: entry}`) into a map keyed by
/// the entry's own id field.
fn parse_wiki_map<T: serde::de::DeserializeOwned>(
    json: &Value,
    id_field: &str,
) -> HashMap<u64, T> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let mut out = HashMap::new();
    if let Some(map) = data.as_object() {
        for (key, value) in map {
            let Some(id) = value
                .get(id_field)
                .and_then(Value::as_u64)
                .or_else(|| key.parse().ok())
            else {
                continue;
            };
            if let Ok(item) = serde_json::from_value::<T>(value.clone()) {
                out.insert(id, item);
            }
        }
    }
    out
}

/// Parse one page of `/wows/encyclopedia/collections/`.
#[must_use]
pub fn parse_collections(json: &Value) -> HashMap<u64, WikiCollection> {
    parse_wiki_map(json, "collection_id")
}

/// Parse one page of `/wows/encyclopedia/collectioncards/`.
#[must_use]
pub fn parse_collection_cards(json: &Value) -> HashMap<u64, CollectionCard> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let mut out = HashMap::new();
    if let Some(map) = data.as_object() {
        for (key, value) in map {
            let Some(id) = value
                .get("card_id")
                .and_then(Value::as_u64)
                .or_else(|| key.parse().ok())
            else {
                continue;
            };
            let str_field = |name: &str| {
                value
                    .get(name)
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string()
            };
            let image = value
                .get("images")
                .and_then(|images| images.get("small"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            out.insert(
                id,
                CollectionCard {
                    card_id: id,
                    collection_id: value
                        .get("collection_id")
                        .and_then(Value::as_u64)
                        .unwrap_or(0),
                    name: str_field("name"),
                    description: str_field("description"),
                    image,
                },
            );
        }
    }
    out
}

/// Parse one page of `/wows/encyclopedia/consumables/`. The `profile` field is
/// a map of profile-id -> description, flattened to a list.
#[must_use]
pub fn parse_consumables(json: &Value) -> HashMap<u64, Consumable> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let mut out = HashMap::new();
    if let Some(map) = data.as_object() {
        for (key, value) in map {
            let Some(id) = value
                .get("consumable_id")
                .and_then(Value::as_u64)
                .or_else(|| key.parse().ok())
            else {
                continue;
            };
            let profile = value
                .get("profile")
                .and_then(Value::as_object)
                .map(|profiles| {
                    profiles
                        .values()
                        .filter_map(|p| p.get("description"))
                        .filter_map(Value::as_str)
                        .map(|description| ConsumableProfile {
                            description: description.to_string(),
                        })
                        .collect()
                })
                .unwrap_or_default();
            let str_field = |key: &str| {
                value
                    .get(key)
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string()
            };
            out.insert(
                id,
                Consumable {
                    consumable_id: id,
                    name: str_field("name"),
                    description: str_field("description"),
                    image: str_field("image"),
                    r#type: str_field("type"),
                    price_credit: value
                        .get("price_credit")
                        .and_then(Value::as_i64)
                        .unwrap_or(0),
                    price_gold: value
                        .get("price_gold")
                        .and_then(Value::as_i64)
                        .unwrap_or(0),
                    profile,
                },
            );
        }
    }
    out
}

/// Parse one page of `/wows/encyclopedia/crewskills/`.
#[must_use]
pub fn parse_commander_skills(json: &Value) -> HashMap<u64, CommanderSkill> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let mut out = HashMap::new();
    if let Some(map) = data.as_object() {
        for (key, value) in map {
            let Some(id) = key.parse::<u64>().ok() else {
                continue;
            };
            let mut skill = serde_json::from_value::<CommanderSkill>(value.clone()).unwrap_or_default();
            skill.skill_id = id;
            let perks = value
                .get("perks")
                .and_then(Value::as_array)
                .map(|list| {
                    list.iter()
                        .filter_map(|perk| {
                            Some(Perk {
                                perk_id: perk.get("perk_id").and_then(Value::as_u64).unwrap_or(0),
                                description: perk
                                    .get("description")
                                    .and_then(Value::as_str)
                                    .unwrap_or("")
                                    .to_string(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            skill.perks = perks;
            out.insert(id, skill);
        }
    }
    out
}
