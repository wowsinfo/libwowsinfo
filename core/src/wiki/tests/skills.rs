//! Commander-skill real-data tests.

use serde_json::json;

use super::super::*;

#[test]
fn skills_expose_structured_tiers() {
    let Some(path) = std::env::var_os("WOWSINFO_JSON") else {
        return;
    };
    let raw = std::fs::read_to_string(path).expect("read wowsinfo.json");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    let data = parse_game_data(&json);
    let skills = all_skill_views(&data, &LangMap::default());
    assert!(skills.len() > 50, "skills: {}", skills.len());
    assert!(
        skills.iter().all(|skill| !skill.tiers.is_empty()),
        "every skill has per-class tiers"
    );
    assert!(
        skills.iter().any(|skill| skill.tiers.iter().any(|tier| tier.tier >= 4)),
        "tier-4 skills exist"
    );
    assert!(
        skills.iter().any(|skill| skill.tiers.iter().any(|tier| tier.ship_class == "Cruiser")),
        "cruiser tiers exist"
    );
}
