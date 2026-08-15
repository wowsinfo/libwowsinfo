//! Comparison table tests.

use super::*;
use crate::wiki::gamedata::{parse_game_data, GameData};
use crate::wiki::LangMap;
use serde_json::json;

fn data_with_two_ships() -> GameData {
    let json = json!({
        "ships": {
            "1": {
                "id": 1, "index": "TEST001", "name": "IDS_A", "description": "",
                "year": "", "paperShip": false, "tier": 8, "region": "USA",
                "type": "Cruiser", "regionID": "IDS_USA", "typeID": "IDS_CRUISER",
                "group": "normal", "costXP": 0, "costGold": 0, "costCR": 0,
                "consumables": [], "nextShips": [],
                "modules": {
                    "_Hull": [{
                        "index": 0, "name": "IDS_H", "cost": {"costXP": 0, "costCR": 0},
                        "components": {"hull": ["H1"], "artillery": ["G1"]}
                    }]
                },
                "components": {
                    "H1": {"health": 30000.0, "protection": 4.0,
                           "mobility": {"speed": 32.0, "turningRadius": 660.0, "rudderTime": 9.0},
                           "visibility": {"sea": 11.5, "plane": 6.0}},
                    "G1": {"range": 14699.0, "sigma": 2.0, "guns": [{
                        "reload": 15.0, "rotation": 25.7, "each": 3,
                        "ammo": ["PAPA_AP"], "vertSector": 41.0, "count": 3}]}
                }
            },
            "2": {
                "id": 2, "index": "TEST002", "name": "IDS_B", "description": "",
                "year": "", "paperShip": false, "tier": 8, "region": "USA",
                "type": "Cruiser", "regionID": "IDS_USA", "typeID": "IDS_CRUISER",
                "group": "normal", "costXP": 0, "costGold": 0, "costCR": 0,
                "consumables": [], "nextShips": [],
                "modules": {}, "components": {}
            }
        },
        "projectiles": {
            "PAPA_AP": {
                "type": "Artillery", "nation": "USA", "name": "IDS_AP",
                "ammoType": "AP", "speed": 800.0, "weight": 120.0, "damage": 5000.0,
                "diameter": 0.203,
                "ap": {"diameter": 0.203, "weight": 120.0, "drag": 0.3,
                       "velocity": 800.0, "krupp": 2400.0}
            }
        }
    });
    parse_game_data(&json)
}

#[test]
fn builds_comparison_table() {
    let data = data_with_two_ships();
    let lang = LangMap::from_entries([
        ("IDS_A".to_string(), "Ship A".to_string()),
        ("IDS_B".to_string(), "Ship B".to_string()),
        ("IDS_CRUISER".to_string(), "Cruiser".to_string()),
        ("IDS_USA".to_string(), "U.S.A.".to_string()),
    ]);
    let compare = build_local_compare(&data, &lang, &[1, 2]).expect("compare");
    assert_eq!(compare.ships.len(), 2);
    assert_eq!(compare.ships[0].name, "Ship A");
    assert_eq!(compare.rows.len(), 15);
    let health = compare.rows.iter().find(|r| r.label == "Health").expect("row");
    assert_eq!(health.values[0], "30000");
    assert_eq!(health.values[1], "-");
    let range = compare.rows.iter().find(|r| r.label == "Gun range").expect("row");
    assert_eq!(range.values[0], "14.7 km");
}

#[test]
fn empty_input_returns_none() {
    let data = data_with_two_ships();
    let lang = LangMap::default();
    assert!(build_local_compare(&data, &lang, &[]).is_none());
    assert!(build_local_compare(&data, &lang, &[999]).is_none());
}
