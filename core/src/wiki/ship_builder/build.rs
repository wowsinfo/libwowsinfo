//! Build the computed stats for a module selection.

use serde_json::Value;

use super::helpers::{as_f64, as_i64, as_str, component, first_component, parse_module_options, sector};
use super::types::{ModuleSelection, ShipBuild};
use crate::wiki::components::{parse_air_defense, parse_guns, parse_hull, parse_special, parse_weapon, AirstrikeStats, AirSupportStats, DepthChargeLauncherStats, DepthChargePackStats, DepthChargeStats, EngineStats, FireControlStats, PingerStats, TorpedoStats};
use crate::wiki::gamedata::ShipInfo;

pub fn build_ship_build(ship: &ShipInfo, selection: ModuleSelection) -> ShipBuild {
    let hull_options = parse_module_options(ship, "_Hull");
    let artillery_options = parse_module_options(ship, "_Artillery");
    let primary_options = parse_module_options(ship, "_PrimaryWeapons");
    let secondary_options = parse_module_options(ship, "_SecondaryWeapons");
    let torpedo_options = parse_module_options(ship, "_Torpedoes");
    let fire_control_options = parse_module_options(ship, "_Suo");
    let fire_control_options_cv = parse_module_options(ship, "_FlightControl");
    let engine_options = parse_module_options(ship, "_Engine");

    let hull_option = hull_options.get(selection.hull).cloned();
    let hull_id = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "hull"))
        .cloned();
    let hull = hull_id
        .as_deref()
        .and_then(|id| component(ship, id))
        .map(parse_hull);

    let artillery_ids = if let Some(art) = artillery_options.get(selection.artillery) {
        art.components.get("artillery").cloned()
    } else if let Some(primary) = primary_options.first() {
        primary.components.get("artillery").cloned()
    } else {
        hull_option
            .as_ref()
            .and_then(|o| o.components.get("artillery").cloned())
    };
    let main_battery = artillery_ids
        .as_ref()
        .and_then(|ids| ids.first())
        .and_then(|id| component(ship, id))
        .map(parse_guns);

    let torpedo_ids = if let Some(t) = torpedo_options.get(selection.torpedoes) {
        t.components.get("torpedoes").cloned()
    } else if let Some(secondary) = secondary_options.first() {
        secondary.components.get("torpedoes").cloned()
    } else {
        hull_option
            .as_ref()
            .and_then(|o| o.components.get("torpedoes").cloned())
    };
    let torpedoes = torpedo_ids
        .as_ref()
        .and_then(|ids| ids.first())
        .and_then(|id| component(ship, id))
        .map(|json| TorpedoStats {
            single_shot: json
                .get("singleShot")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            launchers: json
                .get("launchers")
                .and_then(Value::as_array)
                .map(|arr| arr.iter().map(parse_weapon).collect())
                .unwrap_or_default(),
        });

    let fire_control = fire_control_options
        .get(selection.fire_control)
        .or_else(|| fire_control_options_cv.get(selection.fire_control))
        .and_then(|o| {
            o.components
                .get("fireControl")
                .or_else(|| o.components.get("flightControl"))
        })
        .and_then(|ids| ids.first())
        .and_then(|id| component(ship, id))
        .map(|json| FireControlStats {
            max_dist_coef: as_f64(json, "maxDistCoef"),
            sigma_count_coef: as_f64(json, "sigmaCountCoef"),
        });

    let engine = engine_options
        .get(selection.engine)
        .and_then(|o| first_component(&o.components, "engine"))
        .and_then(|id| component(ship, id))
        .map(|json| EngineStats {
            speed_coef: as_f64(json, "speedCoef"),
        })
        .or_else(|| {
            if engine_options.is_empty() {
                Some(EngineStats {
                    speed_coef: 1.0,
                })
            } else {
                None
            }
        });

    let air_defense = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "airDefense"))
        // Ships without a dedicated AA component carry their AA inside the
        // secondary (ATBA) component (legacy `far` band + `bubbles` block).
        .or_else(|| {
            hull_option
                .as_ref()
                .and_then(|o| first_component(&o.components, "atba"))
        })
        .and_then(|id| component(ship, id))
        .map(parse_air_defense);

    let depth_charges = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "depthCharges"))
        .and_then(|id| component(ship, id))
        .map(|json| {
            let dc = json.get("depthCharge").filter(|v| v.is_object());
            let packs = dc.and_then(|d| d.get("packs")).filter(|v| v.is_object());
            let empty = Value::Null;
            let packs_json = packs.unwrap_or(&empty);
            let launchers = dc
                .and_then(|d| d.get("launchers"))
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|l| {
                            let horiz = sector(l, "horizSector");
                            let vert = sector(l, "vertSector");
                            let rot = sector(l, "rotationSpeed");
                            Some(DepthChargeLauncherStats {
                                name: as_str(l, "name"),
                                num_bombs: as_i64(l, "numBombs"),
                                shoot_angle: as_f64(l, "shootAngle"),
                                shoot_dist: as_f64(l, "shootDist"),
                                start_fall_speed: as_f64(l, "startFallSpeed"),
                                horiz_sector_min: horiz.0,
                                horiz_sector_max: horiz.1,
                                vert_sector_min: vert.0,
                                vert_sector_max: vert.1,
                                fall_roll_acceleration: as_f64(l, "fallRollAcceleration"),
                                roll_speed: as_f64(l, "rollSpeed"),
                                rotation_speed_x: rot.0,
                                rotation_speed_y: rot.1,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            DepthChargeStats {
                reload: as_f64(json, "reload"),
                ammo: as_str(json, "ammo"),
                bombs: as_i64(json, "bombs"),
                groups: as_i64(json, "groups"),
                packs: DepthChargePackStats {
                    num_shots: as_i64(packs_json, "numShots"),
                    shots_in_pack: as_i64(packs_json, "shotsInPack"),
                    max_packs: as_i64(packs_json, "maxPacks"),
                    shot_delay: as_f64(packs_json, "shotDelay"),
                    guns_sequence_type: as_i64(packs_json, "gunsSequenceType"),
                    center_zone_width_part: as_f64(packs_json, "centerZoneWidthPart"),
                    use_shot_nodes_for_sequence: packs_json
                        .get("useShotNodesForSequence")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                },
                launchers,
            }
        });

    let air_support = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "airSupport"))
        .and_then(|id| component(ship, id))
        .map(|json| {
            let strike = json.get("airstrike").filter(|v| v.is_object());
            let empty = Value::Null;
            let strike_json = strike.unwrap_or(&empty);
            AirSupportStats {
                name: as_str(json, "name"),
                charges_num: as_i64(json, "chargesNum"),
                plane: as_str(json, "plane"),
                reload: as_f64(json, "reload"),
                range: as_f64(json, "range"),
                airstrike: AirstrikeStats {
                    auto_usage: strike_json
                        .get("autoUsage")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                    charges_num: as_i64(strike_json, "chargesNum"),
                    climb_angle: as_f64(strike_json, "climbAngle"),
                    fly_away_time: as_f64(strike_json, "flyAwayTime"),
                    max_dist: as_f64(strike_json, "maxDist"),
                    max_plane_flight_dist: as_f64(strike_json, "maxPlaneFlightDist"),
                    min_dist: as_f64(strike_json, "minDist"),
                    reload_time: as_f64(strike_json, "reloadTime"),
                    time_between_shots: as_f64(strike_json, "timeBetweenShots"),
                    time_from_heaven: as_f64(strike_json, "timeFromHeaven"),
                },
            }
        });

    let pinger = parse_module_options(ship, "_Sonar")
        .first()
        .and_then(|o| first_component(&o.components, "pinger"))
        .or_else(|| {
            hull_option
                .as_ref()
                .and_then(|o| first_component(&o.components, "pinger"))
        })
        .and_then(|id| component(ship, id))
        .map(|json| PingerStats {
            reload: as_f64(json, "reload"),
            range: as_f64(json, "range"),
            life_time1: as_f64(json, "lifeTime1"),
            life_time2: as_f64(json, "lifeTime2"),
            speed: as_f64(json, "speed"),
        });

    let special = hull_option
        .as_ref()
        .and_then(|o| first_component(&o.components, "specials"))
        .and_then(|id| component(ship, id))
        .and_then(parse_special);

    let secondaries = secondary_options
        .first()
        .and_then(|o| first_component(&o.components, "atba"))
        .or_else(|| {
            hull_option
                .as_ref()
                .and_then(|o| first_component(&o.components, "atba"))
        })
        .and_then(|id| component(ship, id))
        .map(parse_guns);

    ShipBuild {
        hull,
        main_battery,
        secondaries,
        torpedoes,
        air_defense,
        fire_control,
        engine,
        depth_charges,
        air_support,
        pinger,
        special,
    }
}
