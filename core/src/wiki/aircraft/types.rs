//! Aircraft type definitions.

/// One aircraft entry (`aircrafts.<name>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AircraftInfo {
    pub key: String,
    pub r#type: String,
    pub nation: String,
    /// Localisation key (`IDS_...`).
    pub name: String,
    pub health: f64,
    pub total_planes: i64,
    pub visibility: f64,
    pub speed: f64,
    pub attack_count: Option<i64>,
    pub attacker: Option<i64>,
    pub max_aircraft: Option<i64>,
    pub restore_time: Option<f64>,
    pub bomb_name: Option<String>,
    // Top-level squadron fields (v15.7 `aircrafts.<name>`).
    pub attack_cooldown: Option<f64>,
    pub attack_interval: Option<f64>,
    pub aiming_time: Option<f64>,
    pub aiming_speed_limit_min: Option<f64>,
    pub aiming_speed_limit_max: Option<f64>,
    pub aiming_accuracy_increase_rate: Option<f64>,
    pub aiming_accuracy_decrease_rate: Option<f64>,
    pub aiming_turn_speed_limit: Option<f64>,
    pub preparation_time: Option<f64>,
    pub preparation_speed_limit_min: Option<f64>,
    pub preparation_speed_limit_max: Option<f64>,
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
    pub visibility_factor_by_plane: Option<f64>,
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
    pub plane_consumables: Vec<PlaneConsumableSlot>,
}

/// One plane consumable slot (`planeConsumables.AbilitySlotN`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlaneConsumableSlot {
    pub slot: i64,
    pub abilities: Vec<String>,
    pub special: bool,
}

