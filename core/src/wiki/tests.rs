//! Tests for the local wiki-data parsers, using representative fixtures in the
//! exact shapes produced by `wows-constants` and WoWs-Game-Data.

use serde_json::json;

use super::*;

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
