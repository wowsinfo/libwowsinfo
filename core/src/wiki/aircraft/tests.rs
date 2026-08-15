//! Aircraft parser tests.

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::super::parse::parse_aircrafts;

    #[test]
    fn parses_bombers_and_fighters() {
        let json = json!({
            "PAAB001_Douglas_TBD": {
                "type": "Bomber", "nation": "USA",
                "name": "IDS_PAAB001_DOUGLAS_TBD",
                "health": 12.1, "totalPlanes": 1, "visibility": 10.0, "speed": 130.0
            },
            "PBAL001_M_THESEUS": {
                "type": "Skip", "nation": "United_Kingdom",
                "name": "IDS_PBAL001_M_THESEUS",
                "health": 2490.0, "totalPlanes": 6, "visibility": 10.0, "speed": 182.0,
                "aircraft": {
                    "restoreTime": 77.0, "maxAircraft": 12, "attacker": 3,
                    "attackCount": 1, "cooldown": 9, "bombName": "PBPS727_M_THESEUS"
                }
            }
        });
        let map = parse_aircrafts(&json);
        let bomber = &map["PAAB001_Douglas_TBD"];
        assert_eq!(bomber.r#type, "Bomber");
        assert!(bomber.attack_count.is_none());
        let skip = &map["PBAL001_M_THESEUS"];
        assert_eq!(skip.attack_count, Some(1));
        assert_eq!(skip.attacker, Some(3));
        assert_eq!(skip.max_aircraft, Some(12));
        assert_eq!(skip.restore_time, Some(77.0));
        assert_eq!(skip.bomb_name.as_deref(), Some("PBPS727_M_THESEUS"));
    }

    #[test]
    fn tolerant_of_missing_fields() {
        let map = parse_aircrafts(&json!({ "x": {"name": "IDS_X"} }));
        let entry = &map["x"];
        assert_eq!(entry.health, 0.0);
        assert!(entry.bomb_name.is_none());
    }

    #[test]
    fn parses_flattened_v15_7_squadron_fields() {
        let json = json!({
            "PAAB999_Flat": {
                "type": "Bomber", "nation": "USA",
                "name": "IDS_PAAB999_FLAT",
                "health": 2200.0, "totalPlanes": 9, "visibility": 10.0, "speed": 130.0,
                "attackCount": 2, "attackerSize": 3,
                "bombName": "PAPT910_Mk_13_mod0A_Independence",
                "restorationTime": 60.0
            }
        });
        let map = parse_aircrafts(&json);
        let bomber = &map["PAAB999_Flat"];
        assert_eq!(bomber.attack_count, Some(2));
        assert_eq!(bomber.attacker, Some(3));
        assert_eq!(bomber.restore_time, Some(60.0));
        assert_eq!(
            bomber.bomb_name.as_deref(),
            Some("PAPT910_Mk_13_mod0A_Independence")
        );
    }

    #[test]
    fn parses_full_squadron_fields() {
        let json = json!({
            "PAAB052_Grumman_TBM3": {
                "type": "Bomber", "nation": "USA",
                "name": "IDS_PAAB052_GRUMMAN_TBM3",
                "health": 12.1, "totalPlanes": 6, "visibility": 10.0, "speed": 130.0,
                "attackCooldown": 5, "attackInterval": 7.5,
                "aimingTime": 15.0, "aimingSpeedLimits": [0.5, 1.5],
                "preparationTime": 1.0, "preparationSpeedLimits": [0.5, 1.5],
                "angleOfClimb": 35.0, "angleOfDive": -35.0,
                "jatoDuration": 0.0, "maxForsageAmount": 2,
                "maxNumberOnDeck": 0, "restorationTime": 0.1,
                "bombName": "PAPT910_Mk_13_mod0A_Independence",
                "planeConsumables": {
                    "AbilitySlot0": {
                        "abils": [["PCY034_ForsageBooster", "Allplanes"]],
                        "isSpecial": false, "slot": 0
                    },
                    "AbilitySlot1": {
                        "abils": [], "isSpecial": true, "slot": 1
                    }
                }
            }
        });
        let map = parse_aircrafts(&json);
        let bomber = &map["PAAB052_Grumman_TBM3"];
        assert_eq!(bomber.attack_cooldown, Some(5.0));
        assert_eq!(bomber.attack_interval, Some(7.5));
        assert_eq!(bomber.aiming_time, Some(15.0));
        assert_eq!(bomber.aiming_speed_limit_min, Some(0.5));
        assert_eq!(bomber.aiming_speed_limit_max, Some(1.5));
        assert_eq!(bomber.angle_of_climb, Some(35.0));
        assert_eq!(bomber.angle_of_dive, Some(-35.0));
        assert_eq!(bomber.jato_duration, Some(0.0));
        assert_eq!(bomber.max_forsage_amount, Some(2.0));
        assert_eq!(bomber.max_number_on_deck, Some(0));
        assert_eq!(bomber.restoration_time, Some(0.1));
        assert_eq!(bomber.plane_consumables.len(), 2);
        assert_eq!(bomber.plane_consumables[0].abilities, vec!["PCY034_ForsageBooster"]);
        assert!(bomber.plane_consumables[1].special);
    }
}
