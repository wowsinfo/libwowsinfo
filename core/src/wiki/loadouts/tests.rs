//! Loadout view tests.

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::json;
    use super::super::{
        combined_modifiers, consumable_views, flag_views, skill_views, upgrade_views,
        LocalBuildConfig,
    };
    use crate::wiki::gamedata::{parse_game_data, GameData};
    use crate::wiki::LangMap;

    fn test_data() -> GameData {
        let json = json!({
            "ships": {
                "1": {
                    "id": 1, "index": "T1", "name": "IDS_N", "description": "",
                    "year": "", "paperShip": false, "tier": 8, "region": "USA",
                    "type": "Cruiser", "regionID": "IDS_USA", "typeID": "IDS_CRUISER",
                    "group": "normal", "costXP": 0, "costGold": 0, "costCR": 0,
                    "consumables": [[{"name": "PCY006_SmokeGenerator", "type": "C_TierOne"}]],
                    "nextShips": [],
                    "modules": {}, "components": {}
                }
            },
            "abilities": {
                "PCY006_SmokeGenerator": {
                    "id": 9001, "icon": "PCY006_SmokeGenerator",
                    "name": "IDS_SMOKE", "description": "IDS_SMOKE_DESC",
                    "abilities": {
                        "Cruiser": {"reloadTime": 120.0, "workTime": 30.0, "numConsumables": 3}
                    }
                }
            },
            "skills": {
                "TriggerGmReload": {
                    "id": 9002,
                    "name": "IDS_SKILL", "description": "IDS_SKILL_DESC",
                    "tier": {"Cruiser": 3},
                    "modifiers": {},
                    "LogicTrigger": {
                        "triggerType": "entityIsVisibleTrigger",
                        "modifiers": {"GMShotDelay": 0.9}
                    }
                }
            },
            "modernizations": {
                "PCM001": {
                    "id": 9003,
                    "name": "IDS_U", "description": "IDS_U_DESC", "slot": 1,
                    "costCR": 500, "level": [8], "type": ["Cruiser"], "nation": ["USA"],
                    "modifiers": {"GMShotDelay": 0.88}
                }
            },
            "exteriors": {
                "PCEF005_SM_SignalFlag": {
                    "id": 9004,
                    "type": "Flags", "name": "IDS_F", "description": "IDS_F_DESC",
                    "costCR": 1000, "modifiers": {"speedCoef": 1.05}
                }
            }
        });
        parse_game_data(&json)
    }

    #[test]
    fn builds_loadout_views() {
        let data = test_data();
        let ship = &data.ships[&1];
        let lang = LangMap::from_entries([
            ("IDS_SMOKE".to_string(), "Smoke Generator".to_string()),
            ("IDS_SMOKE_DESC".to_string(), "Smoke".to_string()),
            ("IDS_SKILL".to_string(), "Rapid Reload".to_string()),
            ("IDS_U".to_string(), "Reload Mod 1".to_string()),
            ("IDS_F".to_string(), "SM Flag".to_string()),
        ]);
        let consumables = consumable_views(&data, &lang, ship);
        assert_eq!(consumables.len(), 1);
        assert_eq!(consumables[0].name, "Smoke Generator");
        assert_eq!(consumables[0].reload_s, 120.0);
        assert_eq!(consumables[0].charges, 3);

        let skills = skill_views(&data, &lang, ship, &HashSet::new());
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].tier, 3);
        assert_eq!(skills[0].trigger_type, "entityIsVisibleTrigger");
        assert!(
            skills[0].summary.contains("While spotted"),
            "trigger summary: {}",
            skills[0].summary
        );

        let upgrades = upgrade_views(&data, &lang, ship, &HashSet::new());
        assert_eq!(upgrades.len(), 1);
        assert_eq!(upgrades[0].slot, 1);

        let flags = flag_views(&data, &lang, ship, &HashSet::new());
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0].summary, "speedCoef +5%");

        let config = LocalBuildConfig {
            skills: HashSet::from(["TriggerGmReload".to_string()]),
            upgrades: HashSet::from(["PCM001".to_string()]),
            flags: HashSet::from(["PCEF005_SM_SignalFlag".to_string()]),
            hp_fraction: 1.0,
            spotted: true,
        };
        let mods = combined_modifiers(&data, ship, &config);
        assert!((mods.multiply("Cruiser", "GMShotDelay") - 0.88 * 0.9).abs() < 1e-9);
        assert!((mods.multiply("Cruiser", "speedCoef") - 1.05).abs() < 1e-9);

        let unspotted = LocalBuildConfig {
            spotted: false,
            ..config
        };
        let mods = combined_modifiers(&data, ship, &unspotted);
        assert!((mods.multiply("Cruiser", "GMShotDelay") - 0.88).abs() < 1e-9);
    }
}
