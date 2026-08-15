//! Modifier engine tests.

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::apply::low_hp_multiplier;
    use serde_json::json;

    #[test]
    fn parses_scalar_and_per_class_values() {
        let set = parse_modifiers(&json!({
            "speedCoef": 1.05,
            "GMShotDelay": {"Cruiser": 0.9, "Battleship": 0.95}
        }));
        assert_eq!(set.multiply("Cruiser", "speedCoef"), 1.05);
        assert_eq!(set.multiply("Cruiser", "GMShotDelay"), 0.9);
        assert_eq!(set.multiply("Destroyer", "GMShotDelay"), 1.0);
        assert_eq!(set.multiply("Cruiser", "missing"), 1.0);
    }

    #[test]
    fn merge_multiplies_and_sums() {
        let a = parse_modifiers(&json!({"GMShotDelay": 0.9, "additionalConsumables": 1}));
        let b = parse_modifiers(&json!({"GMShotDelay": 0.95, "additionalConsumables": 1}));
        let merged = a.merged(&b);
        assert!((merged.multiply("Cruiser", "GMShotDelay") - 0.855).abs() < 1e-9);
        assert!((merged.multiply("Cruiser", "additionalConsumables") - 2.0).abs() < 1e-9);
    }

    #[test]
    fn low_hp_reload_scales() {
        let mods = parse_modifiers(&json!({"lastChanceReloadCoefficient": 0.2}));
        assert!((low_hp_multiplier(&mods, "Cruiser", 1.0) - 1.0).abs() < 1e-9);
        assert!((low_hp_multiplier(&mods, "Cruiser", 0.0) - 0.2).abs() < 1e-9);
        assert!((low_hp_multiplier(&mods, "Cruiser", 0.5) - 0.6).abs() < 1e-9);
    }

    #[test]
    fn applies_to_build() {
        let json = serde_json::json!({
            "modules": {
                "_Hull": [{
                    "index": 0, "name": "H", "cost": {"costXP": 0, "costCR": 0},
                    "components": {"hull": ["H1"], "artillery": ["G1"], "airDefense": ["AA1"]}
                }],
                "_Torpedoes": [{
                    "index": 0, "name": "T", "cost": {"costXP": 0, "costCR": 0},
                    "components": {"torpedoes": ["T1"]}
                }],
                "_SecondaryWeapons": [{
                    "index": 0, "name": "S", "cost": {"costXP": 0, "costCR": 0},
                    "components": {"atba": ["G2"]}
                }]
            },
            "components": {
                "H1": {"health": 30000.0, "protection": 4.0,
                       "mobility": {"speed": 32.0, "turningRadius": 660.0, "rudderTime": 9.0},
                       "visibility": {"sea": 11.5, "plane": 6.0}},
                "G1": {"range": 14699.0, "sigma": 2.0, "guns": [{
                    "reload": 15.0, "rotation": 25.7, "each": 3,
                    "ammo": ["PAPA_AP"], "vertSector": 41.0, "count": 3}]},
                "G2": {"range": 5000.0, "sigma": 2.0, "guns": [{
                    "reload": 5.0, "rotation": 10.0, "each": 1,
                    "ammo": ["PAPA_AP"], "vertSector": 20.0, "count": 4}]},
                "T1": {"singleShot": false, "launchers": [{
                    "reload": 100.0, "rotation": 25.0, "each": 4,
                    "ammo": ["TORP"], "vertSector": 20.0, "count": 2}]},
                "AA1": {"near": [{"minRange": 0.1, "maxRange": 2.0, "hitChance": 0.9,
                                  "damage": 60.0, "rof": 0.29, "dps": 200.0, "guns": []}]}
            }
        });
        let ship = crate::wiki::gamedata::ShipInfo {
            modules: json.get("modules").cloned().unwrap_or_default(),
            components: json.get("components").cloned().unwrap_or_default(),
            ..Default::default()
        };
        let build = crate::wiki::ship_builder::build_ship_build(
            &ship,
            crate::wiki::ship_builder::ModuleSelection::default(),
        );
        let mods = parse_modifiers(&json!({
            "GMShotDelay": 0.9, "speedCoef": 1.05, "visibilityFactor": 0.9
        }));
        let adjusted = apply_modifiers(&build, "Cruiser", &mods, 1.0);
        assert!((adjusted.gun_reload_s - 13.5).abs() < 1e-9);
        assert!((adjusted.speed - 33.6).abs() < 1e-9);
        assert!((adjusted.concealment_sea - 10.35).abs() < 1e-9);
        assert!((adjusted.aa_dps - 200.0).abs() < 1e-9);

        // Low HP reduces reload for every armament type (Adrenaline Rush).
        let low_hp = parse_modifiers(&json!({"lastChanceReloadCoefficient": 0.2}));
        let adjusted = apply_modifiers(&build, "Cruiser", &low_hp, 0.5);
        assert!((adjusted.gun_reload_s - 15.0 * 0.6).abs() < 1e-9);
        assert!((adjusted.torp_reload_s - 100.0 * 0.6).abs() < 1e-9);
        assert!((adjusted.secondary_reload_s - 5.0 * 0.6).abs() < 1e-9);
    }
}
