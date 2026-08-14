//! Typed ship component parsers (`ships.<id>.components`).
//!
//! Every component id in the game data maps to one of the shapes below;
//! following the wows-toolkit convention each shape is one typed struct.

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::modifiers::{parse_modifiers, ModifierSet};

/// A weapon group inside an artillery/torpedo component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct WeaponInfo {
    pub reload: f64,
    pub rotation: f64,
    pub each: i64,
    pub ammo: Vec<String>,
    pub vert_sector: f64,
    pub count: i64,
}

/// Burst-fire block (`guns[].burst`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct BurstInfo {
    pub burst_reload_time: f64,
    pub full_reload_time: f64,
    pub shot_intensity: f64,
    pub shots_count: i64,
}

/// An artillery component (`range`, `sigma`, `guns`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct GunStats {
    pub range_m: f64,
    pub sigma: f64,
    pub guns: Vec<WeaponInfo>,
    pub burst: Option<BurstInfo>,
    /// Per-turret details (`turrets[]`), used by the armor panel.
    pub turrets: Vec<TurretInfo>,
    /// The v15.7 `battery` block (caliber/barrels/rof/traverse/dispersion/arcs).
    pub battery: Option<BatteryStats>,
}

/// One artillery turret (`turrets[]`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TurretInfo {
    pub name: String,
    pub caliber: f64,
    pub barrels: i64,
    pub armor: f64,
}

impl GunStats {
    /// Horizontal turret traverse in deg/s. Prefers the v15.7
    /// `battery.traverse[0]`; falls back to `180 / legacy rotation`
    /// (the legacy `rotation` field stores the 180° turn time in seconds).
    pub(crate) fn rotation_deg_s(&self) -> f64 {
        if let Some(battery) = &self.battery
            && let Some(traverse) = battery.traverse.first().copied()
            && traverse > 0.0
        {
            return traverse;
        }
        if let Some(weapon) = self.guns.first()
            && weapon.rotation > 0.0
        {
            return 180.0 / weapon.rotation;
        }
        0.0
    }
}

/// The `battery.dispersion` block: the full dispersion model used by
/// ShipBuilder's `Dispersion.CalculateDispersion` (see DataExtensions.cs).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DispersionStats {
    pub normal_distribution: bool,
    pub taper_dist: f64,
    pub delim: f64,
    pub ellipse_range_min: f64,
    pub ellipse_range_max: f64,
    pub radius_on_zero: f64,
    pub radius_on_delim: f64,
    pub radius_on_max: f64,
    pub ideal_distance: f64,
    pub ideal_radius: f64,
    pub min_radius: f64,
    pub max_ellipse_ranging: f64,
    pub med_ellipse_ranging: f64,
    pub min_ellipse_ranging: f64,
    pub smoke_penalty: f64,
    pub on_move_tar_pos_coeff_zero: f64,
    pub on_move_tar_pos_coeff_delim: f64,
    pub on_move_tar_pos_coeff_max_dist: f64,
    pub on_move_tar_pos_delim: f64,
}

/// One turret firing arc (`battery.firingArcs.<name>`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct FiringArcInfo {
    pub name: String,
    pub horiz_min: f64,
    pub horiz_max: f64,
    pub vert_min: f64,
    pub vert_max: f64,
}

/// The `battery` block of an artillery/secondary component (v15.7 data).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct BatteryStats {
    /// Caliber in meters (`barrelDiameter`).
    pub caliber: f64,
    pub barrels: i64,
    /// Rounds per minute (`60 / shotDelay`).
    pub rof: f64,
    /// Rotation speeds, first entry = horizontal turret traverse in deg/s.
    pub traverse: Vec<f64>,
    /// Ammo-switch time coefficient (`reload * ammoSwitchCoeff`).
    pub ammo_switch_coeff: f64,
    pub dispersion: Option<DispersionStats>,
    pub firing_arcs: Vec<FiringArcInfo>,
}

/// One AA aura band (`near`/`medium`/`far`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AuraInfo {
    pub min_range: f64,
    pub max_range: f64,
    pub hit_chance: f64,
    pub damage: f64,
    pub rof: f64,
    pub dps: f64,
    pub guns: Vec<WeaponInfo>,
    /// v15.7 `antiAir.auras` extras.
    pub area_damage_period: f64,
    pub explosion_count: i64,
    pub shot_delay: f64,
    pub shot_travel_time: f64,
    pub bubble_damage: f64,
    pub inner_bubble_count: i64,
    pub outer_bubble_count: i64,
    pub bubble_radius: f64,
    pub bubble_duration: f64,
    pub enable_barrage: bool,
}

/// Aggregate flak-cloud block (`bubbles`) used by ATBA-based AA.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct BubbleInfo {
    pub inner: i64,
    pub outer: i64,
    pub rof: f64,
    pub min_range: f64,
    pub max_range: f64,
    pub hit_chance: f64,
    pub spawn_time: f64,
    pub damage: f64,
}

/// Torpedo component (`singleShot` + `launchers`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TorpedoStats {
    pub single_shot: bool,
    pub launchers: Vec<WeaponInfo>,
}

/// AA component with its three bands.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirDefenseStats {
    pub near: Vec<AuraInfo>,
    pub medium: Vec<AuraInfo>,
    pub far: Vec<AuraInfo>,
    pub bubbles: Option<BubbleInfo>,
}

/// Fire control component (`maxDistCoef`, `sigmaCountCoef`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct FireControlStats {
    pub max_dist_coef: f64,
    pub sigma_count_coef: f64,
}

/// Engine component (`speedCoef`, usually empty).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct EngineStats {
    pub speed_coef: f64,
}

/// Mobility block of a hull component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct MobilityStats {
    pub speed: f64,
    pub turning_radius: f64,
    pub rudder_time: f64,
}

/// The hull `maneuverability` block (newer game data).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ManeuverabilityStats {
    pub max_reverse_speed: f64,
    pub submarine: Option<SubmarineMobilityStats>,
    /// Raw engine/drag coefficients (`maneuverability.raw`). Acceleration
    /// times are not directly computable from the bundle (no tonnage or
    /// engine up-times), but the raw coefficients are shipped per hull.
    pub raw: Option<ManeuverabilityRaw>,
}

/// Raw engine/drag coefficients (`maneuverability.raw`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ManeuverabilityRaw {
    pub engine_power: f64,
    pub side_drag_coef: f64,
    pub backward_movement_drag_coef: f64,
    pub backward_power_coef: f64,
    pub cooling_off_speed: f64,
    pub speed_coef: f64,
    pub max_rudder_angle: f64,
    pub rudder_power: f64,
    pub underwater_max_rudder_angle: f64,
}

/// Submarine speed modes (`maneuverability.submarine`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SubmarineMobilityStats {
    pub surface_speed: f64,
    pub surface_reverse: f64,
    pub periscope_speed: f64,
    pub periscope_reverse: f64,
    pub max_depth_speed: f64,
    pub max_depth_reverse: f64,
    pub dive_speed: f64,
    pub diving_plane_shift_time: f64,
}

/// Visibility block of a hull component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct VisibilityStats {
    pub sea: f64,
    pub plane: f64,
    pub sea_in_smoke: f64,
    pub plane_in_smoke: f64,
    pub submarine: f64,
    pub sea_fire_coeff: f64,
    pub plane_fire_coeff: f64,
}

/// The hull `concealment` block (newer game data).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ConcealmentStats {
    pub sea_fire: f64,
    pub air_fire: f64,
    pub periscope_depth: f64,
    pub deep_water_depth: f64,
    pub smoke_factor: f64,
    /// `bySubmarineDepth` full table (state -> detectability in km).
    pub by_submarine_depth: Vec<(String, f64)>,
    pub smoke_factor_gk: f64,
    pub visibility_coef_gk_by_plane: f64,
    /// Depth (m) -> visibility coefficient (`visibilityCoeffUnderwaterDepths`).
    pub underwater_depth_coeff: Vec<(f64, f64)>,
    pub underwater_depth_coeff_plane: Vec<(f64, f64)>,
    pub deepwater_vision_coeff: Vec<(String, f64)>,
    pub deepwater_vision_to_plane_coeff: Vec<(String, f64)>,
}

/// Submarine battery block.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SubmarineBatteryStats {
    pub capacity: i64,
    pub regen: f64,
}

/// Hull component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct HullStats {
    pub health: f64,
    pub protection: f64,
    pub mobility: MobilityStats,
    pub maneuverability: Option<ManeuverabilityStats>,
    pub visibility: VisibilityStats,
    pub concealment: Option<ConcealmentStats>,
    pub armor: Option<ArmorStats>,
    pub submarine_battery: Option<SubmarineBatteryStats>,
    /// HP sections + fire/flood model (newer game data, optional).
    pub survivability: Option<SurvivabilityStats>,
}

/// One HP section (citadel, casemate, bow, ...) with its regen ratio.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct HpSectionStats {
    pub name: String,
    pub hp: f64,
    pub regen_ratio: f64,
    pub auto_repair_time: f64,
}

/// Fire or flood damage model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct FireFloodStats {
    pub spots: i64,
    pub chance: f64,
    pub duration: f64,
    pub dps: f64,
    pub total_damage: f64,
}

/// The hull `survivability` block (HP sections + fire/flood).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SurvivabilityStats {
    pub sections: Vec<HpSectionStats>,
    pub fire: Option<FireFloodStats>,
    pub flood: Option<FireFloodStats>,
}

/// One hull armor zone (`armor.zones`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ArmorZone {
    pub zone_id: String,
    pub thickness: f64,
}

/// Barbette armor for one turret (`armor.barbettes`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct BarbetteArmor {
    pub turret: String,
    pub max_thickness: f64,
}

/// The hull `armor` block (zones + barbette associations).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ArmorStats {
    pub zones: Vec<ArmorZone>,
    pub barbettes: Vec<BarbetteArmor>,
}

/// Depth-charge pack settings (`depthCharge.packs`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargePackStats {
    pub num_shots: i64,
    pub shots_in_pack: i64,
    pub max_packs: i64,
    pub shot_delay: f64,
    pub guns_sequence_type: i64,
    pub center_zone_width_part: f64,
    pub use_shot_nodes_for_sequence: bool,
}

/// One depth-charge thrower (`depthCharge.launchers[i]`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargeLauncherStats {
    pub name: String,
    pub num_bombs: i64,
    pub shoot_angle: f64,
    pub shoot_dist: f64,
    pub start_fall_speed: f64,
    pub horiz_sector_min: f64,
    pub horiz_sector_max: f64,
    pub vert_sector_min: f64,
    pub vert_sector_max: f64,
    pub fall_roll_acceleration: f64,
    pub roll_speed: f64,
    pub rotation_speed_x: f64,
    pub rotation_speed_y: f64,
}

/// Depth charge component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DepthChargeStats {
    pub reload: f64,
    pub ammo: String,
    pub bombs: i64,
    pub groups: i64,
    pub packs: DepthChargePackStats,
    pub launchers: Vec<DepthChargeLauncherStats>,
}

/// Airstrike module settings (v15.7 `airstrike` block).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirstrikeStats {
    pub auto_usage: bool,
    pub charges_num: i64,
    pub climb_angle: f64,
    pub fly_away_time: f64,
    pub max_dist: f64,
    pub max_plane_flight_dist: f64,
    pub min_dist: f64,
    pub reload_time: f64,
    pub time_between_shots: f64,
    pub time_from_heaven: f64,
}

/// Air support component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirSupportStats {
    pub name: String,
    pub charges_num: i64,
    pub plane: String,
    pub reload: f64,
    pub range: f64,
    pub airstrike: AirstrikeStats,
}

/// Submarine sonar (pinger) component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct PingerStats {
    pub reload: f64,
    pub range: f64,
    pub life_time1: f64,
    pub life_time2: f64,
    pub speed: f64,
}

/// Special ability (F / rage mode) component, v15.7 `specialAbility.rage`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SpecialStats {
    /// Raw mode key (e.g. `main_gun_accuracy`, `survivability`).
    pub mode: String,
    pub boost_duration: f64,
    pub boost_preparation: f64,
    /// Progress gained per triggering action (`progressPerAction`).
    pub progress_per_action: f64,
    pub progress_name: String,
    /// Required trigger count (`requiredCount`).
    pub required_count: i64,
    pub sub_ribbons: Vec<i64>,
    pub time_limit: f64,
    pub separate_tracking: bool,
    pub start_enabled: bool,
    pub decrement_delay: f64,
    pub decrement_period: f64,
    /// Progress lost per interval after the inactivity delay.
    pub decrement_count: f64,
    pub auto_usage: bool,
    pub modifiers: ModifierSet,
}

fn as_f64(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn as_i64(json: &Value, key: &str) -> i64 {
    json.get(key).and_then(Value::as_i64).unwrap_or(0)
}

/// Stable display order for HP sections (citadel first).
const SECTION_ORDER: [&str; 7] = [
    "citadel",
    "casemate",
    "bow",
    "stern",
    "superstructure",
    "auxiliaryRooms",
    "hull",
];

fn section_label(key: &str) -> String {
    match key {
        "citadel" => "Citadel".to_string(),
        "casemate" => "Casemate".to_string(),
        "bow" => "Bow".to_string(),
        "stern" => "Stern".to_string(),
        "superstructure" => "Superstructure".to_string(),
        "auxiliaryRooms" => "Auxiliary Rooms".to_string(),
        "hull" => "Hull".to_string(),
        _ => {
            let mut chars = key.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        }
    }
}

fn parse_fire_flood(json: &Value) -> Option<FireFloodStats> {
    if json.is_null() {
        return None;
    }
    Some(FireFloodStats {
        spots: as_i64(json, "spots"),
        chance: as_f64(json, "chance"),
        duration: as_f64(json, "duration"),
        dps: as_f64(json, "dps"),
        total_damage: as_f64(json, "totalDamage"),
    })
}

fn parse_submarine_mobility(json: &Value) -> Option<SubmarineMobilityStats> {
    if json.is_null() {
        return None;
    }
    Some(SubmarineMobilityStats {
        surface_speed: as_f64(json, "maxSpeedAtSurface"),
        surface_reverse: as_f64(json, "maxReverseSpeedAtSurface"),
        periscope_speed: as_f64(json, "maxSpeedAtPeriscope"),
        periscope_reverse: as_f64(json, "maxReverseSpeedAtPeriscope"),
        max_depth_speed: as_f64(json, "maxSpeedAtMaxDepth"),
        max_depth_reverse: as_f64(json, "maxReverseSpeedAtMaxDepth"),
        dive_speed: as_f64(json, "maxDiveSpeed"),
        diving_plane_shift_time: as_f64(json, "divingPlaneShiftTime"),
    })
}

fn depth_value(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

/// Parse a `{key: number}` map into sorted pairs (e.g. `bySubmarineDepth`).
fn str_f64_map_sorted(json: &Value, key: &str) -> Vec<(String, f64)> {
    json.get(key)
        .and_then(Value::as_object)
        .map(|map| {
            let mut pairs: Vec<(String, f64)> = map
                .iter()
                .filter_map(|(k, v)| v.as_f64().map(|value| (k.clone(), value)))
                .collect();
            pairs.sort_by(|a, b| a.0.cmp(&b.0));
            pairs
        })
        .unwrap_or_default()
}

/// Parse a numeric-keyed `{depth: coefficient}` table into sorted pairs.
fn num_key_f64_table(json: &Value, key: &str) -> Vec<(f64, f64)> {
    json.get(key)
        .and_then(Value::as_object)
        .map(|map| {
            let mut pairs: Vec<(f64, f64)> = map
                .iter()
                .filter_map(|(k, v)| {
                    let depth = k.parse::<f64>().ok()?;
                    let coeff = v.as_f64()?;
                    Some((depth, coeff))
                })
                .collect();
            pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
            pairs
        })
        .unwrap_or_default()
}

/// Parse one `guns`/`launchers` entry.
pub(crate) fn parse_weapon(json: &Value) -> WeaponInfo {
    WeaponInfo {
        reload: as_f64(json, "reload"),
        rotation: as_f64(json, "rotation"),
        each: as_i64(json, "each"),
        ammo: json
            .get("ammo")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default(),
        vert_sector: as_f64(json, "vertSector"),
        count: as_i64(json, "count"),
    }
}

/// Parse an artillery/secondary component.
pub(crate) fn parse_guns(json: &Value) -> GunStats {
    GunStats {
        range_m: as_f64(json, "range"),
        sigma: as_f64(json, "sigma"),
        guns: json
            .get("guns")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().map(parse_weapon).collect())
            .unwrap_or_default(),
        burst: json.get("burst").filter(|v| !v.is_null()).map(|burst| BurstInfo {
            burst_reload_time: as_f64(burst, "burstReloadTime"),
            full_reload_time: as_f64(burst, "fullReloadTime"),
            shot_intensity: as_f64(burst, "shotIntensity"),
            shots_count: as_i64(burst, "shotsCount"),
        }),
        turrets: json
            .get("turrets")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|turret| {
                        let name = turret.get("name").and_then(Value::as_str)?;
                        Some(TurretInfo {
                            name: name.to_string(),
                            caliber: as_f64(turret, "caliber"),
                            barrels: as_i64(turret, "barrels"),
                            armor: {
                                let a = turret.get("armor").unwrap_or(&Value::Null);
                                match a {
                                    Value::Object(map) => map
                                        .values()
                                        .filter_map(Value::as_f64)
                                        .fold(0.0, f64::max),
                                    _ => a.as_f64().unwrap_or(0.0),
                                }
                            },
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        battery: json.get("battery").and_then(parse_battery),
    }
}

/// Parse the v15.7 `battery` block of an artillery/secondary component.
fn parse_battery(json: &Value) -> Option<BatteryStats> {
    if !json.is_object() {
        return None;
    }
    Some(BatteryStats {
        caliber: as_f64(json, "caliber"),
        barrels: as_i64(json, "barrels"),
        rof: as_f64(json, "rof"),
        traverse: json
            .get("traverse")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default(),
        ammo_switch_coeff: as_f64(json, "ammoSwitchCoeff"),
        dispersion: json
            .get("dispersion")
            .filter(|v| v.is_object())
            .map(|d| DispersionStats {
                normal_distribution: d
                    .get("normalDistribution")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                taper_dist: as_f64(d, "taperDist"),
                delim: as_f64(d, "delim"),
                ellipse_range_min: as_f64(d, "ellipseRangeMin"),
                ellipse_range_max: as_f64(d, "ellipseRangeMax"),
                radius_on_zero: as_f64(d, "radiusOnZero"),
                radius_on_delim: as_f64(d, "radiusOnDelim"),
                radius_on_max: as_f64(d, "radiusOnMax"),
                ideal_distance: as_f64(d, "idealDistance"),
                ideal_radius: as_f64(d, "idealRadius"),
                min_radius: as_f64(d, "minRadius"),
                max_ellipse_ranging: as_f64(d, "maxEllipseRanging"),
                med_ellipse_ranging: as_f64(d, "medEllipseRanging"),
                min_ellipse_ranging: as_f64(d, "minEllipseRanging"),
                smoke_penalty: as_f64(d, "smokePenalty"),
                on_move_tar_pos_coeff_zero: as_f64(d, "onMoveTarPosCoeffZero"),
                on_move_tar_pos_coeff_delim: as_f64(d, "onMoveTarPosCoeffDelim"),
                on_move_tar_pos_coeff_max_dist: as_f64(d, "onMoveTarPosCoeffMaxDist"),
                on_move_tar_pos_delim: as_f64(d, "onMoveTarPosDelim"),
            }),
        firing_arcs: json
            .get("firingArcs")
            .and_then(Value::as_object)
            .map(|arcs| {
                arcs.iter()
                    .filter_map(|(name, arc)| {
                        if !arc.is_object() {
                            return None;
                        }
                        let sector = |key: &str| -> (f64, f64) {
                            arc.get(key)
                                .and_then(Value::as_array)
                                .filter(|arr| arr.len() >= 2)
                                .map(|arr| {
                                    (
                                        arr[0].as_f64().unwrap_or(0.0),
                                        arr[1].as_f64().unwrap_or(0.0),
                                    )
                                })
                                .unwrap_or((0.0, 0.0))
                        };
                        let (horiz_min, horiz_max) = sector("horizSector");
                        let (vert_min, vert_max) = sector("vertSector");
                        Some(FiringArcInfo {
                            name: name.clone(),
                            horiz_min,
                            horiz_max,
                            vert_min,
                            vert_max,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
    })
}

/// Parse one AA aura band entry.
pub(crate) fn parse_aura(json: &Value) -> AuraInfo {
    AuraInfo {
        min_range: as_f64(json, "minRange"),
        max_range: as_f64(json, "maxRange"),
        hit_chance: as_f64(json, "hitChance"),
        damage: as_f64(json, "damage").max(as_f64(json, "areaDamage")),
        rof: as_f64(json, "rof"),
        dps: as_f64(json, "dps"),
        guns: json
            .get("guns")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().map(parse_weapon).collect())
            .unwrap_or_default(),
        area_damage_period: as_f64(json, "areaDamagePeriod"),
        explosion_count: as_i64(json, "explosionCount"),
        shot_delay: as_f64(json, "shotDelay"),
        shot_travel_time: as_f64(json, "shotTravelTime"),
        bubble_damage: as_f64(json, "bubbleDamage"),
        inner_bubble_count: as_i64(json, "innerBubbleCount"),
        outer_bubble_count: as_i64(json, "outerBubbleCount"),
        bubble_radius: as_f64(json, "bubbleRadius"),
        bubble_duration: as_f64(json, "bubbleDuration"),
        enable_barrage: json
            .get("enableBarrage")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    }
}

/// Parse an AA component, preferring the v15.7 `antiAir.auras` blocks and
/// falling back to the legacy top-level bands (ATBA-based AA). Legacy gun
/// mounts are attached to the matching new aura when both exist.
pub(crate) fn parse_air_defense(json: &Value) -> AirDefenseStats {
    let auras = json
        .get("antiAir")
        .and_then(|anti| anti.get("auras"))
        .filter(|v| v.is_object());
    let band = |key: &str| -> Vec<AuraInfo> {
        let mut parsed = auras
            .and_then(|a| a.get(key))
            .and_then(Value::as_array)
            .map(|arr| arr.iter().map(parse_aura).collect::<Vec<_>>())
            .unwrap_or_default();
        if !parsed.is_empty() {
            // The legacy bands use different names per ship (a mount can sit
            // in legacy `far` while the matching stats live in new `medium`),
            // so attach legacy guns by range instead of by band key.
            let mut legacy_all: Vec<AuraInfo> = ["near", "medium", "far"]
                .iter()
                .flat_map(|band_key| {
                    json.get(*band_key)
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten()
                        .map(parse_aura)
                })
                .collect();
            for aura in parsed.iter_mut() {
                let Some(pos) = legacy_all
                    .iter()
                    .position(|old| (old.max_range - aura.max_range).abs() < 0.01)
                else {
                    continue;
                };
                let old = legacy_all.remove(pos);
                if aura.guns.is_empty() {
                    aura.guns = old.guns;
                }
                if aura.dps == 0.0 {
                    aura.dps = old.dps;
                }
                if aura.hit_chance == 0.0 {
                    aura.hit_chance = old.hit_chance;
                }
                if aura.damage == 0.0 {
                    aura.damage = old.damage;
                }
            }
        } else {
            parsed = json
                .get(key)
                .and_then(Value::as_array)
                .map(|arr| arr.iter().map(parse_aura).collect())
                .unwrap_or_default();
        }
        parsed
    };
    AirDefenseStats {
        near: band("near"),
        medium: band("medium"),
        far: band("far"),
        bubbles: json.get("bubbles").filter(|v| v.is_object()).map(|b| BubbleInfo {
            inner: as_i64(b, "inner"),
            outer: as_i64(b, "outer"),
            rof: as_f64(b, "rof"),
            min_range: as_f64(b, "minRange"),
            max_range: as_f64(b, "maxRange"),
            hit_chance: as_f64(b, "hitChance"),
            spawn_time: as_f64(b, "spawnTime"),
            damage: as_f64(b, "damage"),
        }),
    }
}

fn bool_field(json: &Value, key: &str) -> bool {
    json.get(key).and_then(Value::as_bool).unwrap_or(false)
}

/// Parse a special ability component (`specials`): the v15.7 structured
/// `specialAbility.rage` block, falling back to the raw `rageMode` block.
pub(crate) fn parse_special(json: &Value) -> Option<SpecialStats> {
    let rage = json
        .get("specialAbility")
        .and_then(|sa| sa.get("rage"))
        .filter(|v| v.is_object())
        .or_else(|| json.get("rageMode").filter(|v| v.is_object()))?;
    Some(SpecialStats {
        mode: rage
            .get("name")
            .and_then(Value::as_str)
            .or_else(|| rage.get("rageModeName").and_then(Value::as_str))
            .unwrap_or("")
            .to_string(),
        boost_duration: as_f64(rage, "duration").max(as_f64(rage, "boostDuration")),
        boost_preparation: as_f64(rage, "preparation").max(as_f64(rage, "boostPreparation")),
        progress_per_action: as_f64(rage, "progressPerAction"),
        progress_name: rage
            .get("progressName")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        required_count: as_f64(rage, "requiredCount").round() as i64,
        sub_ribbons: rage
            .get("subRibbons")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(Value::as_i64).collect())
            .unwrap_or_default(),
        time_limit: as_f64(rage, "timeLimit"),
        separate_tracking: bool_field(rage, "separateTracking"),
        start_enabled: bool_field(rage, "startEnabled"),
        decrement_delay: as_f64(rage, "inactivityDelay").max(as_f64(rage, "decrementDelay")),
        decrement_period: as_f64(rage, "progressLossInterval").max(as_f64(rage, "decrementPeriod")),
        decrement_count: as_f64(rage, "progressLossPerInterval").max(as_f64(rage, "decrementCount")),
        auto_usage: bool_field(rage, "autoUsage") || bool_field(rage, "isAutoUsage"),
        modifiers: rage
            .get("modifiers")
            .map(parse_modifiers)
            .unwrap_or_default(),
    })
}

/// Parse a hull component.
pub(crate) fn parse_hull(json: &Value) -> HullStats {
    HullStats {
        health: as_f64(json, "health"),
        protection: as_f64(json, "protection"),
        mobility: {
            let m = json.get("mobility").unwrap_or(&Value::Null);
            MobilityStats {
                speed: as_f64(m, "speed"),
                turning_radius: as_f64(m, "turningRadius"),
                rudder_time: as_f64(m, "rudderTime"),
            }
        },
        maneuverability: json.get("maneuverability").filter(|v| !v.is_null()).map(|m| {
            let raw = m
                .get("raw")
                .filter(|v| v.is_object())
                .map(|raw| ManeuverabilityRaw {
                    engine_power: as_f64(raw, "enginePower"),
                    side_drag_coef: as_f64(raw, "sideDragCoef"),
                    backward_movement_drag_coef: as_f64(raw, "backwardMovementDragCoef"),
                    backward_power_coef: as_f64(raw, "backwardPowerCoef"),
                    cooling_off_speed: as_f64(raw, "coolingOffSpeed"),
                    speed_coef: as_f64(raw, "speedCoef"),
                    max_rudder_angle: as_f64(raw, "maxRudderAngle"),
                    rudder_power: as_f64(raw, "rudderPower"),
                    underwater_max_rudder_angle: as_f64(raw, "underwaterMaxRudderAngle"),
                });
            ManeuverabilityStats {
                max_reverse_speed: as_f64(m, "maxReverseSpeed"),
                submarine: parse_submarine_mobility(
                    m.get("submarine").unwrap_or(&Value::Null),
                ),
                raw,
            }
        }),
        visibility: {
            let v = json.get("visibility").unwrap_or(&Value::Null);
            VisibilityStats {
                sea: as_f64(v, "sea"),
                plane: as_f64(v, "plane"),
                sea_in_smoke: as_f64(v, "seaInSmoke"),
                plane_in_smoke: as_f64(v, "planeInSmoke"),
                submarine: as_f64(v, "submarine"),
                sea_fire_coeff: as_f64(v, "seaFireCoeff"),
                plane_fire_coeff: as_f64(v, "planeFireCoeff"),
            }
        },
        concealment: json.get("concealment").filter(|v| !v.is_null()).map(|c| {
            ConcealmentStats {
                sea_fire: as_f64(c, "seaFire"),
                air_fire: as_f64(c, "airFire"),
                periscope_depth: depth_value(
                    c.get("bySubmarineDepth").unwrap_or(&Value::Null),
                    "PERISCOPE",
                ),
                deep_water_depth: depth_value(
                    c.get("bySubmarineDepth").unwrap_or(&Value::Null),
                    "DEEP_WATER",
                ),
                smoke_factor: as_f64(c, "visibilityFactorInSmoke"),
                by_submarine_depth: str_f64_map_sorted(c, "bySubmarineDepth"),
                smoke_factor_gk: as_f64(c, "visibilityCoefGKInSmoke"),
                visibility_coef_gk_by_plane: as_f64(c, "visibilityCoefGKByPlane"),
                underwater_depth_coeff: num_key_f64_table(c, "visibilityCoeffUnderwaterDepths"),
                underwater_depth_coeff_plane: num_key_f64_table(
                    c,
                    "visibilityCoeffUnderwaterDepthsByPlane",
                ),
                deepwater_vision_coeff: str_f64_map_sorted(c, "deepwaterVisionCoeff"),
                deepwater_vision_to_plane_coeff: str_f64_map_sorted(
                    c,
                    "deepwaterVisionToPlaneCoeff",
                ),
            }
        }),
        armor: json.get("armor").filter(|v| !v.is_null()).map(|a| {
            let zone_map = a.get("zones").and_then(Value::as_object);
            let mut zones: Vec<ArmorZone> = a
                .get("zones")
                .and_then(Value::as_object)
                .map(|map| {
                    map.iter()
                        .map(|(id, value)| ArmorZone {
                            zone_id: id.clone(),
                            thickness: value.as_f64().unwrap_or(0.0),
                        })
                        .collect()
                })
                .unwrap_or_default();
            zones.sort_by(|x, y| y.thickness.total_cmp(&x.thickness).then(x.zone_id.cmp(&y.zone_id)));
            let mut barbettes: Vec<BarbetteArmor> = a
                .get("barbettes")
                .and_then(Value::as_object)
                .map(|map| {
                    map.iter()
                        .map(|(turret, ids)| {
                            let max_thickness = ids
                                .as_array()
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|id| {
                                            let zone_id = id
                                                .as_u64()
                                                .map(|n| n.to_string())
                                                .or_else(|| id.as_str().map(str::to_string))?;
                                            zone_map
                                                .and_then(|m| m.get(&zone_id))
                                                .and_then(Value::as_f64)
                                        })
                                        .fold(0.0, f64::max)
                                })
                                .unwrap_or(0.0);
                            BarbetteArmor {
                                turret: turret.clone(),
                                max_thickness,
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();
            barbettes.sort_by(|x, y| x.turret.cmp(&y.turret));
            ArmorStats { zones, barbettes }
        }),
        submarine_battery: json
            .get("submarineBattery")
            .filter(|v| !v.is_null())
            .map(|b| SubmarineBatteryStats {
                capacity: as_i64(b, "capacity"),
                regen: as_f64(b, "regen"),
            }),
        survivability: json.get("survivability").filter(|v| !v.is_null()).map(|s| {
            let mut sections: Vec<HpSectionStats> = s
                .get("sections")
                .and_then(Value::as_object)
                .map(|map| {
                    map.iter()
                        .filter_map(|(key, value)| {
                            let hp = value.get("hp").and_then(Value::as_f64)?;
                            Some(HpSectionStats {
                                name: section_label(key),
                                hp,
                                regen_ratio: as_f64(value, "regenRatio"),
                                auto_repair_time: as_f64(value, "autoRepairTime"),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            sections.sort_by_key(|section| {
                SECTION_ORDER
                    .iter()
                    .position(|name| section.name.eq_ignore_ascii_case(name))
                    .unwrap_or(usize::MAX)
            });
            SurvivabilityStats {
                sections,
                fire: parse_fire_flood(s.get("fire").unwrap_or(&Value::Null)),
                flood: parse_fire_flood(s.get("flood").unwrap_or(&Value::Null)),
            }
        }),
    }
}

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
