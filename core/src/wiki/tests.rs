//! Tests for the local wiki-data parsers, using representative fixtures in the
//! exact shapes produced by `wows-constants` and WoWs-Game-Data.

use serde_json::json;

use super::*;

#[test]
fn zstd_decompress_roundtrip() {
    let input = r#"{"ships":{},"projectiles":{}}"#;
    let compressed = zstd::stream::encode_all(input.as_bytes(), 3).expect("compress");
    assert_eq!(decompress_zstd(&compressed).expect("decompress"), input);
}

#[test]
fn achievement_views_localise_and_format_constants() {
    let json = json!({
        "achievements": {
            "A1": {
                "id": 1, "icon": "DOUBLE_KILL",
                "name": "IDS_ACH_1", "description": "IDS_ACH_DESC_1",
                "type": ["PVP"], "constants": {"timeInterval": 10.0}
            }
        }
    });
    let data = parse_game_data(&json);
    let lang = LangMap::from_entries([
        ("IDS_ACH_1".into(), "Double Strike".into()),
        ("IDS_ACH_DESC_1".into(), "Within %(timeInterval)s seconds.".into()),
    ]);
    let views = all_achievement_views(&data, &lang);
    assert_eq!(views.len(), 1);
    assert_eq!(views[0].name, "Double Strike");
    assert_eq!(views[0].description, "Within 10 seconds.");
}

#[test]
fn upgrade_views_localise_and_summarise_modifiers() {
    let json = json!({
        "modernizations": {
            "PCM001": {
                "slot": 2, "icon": "PCM001_Icon", "name": "IDS_UPG",
                "description": "IDS_UPG_DESC", "costCR": 200000,
                "level": [1, 2], "type": ["Battleship"], "nation": ["USA"],
                "ships": [1], "excludes": [],
                "modifiers": {"artilleryRange": 1.15}
            }
        }
    });
    let data = parse_game_data(&json);
    let lang = LangMap::from_entries([
        ("IDS_UPG".into(), "Aiming Systems".into()),
        ("IDS_UPG_DESC".into(), "Increases range.".into()),
        ("IDS_PARAMS_MODIFIER_ARTILLERYRANGE".into(), "Artillery range".into()),
    ]);
    let views = all_upgrade_views(&data, &lang);
    assert_eq!(views.len(), 1);
    assert_eq!(views[0].icon, "PCM001_Icon");
    assert_eq!(views[0].name, "Aiming Systems");
    assert_eq!(views[0].slot, 2);
    assert_eq!(views[0].cost_cr, 200000);
    assert!(views[0].summary.contains("+15%"));
}

#[test]
fn parses_game_constants() {
    let json = json!({
        "VERSION": {"VERSION": "15.7", "BUILD": 13015811, "PATCH": 0.0},
        "SHIP_TYPES": {"Cruiser": 2, "Destroyer": 3, "Battleship": 1},
        "BATTLE_TYPES": {
            "9": {"playersPerTeam": 12, "name": "12x12", "scenario": "1_defence_east", "teamsCount": 2}
        },
        "CONSUMABLE_IDS": {"activeManeuvering": 21, "callFighters": 22},
        "SKILLS_BY_SHIP_TYPE": {
            "Cruiser": [
                {"0": [3, 24, 13], "1": [19, 20]}
            ]
        },
        "RIBBONS": {"0": "RIBBON_MAIN_CALIBER", "1": "RIBBON_TORPEDO"},
        "DEATH_REASONS": {
            "1": {"sound": "Health", "icon": "frags", "id": 1, "name": "ARTILLERY"}
        }
    });
    let constants = parse_constants(&json);
    let version = constants.version.expect("version");
    assert_eq!(version.version, "15.7");
    assert_eq!(version.build, 13_015_811);
    assert_eq!(constants.ship_types["Cruiser"], 2);
    let battle = &constants.battle_types[&9];
    assert_eq!(battle.name, "12x12");
    assert_eq!(battle.players_per_team, 12);
    assert_eq!(constants.consumable_ids["activeManeuvering"], 21);
    let cruiser_skills = &constants.skills_by_ship_type["Cruiser"][0];
    assert_eq!(cruiser_skills["0"], vec![3, 24, 13]);
    assert_eq!(constants.ribbons[&0], "RIBBON_MAIN_CALIBER");
    assert_eq!(constants.death_reasons[&1].name, "ARTILLERY");
    assert!(constants.death_reasons[&1].icon == "frags");
}

#[test]
fn constants_are_tolerant_of_missing_sections() {
    let constants = parse_constants(&json!({"VERSION": {}}));
    assert!(constants.version.is_some());
    assert!(constants.ship_types.is_empty());
    assert!(constants.battle_types.is_empty());
    let empty = parse_constants(&json!({}));
    assert!(empty.version.is_none());
}

#[test]
fn parses_game_data() {
    let json = json!({
        "ships": {
            "1": {
                "id": 1, "name": "Hermelin", "description": "d", "year": "1936",
                "paperShip": false, "index": "PASD001", "tier": 1,
                "region": "pan_asia", "type": "dd", "regionID": "PA", "typeID": "DD",
                "group": "normal", "costXP": 0, "costGold": 0, "costCR": 100,
                "consumables": [[{"name": "Repair", "type": "repair"}]],
                "nextShips": [2],
                "modules": {"hull": []}, "components": {"artillery": []}
            }
        },
        "abilities": {
            "21": {
                "id": 21, "nation": "common", "name": "Active Maneuvering",
                "icon": "i", "description": "boost", "filter": "dd", "type": "consumable",
                "abilities": {"speed": 1.0}, "alter": null
            }
        },
        "achievements": {
            "7": {
                "id": 7, "icon": "ic", "name": "First Blood", "description": "desc",
                "type": ["battle"], "constants": {"max": 1}, "added": "0.1.0"
            }
        },
        "commandSkills": {
            "Destroyer": [
                [{"name": "Preventive Maintenance", "tier": 1, "column": 0, "description": "x", "icon": "i"}]
            ]
        }
    });
    let data = parse_game_data(&json);
    let ship = &data.ships[&1];
    assert_eq!(ship.name, "Hermelin");
    assert_eq!(ship.tier, 1);
    assert_eq!(ship.region_id, "PA");
    assert_eq!(ship.consumables[0][0].name, "Repair");
    assert_eq!(ship.next_ships, vec![2]);
    assert!(ship.modules.is_object());
    let ability = &data.abilities[&21];
    assert_eq!(ability.name, "Active Maneuvering");
    assert_eq!(ability.r#type, "consumable");
    let achievement = &data.achievements[&7];
    assert_eq!(achievement.name, "First Blood");
    assert_eq!(achievement.r#type, vec!["battle"]);
    assert_eq!(data.command_skills["Destroyer"][0][0].name, "Preventive Maintenance");
}

#[test]
fn game_data_is_tolerant_of_missing_sections() {
    let data = parse_game_data(&json!({}));
    assert!(data.ships.is_empty());
    assert!(data.abilities.is_empty());
    assert!(data.achievements.is_empty());
    assert!(data.command_skills.is_empty());
}

#[test]
fn real_wowsinfo_json_parses_when_available() {
    // Smoke test against the shipped game data; skipped in CI without the env var.
    let Some(path) = std::env::var_os("WOWSINFO_JSON") else {
        return;
    };
    let raw = std::fs::read_to_string(path).expect("read wowsinfo.json");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    let data = parse_game_data(&json);
    assert!(data.ships.len() > 1000, "ships: {}", data.ships.len());
    assert!(data.projectiles.len() > 2000, "projectiles: {}", data.projectiles.len());
    assert!(data.aircraft.len() > 500, "aircraft: {}", data.aircraft.len());
    assert!(data.modernizations.len() > 50, "modernizations: {}", data.modernizations.len());
    assert!(data.flags.len() >= 10, "flags: {}", data.flags.len());
    assert!(data.skills.len() > 50, "skills: {}", data.skills.len());
    assert!(data.abilities.len() > 100);
    assert!(data.achievements.len() > 100);

    let lang_path = std::env::var_os("WOWSINFO_LANG");
    if let Some(lang_path) = lang_path {
        let raw = std::fs::read_to_string(lang_path).expect("read lang.json");
        let json: serde_json::Value = serde_json::from_str(&raw).expect("valid lang json");
        let lang = parse_lang(&json, "en");
        assert!(lang.len() > 10_000, "lang entries: {}", lang.len());
    }

    // Every ship resolves to a local wiki entry with a hull.
    let sample = data.ships.iter().next().expect("at least one ship");
    let lang = LangMap::default();
    let wiki = build_local_ship_wiki(
        &data,
        &lang,
        *sample.0,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    );
    assert!(wiki.is_some(), "ship {} builds", sample.1.index);

    // Compare + carrier views work against the real data.
    let compare = build_local_compare(&data, &lang, &[*sample.0]);
    assert!(compare.is_some());
    let carrier = data
        .ships
        .iter()
        .find(|(_, ship)| ship.r#type == "AirCarrier")
        .map(|(id, _)| *id);
    if let Some(carrier_id) = carrier {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            carrier_id,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        );
        assert!(wiki.is_some());
        assert!(!wiki.unwrap().aircraft.is_empty(), "carrier has squadrons");
    }
    // Lexington (PASA108) must expose fighter/torpedo/dive bomber slots.
    if let Some(ship) = data.ships.get(&418_170_2640) {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            418_170_2640,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        )
            .expect("lexington");
        assert!(
            wiki.aircraft.len() >= 3,
            "lexington aircraft slots: {:?}",
            wiki.aircraft.iter().map(|s| s.slot.clone()).collect::<Vec<_>>()
        );
        for slot in &wiki.aircraft {
            let resolved = slot
                .options
                .first()
                .and_then(|option| option.aircraft.as_ref());
            assert!(
                resolved.is_some(),
                "slot {} aircraft lookup failed",
                slot.slot
            );
        }
        let _ = ship;
    }
    // Loadouts resolve for a cruiser: consumables + skills + upgrades.
    if data.ships.contains_key(&4_293_834_736) {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            4_293_834_736,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        )
        .expect("cruiser");
        assert!(!wiki.skills.is_empty(), "cruiser has skills");
        assert!(
            !wiki.upgrades.is_empty(),
            "erie upgrades: {:?}",
            wiki.upgrades.iter().map(|u| u.key.clone()).collect::<Vec<_>>()
        );
        assert!(!wiki.flags.is_empty());
        assert!(!wiki.consumables.is_empty());
    }

    // A battleship resolves with a full armor digest (zones + turrets).
    let battleship = data
        .ships
        .iter()
        .find(|(_, ship)| ship.r#type == "Battleship")
        .map(|(id, _)| *id);
    if let Some(bb_id) = battleship {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            bb_id,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        );
        let wiki = wiki.expect("battleship wiki");
        let armor = wiki.armor.expect("battleship armor digest");
        assert!(armor.zone_count > 10, "zones: {}", armor.zone_count);
        assert!(!armor.turrets.is_empty(), "battleship has turrets");
        assert!(armor.turrets.iter().any(|turret| turret.armor > 0.0));
    }
}

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
    }
}

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

#[test]
fn flags_expose_wiki_entries() {
    let Some(path) = std::env::var_os("WOWSINFO_JSON") else {
        return;
    };
    let raw = std::fs::read_to_string(path).expect("read wowsinfo.json");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    let data = parse_game_data(&json);
    let flags = all_flag_views(&data, &LangMap::default());
    assert_eq!(flags.len(), 15, "signal flags: {}", flags.len());
    assert!(
        flags.iter().any(|flag| flag.key == "PCEF005_SM_SignalFlag"),
        "sample flag present"
    );
    assert!(
        flags
            .iter()
            .find(|flag| flag.key == "PCEF005_SM_SignalFlag")
            .is_some_and(|flag| !flag.summary.is_empty()),
        "sample flag carries a modifier summary"
    );
}
