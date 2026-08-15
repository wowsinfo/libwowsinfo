//! Ship-wiki parser (`/wows/encyclopedia/ships/` profile).

use serde_json::{Map, Value};

use super::super::guard::guard;
use crate::models::{AntiAircraftProfile, AntiAircraftSlot, ArtilleryProfile, EngineProfile, GunSlot, HullProfile, MinMax, ShellInfo, ShipArmour, ShipConcealment, ShipMobility, ShipProfile, ShipWeaponry, ShipWiki, TorpedoProfile};


/// Parse `/wows/encyclopedia/ships/?ship_id=` into the wiki ship detail.
#[must_use]
pub fn parse_ship_wiki(json: &Value, ship_id: u64) -> Option<ShipWiki> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let ship = data.get(ship_id.to_string())?;
    let profile = ship.get("default_profile").unwrap_or(&empty);
    let str_field = |value: &Value, key: &str| {
        value
            .get(key)
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let int_field = |value: &Value, key: &str| value.get(key).and_then(Value::as_i64).unwrap_or(0);
    let float_field =
        |value: &Value, key: &str| value.get(key).and_then(Value::as_f64).unwrap_or(0.0);
    let min_max = |value: &Value| MinMax {
        min: int_field(value, "min"),
        max: int_field(value, "max"),
    };

    let armour = profile.get("armour").unwrap_or(&empty);
    let armour = ShipArmour {
        total: int_field(armour, "total"),
        health: int_field(armour, "health"),
        citadel: min_max(armour.get("citadel").unwrap_or(&empty)),
        extremities: min_max(armour.get("extremities").unwrap_or(&empty)),
        casemate: min_max(armour.get("casemate").unwrap_or(&empty)),
        deck: min_max(armour.get("deck").unwrap_or(&empty)),
    };

    let mobility = profile.get("mobility").unwrap_or(&empty);
    let mobility = ShipMobility {
        total: int_field(mobility, "total"),
        max_speed: float_field(mobility, "max_speed"),
        turning_radius: int_field(mobility, "turning_radius"),
        rudder_time: float_field(mobility, "rudder_time"),
    };

    let concealment = profile.get("concealment").unwrap_or(&empty);
    let concealment = ShipConcealment {
        total: int_field(concealment, "total"),
        detect_distance_by_ship: float_field(concealment, "detect_distance_by_ship"),
        detect_distance_by_plane: float_field(concealment, "detect_distance_by_plane"),
        detect_distance_by_submarine: float_field(concealment, "detect_distance_by_submarine"),
    };

    let weaponry = profile.get("weaponry").unwrap_or(&empty);
    let weaponry = ShipWeaponry {
        artillery: int_field(weaponry, "artillery"),
        torpedoes: int_field(weaponry, "torpedoes"),
        anti_aircraft: int_field(weaponry, "anti_aircraft"),
        aircraft: int_field(weaponry, "aircraft"),
    };

    let artillery = profile.get("artillery").map(|art| {
        let slots = art
            .get("slots")
            .and_then(Value::as_object)
            .map(|map| {
                map.values()
                    .map(|slot| GunSlot {
                        name: str_field(slot, "name"),
                        barrels: int_field(slot, "barrels"),
                        guns: int_field(slot, "guns"),
                    })
                    .collect()
            })
            .unwrap_or_default();
        let shells = art
            .get("shells")
            .and_then(Value::as_object)
            .map(|map| {
                map.values()
                    .map(|shell| ShellInfo {
                        name: str_field(shell, "name"),
                        r#type: str_field(shell, "type"),
                        damage: int_field(shell, "damage"),
                        bullet_mass: float_field(shell, "bullet_mass"),
                        bullet_speed: float_field(shell, "bullet_speed"),
                        burn_probability: shell.get("burn_probability").and_then(Value::as_f64),
                    })
                    .collect()
            })
            .unwrap_or_default();
        ArtilleryProfile {
            slots,
            shells,
            gun_rate: float_field(art, "gun_rate"),
            max_dispersion: int_field(art, "max_dispersion"),
            distance: float_field(art, "distance"),
        }
    });

    let torpedoes = profile.get("torpedoes").map(|torps| {
        let shells = torps
            .get("shells")
            .and_then(Value::as_object)
            .map(|map| {
                map.values()
                    .map(|shell| ShellInfo {
                        name: str_field(shell, "name"),
                        r#type: str_field(shell, "type"),
                        damage: int_field(shell, "damage"),
                        bullet_mass: float_field(shell, "bullet_mass"),
                        bullet_speed: float_field(shell, "bullet_speed"),
                        burn_probability: shell.get("burn_probability").and_then(Value::as_f64),
                    })
                    .collect()
            })
            .unwrap_or_default();
        TorpedoProfile {
            distance: float_field(torps, "distance"),
            shells,
        }
    });

    let anti_aircraft = profile.get("anti_aircraft").map(|aa| {
        let slots = aa
            .get("slots")
            .and_then(Value::as_object)
            .map(|map| {
                map.values()
                    .map(|slot| AntiAircraftSlot {
                        name: str_field(slot, "name"),
                        caliber: int_field(slot, "caliber"),
                        guns: int_field(slot, "guns"),
                        avg_damage: slot.get("avg_damage").and_then(Value::as_i64),
                    })
                    .collect()
            })
            .unwrap_or_default();
        AntiAircraftProfile {
            defense: int_field(aa, "defense"),
            slots,
        }
    });

    let hull = profile.get("hull").unwrap_or(&empty);
    let engine = profile.get("engine").unwrap_or(&empty);
    let image = ship
        .get("images")
        .and_then(|images| images.get("small"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let next_ships = ship
        .get("next_ships")
        .and_then(Value::as_array)
        .map(|list| list.iter().filter_map(Value::as_u64).collect())
        .unwrap_or_default();

    Some(ShipWiki {
        ship_id,
        name: str_field(ship, "name"),
        description: str_field(ship, "description"),
        nation: str_field(ship, "nation"),
        r#type: str_field(ship, "type"),
        tier: int_field(ship, "tier"),
        is_premium: ship.get("is_premium").and_then(Value::as_bool).unwrap_or(false),
        price_credit: int_field(ship, "price_credit"),
        price_gold: int_field(ship, "price_gold"),
        next_ships,
        image,
        profile: ShipProfile {
            armour,
            mobility,
            concealment,
            weaponry,
            artillery,
            torpedoes,
            anti_aircraft,
            hull: HullProfile {
                health: int_field(hull, "health"),
                artillery_barrels: int_field(hull, "artillery_barrels"),
                torpedoes_barrels: int_field(hull, "torpedoes_barrels"),
                anti_aircraft_barrels: int_field(hull, "anti_aircraft_barrels"),
            },
            engine: EngineProfile {
                max_speed: float_field(engine, "max_speed"),
            },
        },
    })
}

