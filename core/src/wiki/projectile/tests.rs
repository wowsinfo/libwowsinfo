//! Projectile parser tests.

use serde_json::json;
use super::parse::parse_projectiles;


#[test]
fn parses_ap_and_he_shells() {
    let json = json!({
        "PAPA011_Shell_406mm_AP_AP_Mk_8": {
            "type": "Artillery", "nation": "USA",
            "name": "IDS_PAPA011_SHELL_406MM_AP_AP_MK_8",
            "ammoType": "AP", "speed": 701.0, "weight": 1225.0,
            "damage": 13100.0, "ricochetAngle": 45.0,
            "ricochetAlways": 60.0, "diameter": 0.406,
            "ap": {"diameter": 0.406, "weight": 1225.0, "drag": 0.352,
                   "velocity": 701.0, "krupp": 2598.0},
            "overmatch": 28, "fuseTime": 0.033,
            "airDrag": 0.352, "armingThreshold": 68.0,
            "capNormalizeMaxAngle": 6.0, "explosionRadius": 200.0,
            "krupp": 2598.0, "shellCap": true, "splashRadius": 2.15,
            "underwaterDistFactor": 0.8, "underwaterPenetrationFactor": 0.9,
            "waterDrag": 10.0, "distParams": [-0.12, 0.1, 0.12, 0.0],
            "distTile": 3.0
        },
        "PAPA002_Shell_203mm_HE_HC_Mk_25": {
            "type": "Artillery", "nation": "USA",
            "name": "IDS_PAPA002_SHELL_203MM_HE_HC_MK_25",
            "ammoType": "HE", "speed": 823.0, "weight": 118.0,
            "damage": 2800.0, "penHE": 34.0, "burnChance": 0.14,
            "diameter": 0.203
        }
    });
    let map = parse_projectiles(&json);
    assert_eq!(map.len(), 2);
    let ap = &map["PAPA011_Shell_406mm_AP_AP_Mk_8"];
    assert_eq!(ap.ammo_type, "AP");
    assert_eq!(ap.overmatch, Some(28));
    assert_eq!(ap.calibre_mm(), 406.0);
    let ap_block = ap.ap.as_ref().expect("ap block");
    assert_eq!(ap_block.krupp, 2598.0);
    assert_eq!(ap_block.drag, 0.352);
    assert_eq!(ap.air_drag, Some(0.352));
    assert_eq!(ap.arming_threshold, Some(68.0));
    assert_eq!(ap.cap_normalize_max_angle, Some(6.0));
    assert_eq!(ap.explosion_radius, Some(200.0));
    assert_eq!(ap.krupp, Some(2598.0));
    assert_eq!(ap.shell_cap, Some(true));
    assert_eq!(ap.splash_radius, Some(2.15));
    assert_eq!(ap.underwater_dist_factor, Some(0.8));
    assert_eq!(ap.underwater_penetration_factor, Some(0.9));
    assert_eq!(ap.water_drag, Some(10.0));
    assert_eq!(ap.dist_params, vec![-0.12, 0.1, 0.12, 0.0]);
    assert_eq!(ap.dist_tile, Some(3.0));
    let he = &map["PAPA002_Shell_203mm_HE_HC_Mk_25"];
    assert_eq!(he.pen_he, Some(34.0));
    assert_eq!(he.burn_chance, Some(0.14));
    assert!(he.ap.is_none());
}

#[test]
fn parses_torpedoes() {
    let json = json!({
        "PAPT001_Torpedo_533mm_Mk_15": {
            "type": "Torpedo", "nation": "USA",
            "name": "IDS_PAPT001_TORPEDO_533MM_MK_15",
            "speed": 55.0, "visibility": 1.1, "range": 305.0,
            "floodChance": 190.0, "alphaDamage": 32100.0,
            "damage": 900.0, "deepWater": false,
            "armingDistance": 55.0, "depth": 0.14,
            "splashArmorCoeff": 0.3, "splashCubeSize": 1.0,
            "damageCoeffMaxPing": 2.0, "alertDist": 100.0,
            "canHitClasses": ["Battleship", "Cruiser", "Destroyer"],
            "acousticDetection": {
                "countdown": 1.0, "maxDepthLevel": 5.0, "maxPitch": 5.0,
                "maxYaw": 5.0, "pathLength": 500, "searchAngle": 45,
                "searchRadius": 2000, "speedDecrCoef": 0.8,
                "verticalAcceleration": 5.0, "yawChangeSpeed": 5.0
            }
        }
    });
    let map = parse_projectiles(&json);
    let torp = &map["PAPT001_Torpedo_533mm_Mk_15"];
    assert_eq!(torp.r#type, "Torpedo");
    assert_eq!(torp.range, Some(305.0));
    assert_eq!(torp.flood_chance, Some(190.0));
    assert_eq!(torp.visibility, Some(1.1));
    assert_eq!(torp.alpha_damage, Some(32100.0));
    assert_eq!(torp.arming_distance, Some(55.0));
    assert_eq!(torp.depth, Some(0.14));
    assert_eq!(torp.splash_armor_coeff, Some(0.3));
    assert_eq!(torp.damage_coeff_max_ping, Some(2.0));
    assert_eq!(torp.alert_dist, Some(100.0));
    assert_eq!(torp.can_hit_classes, vec!["Battleship", "Cruiser", "Destroyer"]);
    let acoustic = torp.acoustic_detection.as_ref().expect("acoustic block");
    assert_eq!(acoustic.search_radius, 2000.0);
    assert_eq!(acoustic.speed_decr_coef, 0.8);
}

#[test]
fn tolerant_of_missing_fields() {
    let json = json!({ "weird": {"name": "IDS_X"} });
    let map = parse_projectiles(&json);
    let entry = &map["weird"];
    assert_eq!(entry.damage, 0.0);
    assert!(entry.ap.is_none());
    assert!(entry.pen_he.is_none());
}

#[test]
fn parses_depth_charge_ammo() {
    let json = json!({
        "PAPD004_mk6_shoot": {
            "type": "DepthCharge", "nation": "USA",
            "name": "IDS_PAPD004_MK6_SHOOT",
            "damage": 3800.0, "burnChance": 0.15,
            "floodChance": 23.0, "sinkSpeed": 300.0,
            "detonationDepth": -80, "splashRadius": 26.67,
            "floodGeneration": true,
            "pointsOfDamage": [[0.0, 1.0], [0.15, 1.0], [0.151, 0.33], [1.0, 0.33]],
            "ignoreClasses": ["AirCarrier", "Battleship", "Cruiser", "Destroyer", "Auxiliary"],
            "alertDist": 100.0, "explosivePower": 100.0, "integralPower": 10.0,
            "fallDistance": 20.0, "fallTime": 20.0,
            "buoyancyToDamageCoeff": {
                "DEEP_WATER": 1.0, "PERISCOPE": 1.0, "SURFACE": 1.0
            }
        }
    });
    let map = parse_projectiles(&json);
    let dc = &map["PAPD004_mk6_shoot"];
    assert_eq!(dc.r#type, "DepthCharge");
    assert_eq!(dc.damage, 3800.0);
    assert_eq!(dc.burn_chance, Some(0.15));
    assert_eq!(dc.flood_chance, Some(23.0));
    assert_eq!(dc.sink_speed, Some(300.0));
    assert_eq!(dc.detonation_depth, Some(-80.0));
    assert_eq!(dc.splash_radius, Some(26.67));
    assert_eq!(dc.flood_generation, Some(true));
    assert_eq!(dc.points_of_damage, vec![(0.0, 1.0), (0.15, 1.0), (0.151, 0.33), (1.0, 0.33)]);
    assert_eq!(dc.ignore_classes.len(), 5);
    assert_eq!(dc.alert_dist, Some(100.0));
    assert_eq!(dc.explosive_power, Some(100.0));
    assert_eq!(dc.integral_power, Some(10.0));
    assert_eq!(dc.fall_distance, Some(20.0));
    assert_eq!(dc.fall_time, Some(20.0));
    assert_eq!(dc.buoyancy_to_damage_coeff.len(), 3);
}
