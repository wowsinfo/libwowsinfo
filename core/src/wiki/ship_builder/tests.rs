//! Ship builder tests.

#[cfg(test)]
mod tests {
    use super::super::helpers::parse_module_options;
    use super::super::build::build_ship_build;
    use super::super::types::ModuleSelection;
    use crate::wiki::gamedata::{GameData, ShipInfo};

    #[test]
    fn module_options_parse_and_build() {
        let json = serde_json::json!({
            "modules": {
                "_Hull": [
                    {"index": 0, "name": "IDS_A", "cost": {"costXP": 0, "costCR": 100},
                     "components": {"hull": ["A_Hull"], "artillery": ["A1"]}},
                    {"index": 1, "name": "IDS_B", "cost": {"costXP": 1000, "costCR": 200},
                     "components": {"hull": ["B_Hull"], "artillery": ["B1"]}}
                ]
            },
            "components": {
                "A_Hull": {"health": 100.0, "protection": 4.0, "mobility": {"speed": 30.0},
                           "visibility": {"sea": 10.0}},
                "B_Hull": {"health": 120.0, "protection": 4.0, "mobility": {"speed": 31.0},
                           "visibility": {"sea": 10.0}}
            }
        });
        let ship = ShipInfo {
            modules: json.get("modules").cloned().unwrap_or_default(),
            components: json.get("components").cloned().unwrap_or_default(),
            ..Default::default()
        };
        let options = parse_module_options(&ship, "_Hull");
        assert_eq!(options.len(), 2);
        assert_eq!(options[1].cost_xp, 1000);
        assert_eq!(options[1].components["hull"], vec!["B_Hull".to_string()]);
        let build = build_ship_build(
            &ship,
            ModuleSelection {
                hull: 1,
                ..ModuleSelection::default()
            },
        );
        let hull = build.hull.expect("hull");
        assert_eq!(hull.health, 120.0);
        assert_eq!(hull.mobility.speed, 31.0);
        let _ = GameData::default();
    }
}
