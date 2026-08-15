//! Airstrike real-data tests.


use super::super::*;

#[test]
fn airstrike_parses_full_panel() {
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
        4_273_977_328,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    )
    .expect("air support ship wiki");

    let strike = wiki.air_support.expect("airstrike");
    assert!(strike.auto_usage);
    assert_eq!(strike.charges, 1);
    assert!((strike.reload_s - 25.0).abs() < 1e-6);
    assert!((strike.range_km - 8.0).abs() < 1e-6);
    assert!((strike.min_dist_m - 500.0).abs() < 1e-6);
    assert!((strike.max_dist_m - 8000.0).abs() < 1e-6);
    assert!((strike.max_plane_flight_dist_m - 3800.0).abs() < 1e-6);
    assert!((strike.climb_angle_deg - 30.0).abs() < 1e-6);
    assert!((strike.fly_away_time_s - 5.0).abs() < 1e-6);
    assert!((strike.time_between_shots_s - 2.0).abs() < 1e-6);
    assert!((strike.time_from_heaven_s - 2.0).abs() < 1e-6);
}
