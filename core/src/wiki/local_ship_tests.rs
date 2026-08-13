//! Tests for the local ship wiki assembly, using a representative fixture.

use serde_json::json;

use super::gamedata::parse_game_data;
use super::{build_local_ship_wiki, LangMap, ModuleSelection};

fn test_data() -> super::gamedata::GameData {
    let json = json!({
        "ships": {
            "1": {
                "id": 1, "index": "TEST001", "name": "IDS_TEST", "description": "IDS_D",
                "year": "IDS_Y", "paperShip": false, "tier": 8, "region": "USA",
                "type": "Cruiser", "regionID": "IDS_USA", "typeID": "IDS_CRUISER",
                "group": "normal", "costXP": 0, "costGold": 0, "costCR": 100,
                "consumables": [], "nextShips": [2],
                "modules": {
                    "_Hull": [{
                        "index": 0, "name": "IDS_HULL", "cost": {"costXP": 0, "costCR": 0},
                        "components": {"hull": ["A_Hull"], "artillery": ["A1_Guns"],
                                       "airDefense": [], "torpedoes": []}
                    }]
                },
                "components": {
                    "A_Hull": {"health": 30000.0, "protection": 4.0,
                               "mobility": {"speed": 32.0, "turningRadius": 660.0, "rudderTime": 9.0},
                               "visibility": {"sea": 11.5, "plane": 6.0}},
                    "A1_Guns": {"range": 14699.0, "sigma": 2.0, "guns": [{
                        "reload": 15.0, "rotation": 25.7, "each": 3,
                        "ammo": ["PAPA_AP"], "vertSector": 41.0, "count": 3}]}
                }
            },
            "2": {
                "id": 2, "index": "TEST002", "name": "IDS_T2", "description": "",
                "year": "", "paperShip": false, "tier": 8, "region": "USA",
                "type": "Cruiser", "regionID": "IDS_USA", "typeID": "IDS_CRUISER",
                "group": "normal", "costXP": 0, "costGold": 0, "costCR": 100,
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
fn builds_local_ship_wiki() {
    let data = test_data();
    let lang = LangMap::from_entries([
        ("IDS_TEST".to_string(), "Test Ship".to_string()),
        ("IDS_HULL".to_string(), "Hull A".to_string()),
        ("IDS_AP".to_string(), "203 mm AP".to_string()),
        ("IDS_CRUISER".to_string(), "Cruiser".to_string()),
        ("IDS_USA".to_string(), "U.S.A.".to_string()),
    ]);
    let wiki = build_local_ship_wiki(
        &data,
        &lang,
        1,
        ModuleSelection::default(),
        &super::LocalBuildConfig::default(),
    )
        .expect("ship");
    assert_eq!(wiki.name, "Test Ship");
    assert_eq!(wiki.ship_type, "Cruiser");
    assert_eq!(wiki.region, "U.S.A.");
    assert_eq!(wiki.similar_ships.len(), 1);
    assert_eq!(wiki.similar_ships[0].index, "TEST002");
    let mb = wiki.main_battery.expect("main battery");
    assert_eq!(mb.configuration, "3 x 3");
    assert_eq!(mb.shells.len(), 1);
    assert_eq!(mb.shells[0].name, "203 mm AP");
    assert_eq!(wiki.penetration_curves.len(), 1);
    assert!(wiki.penetration_curves[0].points.len() >= 10);
    let hull = wiki.hull.expect("hull");
    assert_eq!(hull.health, 30000.0);
    assert_eq!(hull.mobility.speed, 32.0);
    assert_eq!(hull.visibility.sea, 11.5);
}
