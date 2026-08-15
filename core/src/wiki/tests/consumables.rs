//! Consumable alter-variant real-data tests.

use serde_json::json;

use super::super::*;

#[test]
fn consumables_expose_alter_variants() {
    let Some(path) = std::env::var_os("WOWSINFO_JSON") else {
        return;
    };
    let raw = std::fs::read_to_string(path).expect("read wowsinfo.json");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    let data = parse_game_data(&json);
    let consumables = all_consumable_views(&data, &LangMap::default());
    assert!(consumables.len() >= 201, "consumables: {}", consumables.len());
    assert!(
        consumables.iter().any(|consumable| !consumable.alters.is_empty()),
        "at least one consumable has alter variants"
    );
    let smoke = consumables
        .iter()
        .find(|consumable| consumable.key == "PCY006_SmokeGenerator")
        .expect("smoke generator");
    assert!(smoke.alters.len() >= 2, "smoke alters: {:?}", smoke.alters);
    assert!(
        smoke
            .alters
            .iter()
            .any(|alter| alter.key == "PCY006_SmokeGeneratorCrawler"),
        "crawler smoke alter present"
    );
}
