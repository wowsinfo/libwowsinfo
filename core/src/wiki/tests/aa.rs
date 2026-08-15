//! Anti-air real-data tests.


use super::super::*;

#[test]
fn kawachi_air_defense_parses_rich_auras() {
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
        4_293_867_216,
        ModuleSelection {
            hull: 1,
            ..ModuleSelection::default()
        },
        &LocalBuildConfig::default(),
    )
    .expect("kawachi wiki");
    let air_defense = wiki.air_defense.expect("kawachi AA");

    // B_AirDefense: rich medium aura (explosions/shot travel) + legacy mounts.
    assert_eq!(air_defense.medium.len(), 1);
    let aura = &air_defense.medium[0];
    assert!((aura.dps - 10.5).abs() < 1e-6);
    assert_eq!(aura.explosion_count, 15);
    assert!((aura.shot_travel_time - 1.5).abs() < 1e-6);
    assert!((aura.max_range - 3.0).abs() < 1e-6);
    assert_eq!(aura.guns.len(), 1, "legacy gun mounts merged into the aura");
}

#[test]
fn atba_only_air_defense_uses_bubbles_fallback() {
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
        3_867_587_856,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    )
    .expect("atba-aa wiki");
    let air_defense = wiki.air_defense.expect("AA from the ATBA component");
    assert_eq!(air_defense.far.len(), 1);
    assert!(air_defense.far[0].dps > 0.0);
    let bubbles = air_defense.bubbles.expect("flak cloud block");
    assert!(bubbles.inner > 0 || bubbles.outer > 0);
    assert!(bubbles.damage > 0.0);
}
