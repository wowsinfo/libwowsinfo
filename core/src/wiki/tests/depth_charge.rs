//! Depth-charge real-data tests.


use super::super::*;

#[test]
fn depth_charges_parse_full_panel() {
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
        4_288_591_856,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    )
    .expect("depth charge ship wiki");

    let depth = wiki.depth_charges.expect("depth charges");
    assert!((depth.reload_s - 40.0).abs() < 1e-6);
    assert_eq!(depth.groups, 2);
    assert_eq!(depth.bombs, 8);
    assert_eq!(depth.launcher_count, 8);
    assert_eq!(depth.bombs_per_charge, 8, "8 throwers x 1 bomb x 1 shot");

    let packs = depth.packs.expect("pack settings");
    assert_eq!(packs.shots, 1);
    assert_eq!(packs.max_packs, 2);
    assert!((packs.shot_delay_s - 1.0).abs() < 1e-6);
    assert!((packs.center_zone_width_part - 0.35).abs() < 1e-6);

    let launcher = &depth.launchers[0];
    assert_eq!(launcher.bombs, 1);
    assert_eq!(launcher.horizontal_sector, "-150° .. 150°");
    assert_eq!(launcher.vertical_sector, "0° .. 90°");

    assert!((depth.damage - 3800.0).abs() < 1e-6);
    assert!((depth.fire_chance - 15.0).abs() < 1e-6, "0.15 -> 15%");
    assert!((depth.flood_chance - 23.0).abs() < 1e-6);
    assert_eq!(depth.sink_speed, Some(300.0));
    assert_eq!(depth.detonation_depth_m, Some(80.0));
    assert_eq!(depth.splash_radius_m, Some(26.67));
    assert_eq!(depth.points_of_damage.len(), 4);
    assert_eq!(depth.can_hit_classes, vec!["Submarine"]);
    assert_eq!(depth.buoyancy.len(), 5);
    assert!((depth.fall_distance.unwrap_or(0.0) - 20.0).abs() < 1e-6);
}
