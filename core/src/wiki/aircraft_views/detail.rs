//! Resolved aircraft detail.

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::helpers::aircraft_type_keys;
use crate::wiki::aircraft::AircraftInfo;
use crate::wiki::gamedata::GameData;
use crate::wiki::local_ship::ShellView;
use crate::wiki::modifiers::ModifierSet;
use crate::wiki::LangMap;

/// Resolved carrier aircraft detail.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AircraftDetail {
    pub key: String,
    pub name: String,
    pub r#type: String,
    pub health: f64,
    pub total_planes: i64,
    pub speed: f64,
    pub visibility: f64,
    pub attack_count: Option<i64>,
    pub attacker: Option<i64>,
    pub max_aircraft: Option<i64>,
    pub restore_time: Option<f64>,
    pub bomb: Option<ShellView>,
    /// Weapon reference (`bombName`).
    pub bomb_name: String,
    pub attack_cooldown: Option<f64>,
    pub attack_interval: Option<f64>,
    pub aiming_time: Option<f64>,
    /// `min .. max` aiming speed limits.
    pub aiming_speed_limits: String,
    pub aiming_accuracy_increase_rate: Option<f64>,
    pub aiming_accuracy_decrease_rate: Option<f64>,
    pub aiming_turn_speed_limit: Option<f64>,
    pub preparation_time: Option<f64>,
    /// `min .. max` preparation speed limits.
    pub preparation_speed_limits: String,
    pub preparation_turn_speed_limit: Option<f64>,
    pub climb_speed_coef: Option<f64>,
    pub dive_speed_coef: Option<f64>,
    pub angle_of_climb: Option<f64>,
    pub angle_of_dive: Option<f64>,
    pub post_attack_invulnerability_duration: Option<f64>,
    pub jato_duration: Option<f64>,
    pub jato_speed_multiplier: Option<f64>,
    pub max_forsage_amount: Option<f64>,
    pub forsage_regeneration: Option<f64>,
    pub speed_min: Option<f64>,
    pub speed_max: Option<f64>,
    pub attacker_damage_taken_multiplier: Option<f64>,
    pub damage_taken_multiplier: Option<f64>,
    pub bomb_falling_time: Option<f64>,
    pub bombing_drop_point_time: Option<f64>,
    pub empty_return_speed_multiplier: Option<f64>,
    pub max_rotate_speed: Option<f64>,
    pub plane_speedup_coef: Option<f64>,
    pub can_stop: bool,
    pub max_number_on_deck: Option<i64>,
    pub restoration_time: Option<f64>,
    pub restore_amount: Option<i64>,
    pub start_on_deck: Option<i64>,
    pub plane_consumables: Vec<PlaneConsumableView>,
    /// Squadron stats after the selected skill modifiers are applied.
    pub adjusted_health: f64,
    pub adjusted_speed: f64,
}

/// One resolved plane consumable slot.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct PlaneConsumableView {
    pub slot: i64,
    pub abilities: Vec<String>,
    pub special: bool,
}

fn limits(min: Option<f64>, max: Option<f64>) -> String {
    match (min, max) {
        (Some(a), Some(b)) => format!("{a} .. {b}"),
        (Some(a), None) => format!("{a}"),
        _ => String::new(),
    }
}

impl AircraftDetail {
    pub(super) fn from_aircraft(
        lang: &LangMap,
        data: &GameData,
        aircraft: &AircraftInfo,
        mods: &ModifierSet,
        ship_class: &str,
    ) -> Self {
        let bomb = aircraft
            .bomb_name
            .as_deref()
            .and_then(|key| data.projectiles.get(key))
            .map(|projectile| ShellView::from_projectile(lang, projectile));
        let (health_key, speed_key) = aircraft_type_keys(&aircraft.r#type);
        let ability_name = |id: &str| {
            data.abilities
                .values()
                .find(|ability| ability.icon == id)
                .map(|ability| lang.get(&ability.name))
                .unwrap_or_else(|| id.to_string())
        };
        let plane_consumables = aircraft
            .plane_consumables
            .iter()
            .map(|slot| PlaneConsumableView {
                slot: slot.slot,
                special: slot.special,
                abilities: slot
                    .abilities
                    .iter()
                    .map(|id| ability_name(id))
                    .collect(),
            })
            .collect();
        Self {
            key: aircraft.key.clone(),
            name: lang.get(&aircraft.name),
            r#type: aircraft.r#type.clone(),
            health: aircraft.health,
            total_planes: aircraft.total_planes,
            speed: aircraft.speed,
            visibility: aircraft.visibility,
            attack_count: aircraft.attack_count,
            attacker: aircraft.attacker,
            max_aircraft: aircraft.max_aircraft,
            restore_time: aircraft.restore_time,
            bomb,
            bomb_name: aircraft.bomb_name.clone().unwrap_or_default(),
            attack_cooldown: aircraft.attack_cooldown,
            attack_interval: aircraft.attack_interval,
            aiming_time: aircraft.aiming_time,
            aiming_speed_limits: limits(
                aircraft.aiming_speed_limit_min,
                aircraft.aiming_speed_limit_max,
            ),
            aiming_accuracy_increase_rate: aircraft.aiming_accuracy_increase_rate,
            aiming_accuracy_decrease_rate: aircraft.aiming_accuracy_decrease_rate,
            aiming_turn_speed_limit: aircraft.aiming_turn_speed_limit,
            preparation_time: aircraft.preparation_time,
            preparation_speed_limits: limits(
                aircraft.preparation_speed_limit_min,
                aircraft.preparation_speed_limit_max,
            ),
            preparation_turn_speed_limit: aircraft.preparation_turn_speed_limit,
            climb_speed_coef: aircraft.climb_speed_coef,
            dive_speed_coef: aircraft.dive_speed_coef,
            angle_of_climb: aircraft.angle_of_climb,
            angle_of_dive: aircraft.angle_of_dive,
            post_attack_invulnerability_duration: aircraft
                .post_attack_invulnerability_duration,
            jato_duration: aircraft.jato_duration,
            jato_speed_multiplier: aircraft.jato_speed_multiplier,
            max_forsage_amount: aircraft.max_forsage_amount,
            forsage_regeneration: aircraft.forsage_regeneration,
            speed_min: aircraft.speed_min,
            speed_max: aircraft.speed_max,
            attacker_damage_taken_multiplier: aircraft.attacker_damage_taken_multiplier,
            damage_taken_multiplier: aircraft.damage_taken_multiplier,
            bomb_falling_time: aircraft.bomb_falling_time,
            bombing_drop_point_time: aircraft.bombing_drop_point_time,
            empty_return_speed_multiplier: aircraft.empty_return_speed_multiplier,
            max_rotate_speed: aircraft.max_rotate_speed,
            plane_speedup_coef: aircraft.plane_speedup_coef,
            can_stop: aircraft.can_stop,
            max_number_on_deck: aircraft.max_number_on_deck,
            restoration_time: aircraft.restoration_time,
            restore_amount: aircraft.restore_amount,
            start_on_deck: aircraft.start_on_deck,
            plane_consumables,
            adjusted_health: aircraft.health
                * mods.multiply(ship_class, "planeHealthCoeff")
                * mods.multiply(ship_class, health_key),
            adjusted_speed: aircraft.speed
                * mods.multiply(ship_class, "planeSpeed")
                * mods.multiply(ship_class, speed_key),
        }
    }
}
