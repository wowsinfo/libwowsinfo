//! Special ability (rage) real-data tests.

use serde_json::json;

use super::super::*;

#[test]
fn rage_mode_parses_real_special_ability() {
    let Some(path) = std::env::var_os("WOWSINFO_JSON") else {
        return;
    };
    let raw = std::fs::read_to_string(path).expect("read wowsinfo.json");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    let data = parse_game_data(&json);
    let lang = LangMap::default();
    let wiki = build_local_ship_wiki(
        &data,
        &lang,
        4_178_524_144,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    )
    .expect("rage ship wiki");
    let special = wiki.special_ability.expect("special ability");

    assert_eq!(special.mode, "survivability");
    assert_eq!(special.name, "Survivability");
    assert!((special.duration_s - 45.0).abs() < 1e-6);
    assert_eq!(special.progress_name, "main_gun_hit");
    assert!((special.progress_per_action - 6.0).abs() < 1e-6);
    assert_eq!(special.required_count, 1);
    assert_eq!(special.sub_ribbons, vec![14, 15, 16, 17, 28]);
    assert!((special.inactivity_delay_s - 50.0).abs() < 1e-6);
    assert!((special.progress_loss_interval_s - 1.0).abs() < 1e-6);
    assert!((special.progress_loss_per_interval - 5.0).abs() < 1e-6);
    assert!(!special.auto_usage);
    assert!(
        special
            .modifiers
            .iter()
            .any(|line| line.contains("AA damage") && line.contains("+25%")),
        "modifiers: {:?}",
        special.modifiers
    );
}

#[test]
fn bundled_asset_contains_rage_data() {
    let Some(path) = std::env::var_os("WOWSINFO_ZST") else {
        return;
    };
    let raw = std::fs::read(path).expect("read bundled zst");
    let text = decompress_zstd(&raw).expect("decompress bundled zst");
    assert!(
        text.contains("\"specialAbility\"") && text.contains("\"rage\""),
        "bundled bundle has the special ability block"
    );
    assert!(text.contains("PASB111"), "bundled bundle has Maine");
}
