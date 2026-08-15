//! Shell card view model.

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::LangMap;
use crate::wiki::projectile::ProjectileInfo;

/// One shell card shown on the wiki (resolved against the lang map).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ShellView {
    pub key: String,
    pub name: String,
    pub ammo_type: String,
    pub damage: i64,
    pub burn_chance: Option<f64>,
    pub pen_he: Option<f64>,
    pub pen_sap: Option<f64>,
    pub speed: f64,
    pub weight: f64,
    pub calibre_mm: f64,
    pub fuse_time: Option<f64>,
    pub ricochet_angle: Option<f64>,
    pub ricochet_always: Option<f64>,
    pub overmatch: Option<i64>,
    pub range: Option<f64>,
    pub flood_chance: Option<f64>,
    pub visibility: Option<f64>,
    pub alpha_damage: Option<f64>,
    // Ballistics (ShipBuilder shell card).
    pub air_drag: Option<f64>,
    pub arming_threshold: Option<f64>,
    pub cap_normalize_max_angle: Option<f64>,
    pub explosion_radius: Option<f64>,
    pub krupp: Option<f64>,
    pub shell_cap: Option<bool>,
    pub splash_radius: Option<f64>,
    pub underwater_dist_factor: Option<f64>,
    pub underwater_penetration_factor: Option<f64>,
    pub water_drag: Option<f64>,
    /// Dispersion distance parameters (`distParams`, 4 values) + tile size.
    pub dist_params: Vec<f64>,
    pub dist_tile: Option<f64>,
}

impl ShellView {
    pub(crate) fn from_projectile(lang: &LangMap, projectile: &ProjectileInfo) -> Self {
        Self {
            key: projectile.key.clone(),
            name: lang.get(&projectile.name),
            ammo_type: projectile.ammo_type.clone(),
            damage: projectile.damage.round() as i64,
            burn_chance: projectile.burn_chance,
            pen_he: projectile.pen_he,
            pen_sap: projectile.pen_sap,
            speed: projectile.speed,
            weight: projectile.weight,
            calibre_mm: projectile.calibre_mm(),
            fuse_time: projectile.fuse_time,
            ricochet_angle: projectile.ricochet_angle,
            ricochet_always: projectile.ricochet_always,
            overmatch: projectile.overmatch,
            range: projectile.range,
            flood_chance: projectile.flood_chance,
            visibility: projectile.visibility,
            alpha_damage: projectile.alpha_damage,
            air_drag: projectile.air_drag,
            arming_threshold: projectile.arming_threshold,
            cap_normalize_max_angle: projectile.cap_normalize_max_angle,
            explosion_radius: projectile.explosion_radius,
            krupp: projectile.krupp,
            shell_cap: projectile.shell_cap,
            splash_radius: projectile.splash_radius,
            underwater_dist_factor: projectile.underwater_dist_factor,
            underwater_penetration_factor: projectile.underwater_penetration_factor,
            water_drag: projectile.water_drag,
            dist_params: projectile.dist_params.clone(),
            dist_tile: projectile.dist_tile,
        }
    }
}
