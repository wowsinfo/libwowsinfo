//! Torpedo view models.

use std::collections::HashSet;

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::battery::{configuration, shells_from_weapons};
use super::shell::ShellView;
use crate::wiki::components::TorpedoStats;
use crate::wiki::gamedata::GameData;
use crate::wiki::projectile::ProjectileInfo;
use crate::wiki::LangMap;

/// Acoustic homing values of a torpedo.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AcousticDetectionView {
    pub countdown: f64,
    pub max_depth_level: f64,
    pub max_pitch: f64,
    pub max_yaw: f64,
    pub path_length: f64,
    pub search_angle: f64,
    pub search_radius: f64,
    pub speed_decr_coef: f64,
    pub vertical_acceleration: f64,
    pub yaw_change_speed: f64,
}

/// One resolved torpedo type in a launcher bank.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TorpedoDetailView {
    pub key: String,
    pub name: String,
    pub deep_water: bool,
    /// Display damage: `(alpha / 3 + damage).round()`.
    pub damage: i64,
    pub alpha_damage: i64,
    /// Full-salvo damage (`torpedo_count * damage`).
    pub salvo_damage: i64,
    /// Range in km (`range / (100/3)`).
    pub range_km: f64,
    pub speed_kt: f64,
    /// Detectability in km.
    pub detectability_km: f64,
    /// Reaction time in seconds (`detectability / (speed * 2.6854) * 1000`).
    pub reaction_time_s: f64,
    pub arming_distance_m: Option<f64>,
    pub depth_m: Option<f64>,
    pub flood_chance: Option<f64>,
    pub splash_armor_coeff: Option<f64>,
    pub splash_cube_size: Option<f64>,
    /// Ping damage coefficient (`damageCoeffMaxPing`).
    pub ping_damage_coeff: Option<f64>,
    pub acoustic_detection: Option<AcousticDetectionView>,
    pub can_hit_classes: Vec<String>,
}

/// Resolved torpedo view.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TorpedoView {
    pub name: String,
    pub single_shot: bool,
    pub reload_s: f64,
    pub rotation_deg_s: f64,
    /// Time for a 180° launcher turn in seconds.
    pub turn_time_s: f64,
    pub configuration: String,
    pub shells: Vec<ShellView>,
    /// Total torpedo tubes across all launchers.
    pub torpedo_count: i64,
    /// One detail card per torpedo type carried by the launchers.
    pub torpedoes: Vec<TorpedoDetailView>,
}

pub(super) fn torpedo_view(
    lang: &LangMap,
    data: &GameData,
    module_name: &str,
    torps: &TorpedoStats,
) -> TorpedoView {
    let first = torps.launchers.first().cloned().unwrap_or_default();
    // Legacy `rotation` is the 180° launcher turn time in seconds.
    let rotation_deg_s = if first.rotation > 0.0 {
        180.0 / first.rotation
    } else {
        0.0
    };
    let torpedo_count = torps.launchers.iter().map(|launcher| launcher.count * launcher.each).sum::<i64>();
    let mut seen = HashSet::new();
    let torpedoes = torps
        .launchers
        .iter()
        .flat_map(|launcher| launcher.ammo.iter())
        .filter(|key| seen.insert((*key).clone()))
        .filter_map(|key| {
            let projectile = data.projectiles.get(key)?;
            (projectile.r#type == "Torpedo").then(|| torpedo_detail(lang, projectile, torpedo_count))
        })
        .collect();
    TorpedoView {
        name: lang.get(module_name),
        single_shot: torps.single_shot,
        reload_s: first.reload,
        rotation_deg_s,
        turn_time_s: first.rotation,
        configuration: configuration(&torps.launchers),
        shells: shells_from_weapons(lang, data, &torps.launchers),
        torpedo_count,
        torpedoes,
    }
}

fn torpedo_detail(
    lang: &LangMap,
    projectile: &ProjectileInfo,
    torpedo_count: i64,
) -> TorpedoDetailView {
    let alpha = projectile.alpha_damage.unwrap_or(0.0);
    let damage = (alpha / 3.0 + projectile.damage).round() as i64;
    let speed = projectile.speed.max(0.0);
    let visibility = projectile.visibility.unwrap_or(0.0);
    let reaction_time = if speed > 0.0 {
        visibility / speed / 2.6854 * 1000.0
    } else {
        0.0
    };
    TorpedoDetailView {
        key: projectile.key.clone(),
        name: lang.get(&projectile.name),
        deep_water: projectile.deep_water,
        damage,
        alpha_damage: alpha.round() as i64,
        salvo_damage: damage * torpedo_count,
        range_km: projectile.range.map_or(0.0, |range| range / (100.0 / 3.0)),
        speed_kt: speed,
        detectability_km: visibility,
        reaction_time_s: reaction_time,
        arming_distance_m: projectile.arming_distance,
        depth_m: projectile.depth,
        flood_chance: projectile.flood_chance,
        splash_armor_coeff: projectile.splash_armor_coeff,
        splash_cube_size: projectile.splash_cube_size,
        ping_damage_coeff: projectile.damage_coeff_max_ping,
        acoustic_detection: projectile.acoustic_detection.as_ref().map(|ad| AcousticDetectionView {
            countdown: ad.countdown,
            max_depth_level: ad.max_depth_level,
            max_pitch: ad.max_pitch,
            max_yaw: ad.max_yaw,
            path_length: ad.path_length,
            search_angle: ad.search_angle,
            search_radius: ad.search_radius,
            speed_decr_coef: ad.speed_decr_coef,
            vertical_acceleration: ad.vertical_acceleration,
            yaw_change_speed: ad.yaw_change_speed,
        }),
        can_hit_classes: projectile.can_hit_classes.clone(),
    }
}
