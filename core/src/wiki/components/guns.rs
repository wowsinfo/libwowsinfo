//! Artillery / gun component shapes and parsers.

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::helpers::{as_f64, as_i64};

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

/// Burst / switchable-mode block (`switchable`, legacy `burst`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct BurstInfo {
    pub burst_reload_time: f64,
    pub full_reload_time: f64,
    pub shot_intensity: f64,
    pub shots_count: i64,
    /// Alternative shells loaded in the switchable mode (`secondaryAmmoList`).
    pub secondary_ammo: Vec<String>,
    /// Mode modifiers (e.g. `GMPenetrationCoeffHE` for Zorkiy's burst);
    /// always plain scalar values in the bundle.
    pub modifiers: Vec<(String, f64)>,
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
        burst: json
            .get("switchable")
            .or_else(|| json.get("burst"))
            .filter(|v| !v.is_null())
            .map(|burst| BurstInfo {
                burst_reload_time: as_f64(burst, "burstReloadTime"),
                full_reload_time: as_f64(burst, "fullReloadTime"),
                shot_intensity: as_f64(burst, "shotIntensity"),
                shots_count: as_i64(burst, "shotsCount"),
                secondary_ammo: burst
                    .get("secondaryAmmoList")
                    .and_then(Value::as_array)
                    .map(|arr| {
                        arr.iter()
                            .filter_map(Value::as_str)
                            .map(ToOwned::to_owned)
                            .collect()
                    })
                    .unwrap_or_default(),
                modifiers: burst
                    .get("modifiers")
                    .and_then(Value::as_object)
                    .map(|map| {
                        map.iter()
                            .filter_map(|(key, value)| {
                                value.as_f64().map(|number| (key.clone(), number))
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
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
