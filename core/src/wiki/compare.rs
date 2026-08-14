//! Ship comparison table.
//!
//! Ports the Flutter two `compare_ship_page` and the wows-toolkit
//! `ComparisonShip` concept: pick up to a handful of ships and line up their
//! headline stats in a table. All values are computed from the same
//! `wowsinfo.json` build used by the ship detail screen.

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::gamedata::{GameData, ShipInfo};
use super::ship_builder::{build_ship_build, ModuleSelection, ShipBuild};
use super::LangMap;

/// One ship column of the comparison table.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct CompareShipHeader {
    pub ship_id: u64,
    pub index: String,
    pub name: String,
    pub tier: i64,
}

/// A similar ship (same tier and type).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SimilarShip {
    pub ship_id: u64,
    pub index: String,
    pub name: String,
    pub tier: i64,
    pub nation: String,
    pub ship_type: String,
}

/// Ships of the same tier and type as `ship` (up to 24, sorted by nation).
#[must_use]
pub fn similar_ships(data: &GameData, lang: &LangMap, ship: &ShipInfo) -> Vec<SimilarShip> {
    let mut out: Vec<SimilarShip> = data
        .ships
        .iter()
        .filter(|(id, candidate)| {
            **id != ship.id
                && candidate.tier == ship.tier
                && candidate.r#type == ship.r#type
                && candidate.paper_ship == ship.paper_ship
        })
        .map(|(id, candidate)| SimilarShip {
            ship_id: *id,
            index: candidate.index.clone(),
            name: lang.get(&candidate.name),
            tier: candidate.tier,
            nation: candidate.region.clone(),
            ship_type: candidate.r#type.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.nation.cmp(&b.nation).then(a.name.cmp(&b.name)));
    out.truncate(24);
    out
}

/// One stat row aligned with `LocalCompare.ships`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct CompareRow {
    pub label: String,
    pub values: Vec<String>,
}

/// The full comparison table.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct LocalCompare {
    pub ships: Vec<CompareShipHeader>,
    pub rows: Vec<CompareRow>,
}

fn fmt(value: f64, digits: usize) -> String {
    format!("{:.*}", digits, value)
}

fn fmt_int(value: f64) -> String {
    format!("{}", value.round() as i64)
}

fn main_battery_values(build: &ShipBuild) -> (String, String, String, String) {
    let Some(guns) = &build.main_battery else {
        return ("-".to_string(), "-".to_string(), "-".to_string(), "-".to_string());
    };
    let config = guns
        .guns
        .iter()
        .map(|g| format!("{} x {}", g.count, g.each))
        .collect::<Vec<_>>()
        .join(" ");
    let range = guns.range_m * build.fire_control.as_ref().map_or(1.0, |fc| fc.max_dist_coef.max(0.0));
    let reload = guns.guns.first().map_or(0.0, |g| g.reload);
    let sigma = guns.sigma;
    (
        config,
        format!("{} km", fmt(range / 1000.0, 1)),
        format!("{} s", fmt(reload, 1)),
        fmt(sigma, 2),
    )
}

fn torpedo_values(data: &GameData, build: &ShipBuild) -> (String, String) {
    let Some(torps) = &build.torpedoes else {
        return ("-".to_string(), "-".to_string());
    };
    let mut max_range: f64 = 0.0;
    let mut max_damage: f64 = 0.0;
    for launcher in &torps.launchers {
        for key in &launcher.ammo {
            let Some(shell) = data.projectiles.get(key) else {
                continue;
            };
            max_range = max_range.max(shell.range.unwrap_or(0.0) / (100.0 / 3.0));
            let damage = shell.alpha_damage.unwrap_or(0.0) / 3.0 + shell.damage;
            max_damage = max_damage.max(damage);
        }
    }
    let range = if max_range > 0.0 {
        format!("{} km", fmt(max_range, 1))
    } else {
        "-".to_string()
    };
    let damage = if max_damage > 0.0 {
        fmt_int(max_damage)
    } else {
        "-".to_string()
    };
    (range, damage)
}

fn aa_dps(build: &ShipBuild) -> String {
    let Some(aa) = &build.air_defense else {
        return "-".to_string();
    };
    let total = aa
        .near
        .iter()
        .chain(&aa.medium)
        .chain(&aa.far)
        .map(|aura| aura.dps)
        .sum::<f64>();
    fmt_int(total)
}

fn secondaries_values(build: &ShipBuild) -> String {
    build
        .secondaries
        .as_ref()
        .map(|guns| {
            guns.guns
                .iter()
                .map(|g| format!("{} x {}", g.count, g.each))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_else(|| "-".to_string())
}

/// Build one ship's comparison row values.
fn ship_values(data: &GameData, lang: &LangMap, ship: &ShipInfo) -> Vec<String> {
    let build = build_ship_build(ship, ModuleSelection::default());
    let (config, range, reload, sigma) = main_battery_values(&build);
    let (torp_range, torp_damage) = torpedo_values(data, &build);
    let hull = &build.hull;
    vec![
        ship.tier.to_string(),
        lang.get(&ship.type_id),
        ship.region.clone(),
        hull.as_ref().map_or_else(|| "-".to_string(), |h| fmt_int(h.health)),
        hull.as_ref()
            .map_or_else(|| "-".to_string(), |h| format!("{} kn", fmt(h.mobility.speed, 1))),
        hull.as_ref()
            .map_or_else(|| "-".to_string(), |h| format!("{} s", fmt(h.mobility.rudder_time, 1))),
        hull.as_ref()
            .map_or_else(|| "-".to_string(), |h| format!("{} km", fmt(h.visibility.sea, 1))),
        config,
        range,
        reload,
        sigma,
        torp_range,
        torp_damage,
        aa_dps(&build),
        secondaries_values(&build),
    ]
}

const ROW_LABELS: [&str; 15] = [
    "Tier",
    "Type",
    "Nation",
    "Health",
    "Speed",
    "Rudder",
    "Concealment",
    "Main battery",
    "Gun range",
    "Reload",
    "Sigma",
    "Torpedo range",
    "Torpedo damage",
    "AA DPS",
    "Secondaries",
];

/// Build the comparison table for `ship_ids` (best effort per ship).
#[must_use]
pub fn build_local_compare(
    data: &GameData,
    lang: &LangMap,
    ship_ids: &[u64],
) -> Option<LocalCompare> {
    if ship_ids.is_empty() {
        return None;
    }
    let ships: Vec<&ShipInfo> = ship_ids
        .iter()
        .filter_map(|id| data.ships.get(id))
        .collect();
    if ships.is_empty() {
        return None;
    }
    let headers = ships
        .iter()
        .map(|ship| CompareShipHeader {
            ship_id: ship.id,
            index: ship.index.clone(),
            name: lang.get(&ship.name),
            tier: ship.tier,
        })
        .collect();
    let columns: Vec<Vec<String>> = ships
        .iter()
        .map(|ship| ship_values(data, lang, ship))
        .collect();
    let rows = ROW_LABELS
        .iter()
        .enumerate()
        .map(|(index, label)| CompareRow {
            label: (*label).to_string(),
            values: columns.iter().map(|column| column[index].clone()).collect(),
        })
        .collect();
    Some(LocalCompare { ships: headers, rows })
}

#[cfg(test)]
mod tests {
    use super::*;
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
        super::super::gamedata::parse_game_data(&json)
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
}
