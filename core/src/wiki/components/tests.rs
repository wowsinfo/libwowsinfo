//! Component parser tests.

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_and_burst_parse() {
        let json = serde_json::json!({
            "range": 14699.0, "sigma": 2.0,
            "guns": [{"reload": 15.0, "rotation": 25.7, "each": 3,
                      "ammo": ["PAPA002", "PAPA001"], "vertSector": 41.0, "count": 3}],
            "burst": {"burstReloadTime": 4.0, "fullReloadTime": 30.0,
                      "shotIntensity": 0.5, "shotsCount": 4}
        });
        let guns = parse_guns(&json);
        assert_eq!(guns.range_m, 14699.0);
        assert_eq!(guns.guns.len(), 1);
        assert_eq!(guns.guns[0].each, 3);
        assert_eq!(guns.guns[0].ammo, vec!["PAPA002", "PAPA001"]);
        let burst = guns.burst.expect("burst");
        assert_eq!(burst.shots_count, 4);
    }

    #[test]
    fn hull_and_aa_parse() {
        let hull = serde_json::json!({
            "health": 30500.0, "protection": 4.0,
            "mobility": {"speed": 32.5, "turningRadius": 660.0, "rudderTime": 9.0},
            "visibility": {"sea": 11.5, "plane": 6.0},
            "submarineBattery": {"capacity": 10, "regen": 0.5}
        });
        let parsed = parse_hull(&hull);
        assert_eq!(parsed.health, 30500.0);
        assert_eq!(parsed.mobility.speed, 32.5);
        assert_eq!(parsed.submarine_battery.as_ref().map(|b| b.capacity), Some(10));

        let aa = serde_json::json!({
            "medium": [{"minRange": 0.1, "maxRange": 3.5, "hitChance": 0.9,
                        "damage": 37.0, "rof": 0.29, "dps": 129.5,
                        "guns": [{"ammo": "PAGA002", "each": 2, "reload": 5.0,
                                  "name": "IDS_X", "count": 6}]}]
        });
        let stats = parse_air_defense(&aa);
        assert_eq!(stats.medium.len(), 1);
        assert_eq!(stats.medium[0].dps, 129.5);
        assert_eq!(stats.medium[0].guns[0].count, 6);
    }

    #[test]
    fn parses_hull_survivability_sections() {
        let hull = serde_json::json!({
            "health": 29000.0,
            "protection": 19.0,
            "survivability": {
                "sections": {
                    "hull": {"hp": 21800.0, "regenRatio": 0.5, "autoRepairTime": 10},
                    "citadel": {"hp": 165500.0, "regenRatio": 0.1, "autoRepairTime": 10},
                    "auxiliaryRooms": {"hp": 37700.0, "regenRatio": 0.5, "autoRepairTime": 40}
                },
                "fire": {"spots": 4, "chance": 1.0, "duration": 60.0, "dps": 87.0, "totalDamage": 5220.0},
                "flood": {"spots": 2, "chance": 0.27, "duration": 40.0, "dps": 145.0, "totalDamage": 5800.0}
            }
        });
        let stats = parse_hull(&hull);
        let surv = stats.survivability.expect("survivability");
        assert_eq!(surv.sections.len(), 3);
        assert_eq!(surv.sections[0].name, "Citadel");
        assert_eq!(surv.sections[0].hp, 165500.0);
        assert_eq!(surv.sections[0].regen_ratio, 0.1);
        assert_eq!(surv.sections[2].name, "Auxiliary Rooms");
        let fire = surv.fire.expect("fire");
        assert_eq!(fire.spots, 4);
        assert_eq!(fire.total_damage, 5220.0);
    }

    #[test]
    fn parses_hull_maneuverability_and_concealment() {
        let hull = serde_json::json!({
            "mobility": {"speed": 27.0, "turningRadius": 560.0, "rudderTime": 6.4},
            "maneuverability": {
                "maxReverseSpeed": 11.65,
                "submarine": {
                    "maxSpeedAtSurface": 27.0, "maxReverseSpeedAtSurface": 11.65,
                    "maxSpeedAtPeriscope": 27.0, "maxReverseSpeedAtPeriscope": 11.65,
                    "maxSpeedAtMaxDepth": 12.99, "maxReverseSpeedAtMaxDepth": 5.6,
                    "maxDiveSpeed": 2.5, "divingPlaneShiftTime": 20.84
                }
            },
            "visibility": {"sea": 6.4, "plane": 2.3},
            "concealment": {
                "seaFire": 2.0, "airFire": 1.0,
                "bySubmarineDepth": {"PERISCOPE": 6.0, "DEEP_WATER": 2.0},
                "visibilityFactorInSmoke": 0.000001
            }
        });
        let stats = parse_hull(&hull);
        let man = stats.maneuverability.expect("maneuverability");
        assert_eq!(man.max_reverse_speed, 11.65);
        let sub = man.submarine.expect("submarine");
        assert_eq!(sub.periscope_speed, 27.0);
        assert_eq!(sub.dive_speed, 2.5);
        let concealment = stats.concealment.expect("concealment");
        assert_eq!(concealment.sea_fire, 2.0);
        assert_eq!(concealment.periscope_depth, 6.0);
        assert_eq!(concealment.deep_water_depth, 2.0);
    }

    #[test]
    fn parses_turrets_and_hull_armor() {
        let guns = serde_json::json!({
            "range": 14240.0,
            "turrets": [
                {"name": "HP_AGM_1", "caliber": 0.406, "barrels": 3,
                 "armor": {"65568": 203.0, "65636": 305.0}},
                {"name": "HP_AGM_2", "caliber": 0.406, "barrels": 3, "armor": 406.0}
            ]
        });
        let stats = parse_guns(&guns);
        assert_eq!(stats.turrets.len(), 2);
        assert_eq!(stats.turrets[0].caliber, 0.406);
        assert_eq!(stats.turrets[0].armor, 305.0, "max of turret zone armor");
        assert_eq!(stats.turrets[1].armor, 406.0);

        let hull = serde_json::json!({
            "armor": {
                "zones": {"1": 16.0, "65568": 203.0, "65636": 305.0},
                "barbettes": {
                    "HP_AGM_1": [65636, 65568],
                    "HP_AGM_2": [1]
                }
            }
        });
        let parsed = parse_hull(&hull);
        let armor = parsed.armor.expect("armor");
        assert_eq!(armor.zones.len(), 3);
        assert_eq!(armor.zones[0].thickness, 305.0, "sorted by thickness desc");
        assert_eq!(
            armor.barbettes[0].max_thickness, 305.0,
            "barbette resolves zone ids through the zone map"
        );
    }

    #[test]
    fn parses_new_aura_blocks_and_legacy_bubbles() {
        // v15.7 AirDefense component: rich auras + legacy gun mounts.
        let air_defense = serde_json::json!({
            "medium": [{
                "minRange": 0.1, "maxRange": 3.0, "hitChance": 0.75,
                "damage": 3.0, "rof": 0.29, "dps": 10.5,
                "guns": [{"ammo": "PJGA119", "each": 1, "reload": 5.0,
                          "name": "IDS_PJGA119", "count": 4}]
            }],
            "antiAir": {"auras": {
                "medium": [{
                    "minRange": 0.1, "maxRange": 3.0, "hitChance": 0.75,
                    "areaDamage": 3.0, "areaDamagePeriod": 0.285714285714,
                    "explosionCount": 15, "shotDelay": 0.5,
                    "shotTravelTime": 1.5, "bubbleDamage": 0.0,
                    "innerBubbleCount": 0, "outerBubbleCount": 0,
                    "bubbleRadius": 1.0, "bubbleDuration": 4.75,
                    "enableBarrage": true, "dps": 10.5
                }]
            }}
        });
        let stats = parse_air_defense(&air_defense);
        assert!(stats.near.is_empty());
        assert_eq!(stats.medium.len(), 1);
        let aura = &stats.medium[0];
        assert_eq!(aura.explosion_count, 15);
        assert_eq!(aura.shot_travel_time, 1.5);
        assert_eq!(aura.bubble_duration, 4.75);
        assert!(aura.enable_barrage);
        assert_eq!(aura.guns.len(), 1, "legacy gun mounts attached");
        assert_eq!(aura.guns[0].count, 4);

        // ATBA component: legacy band + aggregate bubbles block.
        let atba = serde_json::json!({
            "far": [{
                "minRange": 0.1, "maxRange": 5.8, "hitChance": 1.0,
                "damage": 7.0, "rof": 0.29, "dps": 24.5,
                "guns": [{"ammo": "PAGS022", "each": 1, "reload": 6.0,
                          "name": "IDS_PAGS022", "count": 2}]
            }],
            "bubbles": {
                "inner": 1, "outer": 0, "rof": 5.0, "minRange": 3.5,
                "maxRange": 5.8, "hitChance": 1.0, "spawnTime": 1.51,
                "damage": 1260.0
            }
        });
        let stats = parse_air_defense(&atba);
        assert_eq!(stats.far.len(), 1);
        assert_eq!(stats.far[0].dps, 24.5);
        let bubbles = stats.bubbles.expect("bubbles");
        assert_eq!(bubbles.inner, 1);
        assert_eq!(bubbles.outer, 0);
        assert_eq!(bubbles.damage, 1260.0);
        assert_eq!(bubbles.spawn_time, 1.51);
    }

    #[test]
    fn parses_structured_rage_mode() {
        let special = serde_json::json!({
            "rageMode": {
                "GameLogicTrigger": {
                    "Action": {"progress": 6.0, "progressName": "main_gun_hit"},
                    "Activator": {"requiredCount": 1, "subRibbons": [14, 15, 16],
                                  "timeLimit": 0.0, "separateTracking": true},
                    "startEnabled": true
                },
                "boostDuration": 45.0, "boostPreparation": 0.0,
                "decrementCount": 5.0, "decrementDelay": 50.0,
                "decrementPeriod": 1.0, "isAutoUsage": false,
                "rageModeName": "survivability",
                "modifiers": {"AAAuraDamage": {"Battleship": 1.25},
                              "vulnerabilityBurn": 0.35}
            },
            "specialAbility": {"rage": {
                "mode": "rage", "progressPerAction": 6.0,
                "progressName": "main_gun_hit", "requiredCount": 1,
                "subRibbons": [14, 15, 16, 17, 28], "timeLimit": 0.0,
                "separateTracking": true, "startEnabled": true,
                "name": "survivability", "duration": 45.0, "preparation": 0.0,
                "inactivityDelay": 50.0, "progressLossInterval": 1.0,
                "progressLossPerInterval": 5.0, "autoUsage": false,
                "modifiers": {"AAAuraDamage": {"Battleship": 1.25},
                              "vulnerabilityBurn": 0.35}
            }}
        });
        let stats = parse_special(&special).expect("special ability");
        assert_eq!(stats.mode, "survivability");
        assert_eq!(stats.boost_duration, 45.0);
        assert_eq!(stats.progress_per_action, 6.0);
        assert_eq!(stats.progress_name, "main_gun_hit");
        assert_eq!(stats.required_count, 1);
        assert_eq!(stats.sub_ribbons, vec![14, 15, 16, 17, 28]);
        assert!(stats.separate_tracking);
        assert!(stats.start_enabled);
        assert_eq!(stats.decrement_delay, 50.0);
        assert_eq!(stats.decrement_period, 1.0);
        assert_eq!(stats.decrement_count, 5.0);
        assert!(!stats.auto_usage);
        assert_eq!(stats.modifiers.entries.len(), 2);
    }
}
