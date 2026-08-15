//! Main battery (Kawachi) real-data tests.


use super::super::*;

#[test]
fn kawachi_main_battery_matches_shipbuilder_values() {
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
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    )
    .expect("kawachi wiki");
    let battery = wiki.main_battery.expect("kawachi main battery");

    // Published stats (wowsdb): 305 mm, 6x2, 30 s reload, 36 s rotation,
    // 9.88 km range, 155 m max dispersion.
    assert!((battery.caliber_mm - 305.0).abs() < 1e-6);
    assert_eq!(battery.barrels, 12);
    assert!((battery.rof - 2.0).abs() < 1e-6);
    assert!((battery.reload_s - 30.0).abs() < 1e-6);
    assert!((battery.traverse_deg_s - 5.0).abs() < 1e-6);
    assert!((battery.turn_time_s - 36.0).abs() < 1e-6);
    assert!((battery.range_m - 9880.0).abs() < 1e-6);

    let disp = battery.dispersion.expect("kawachi dispersion model");
    assert!(disp.normal_distribution);
    assert!((disp.taper_dist_m - 3000.0).abs() < 1e-6);
    assert!((disp.delim_dist_m - 4940.0).abs() < 1e-6);
    assert!((disp.at_max.horizontal_m - 155.136).abs() < 1e-3);
    assert!((disp.at_max.vertical_m - 124.109).abs() < 1e-3);
    assert_eq!(disp.samples.len(), 1, "5 km sample below the 9.88 km max");
    assert!((disp.samples[0].range_m - 5000.0).abs() < 1e-6);
    assert!((disp.samples[0].horizontal_m - 120.0).abs() < 1e-6);

    assert_eq!(battery.firing_arcs.len(), 6, "six Kawachi turrets");
    assert!(
        battery
            .firing_arcs
            .iter()
            .all(|arc| arc.vert_max == 15.0 && arc.vert_min == -2.0)
    );

    // DPM / salvo values: 12 barrels x 2 rpm x shell alpha.
    assert_eq!(battery.per_shell_dpm.len(), 2, "HE + AP");
    for entry in &battery.per_shell_dpm {
        let shell = battery
            .shells
            .iter()
            .find(|s| s.key == entry.shell_key)
            .expect("shell key");
        assert_eq!(entry.dpm, (shell.damage as f64 * 12.0 * 2.0).round() as i64);
        assert_eq!(entry.salvo_damage, shell.damage * 12);
        assert!((entry.salvo_weight_kg - shell.weight * 12.0).abs() < 1e-6);
        let expected_fpm = shell
            .burn_chance
            .map(|chance| ((chance * 12.0 * 2.0) * 100.0).round() / 100.0)
            .unwrap_or(0.0);
        assert!(
            (entry.potential_fpm - expected_fpm).abs() < 1e-9,
            "{} potential_fpm {} != {}",
            entry.shell_key,
            entry.potential_fpm,
            expected_fpm
        );
    }
    assert!(
        battery.per_shell_dpm.iter().any(|entry| entry.potential_fpm > 0.0),
        "HE shell has fires per minute"
    );
}
