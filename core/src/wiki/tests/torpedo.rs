//! Torpedo (Constellation) real-data tests.


use super::super::*;

#[test]
fn constellation_torpedoes_match_shipbuilder_values() {
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
        3_730_782_192,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    )
    .expect("constellation wiki");
    let torpedo = wiki.torpedoes.expect("constellation torpedoes");

    // Launcher bank: 2x3 tubes, 98 s reload, 7.2 s turn time.
    assert_eq!(torpedo.configuration, "2 x 3");
    assert_eq!(torpedo.torpedo_count, 6);
    assert!((torpedo.reload_s - 98.0).abs() < 1e-6);
    assert!((torpedo.turn_time_s - 7.2).abs() < 1e-6);
    assert!((torpedo.rotation_deg_s - 25.0).abs() < 1e-6);

    let torp = torpedo
        .torpedoes
        .iter()
        .find(|t| t.key == "PAPT047_MK_11_CONSTELLATION")
        .expect("constellation torpedo");
    assert_eq!(torp.key, "PAPT047_MK_11_CONSTELLATION");
    // Display damage = alpha/3 + damage; range = raw / (100/3) km;
    // reaction = detectability / (speed * 2.6854) * 1000.
    assert_eq!(torp.damage, 16_633);
    assert_eq!(torp.alpha_damage, 46_600);
    assert_eq!(torp.salvo_damage, 99_798);
    assert!((torp.range_km - 9.15).abs() < 1e-6);
    assert!((torp.speed_kt - 55.0).abs() < 1e-6);
    assert!((torp.detectability_km - 1.1).abs() < 1e-6);
    assert!(
        (torp.reaction_time_s - (1.1 / 55.0 / 2.6854 * 1000.0)).abs() < 1e-9,
        "reaction: {}",
        torp.reaction_time_s
    );
    assert_eq!(torp.arming_distance_m, Some(55.0));
    assert_eq!(torp.depth_m, Some(0.14));
    assert_eq!(torp.flood_chance, Some(279.0));
    assert_eq!(torp.splash_armor_coeff, Some(0.4));
    assert_eq!(torp.splash_cube_size, Some(1.2));
    assert_eq!(torp.ping_damage_coeff, Some(2.0));
    assert!(torp.acoustic_detection.is_none());
    assert_eq!(torp.can_hit_classes.len(), 6);
}
