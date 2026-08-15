//! Game-data parser tests (fixtures + real bundle).

use serde_json::json;

use super::super::*;

#[test]
fn zstd_decompress_roundtrip() {
    let input = r#"{"ships":{},"projectiles":{}}"#;
    let compressed = zstd::stream::encode_all(input.as_bytes(), 3).expect("compress");
    assert_eq!(decompress_zstd(&compressed).expect("decompress"), input);
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
