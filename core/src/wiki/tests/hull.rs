//! Hull maneuverability / concealment real-data tests.


use super::super::*;

#[test]
fn maneuverability_parses_raw_engine_coefficients() {
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
        4_292_851_696,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    )
    .expect("maneuverability wiki");
    let hull = wiki.hull.expect("hull");
    let maneuverability = hull.maneuverability.expect("maneuverability");
    assert!(maneuverability.max_reverse_speed > 0.0);
    let raw = maneuverability.raw.expect("raw coefficients");
    assert!(raw.engine_power > 0.0);
    assert!(raw.side_drag_coef > 0.0);
    assert!(raw.backward_movement_drag_coef > 0.0);
    assert!(raw.max_rudder_angle > 0.0);
}

#[test]
fn concealment_parses_coefficient_tables() {
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
        4_292_851_696,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    )
    .expect("concealment wiki");
    let hull = wiki.hull.expect("hull");
    let concealment = hull.concealment.expect("concealment");
    assert!(concealment.sea_fire > 0.0);
    assert!(concealment.air_fire > 0.0);
    assert_eq!(concealment.by_submarine_depth.len(), 5);
    assert!(
        concealment
            .by_submarine_depth
            .iter()
            .any(|(state, _)| state == "PERISCOPE"),
        "periscope row present"
    );
    assert!(!concealment.underwater_depth_coeff.is_empty());
    assert!(!concealment.underwater_depth_coeff_plane.is_empty());
}
