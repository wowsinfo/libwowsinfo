//! Achievement / upgrade view tests.

use serde_json::json;

use super::super::*;

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
