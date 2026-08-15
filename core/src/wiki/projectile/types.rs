//! Projectile type definitions.

/// Armor-piercing ballistic block (`projectiles.<name>.ap`).

/// Armor-piercing ballistic block (`projectiles.<name>.ap`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApInfo {
    pub diameter_m: f64,
    pub weight_kg: f64,
    pub drag: f64,
    pub velocity: f64,
    pub krupp: f64,
}

/// Acoustic homing block of a torpedo (`acousticDetection`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AcousticDetectionInfo {
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

/// One projectile entry (`projectiles.<name>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProjectileInfo {
    pub key: String,
    pub r#type: String,
    pub nation: String,
    /// Localisation key (`IDS_...`) resolved against the bundled lang data.
    pub name: String,
    pub ammo_type: String,
    pub speed: f64,
    pub weight: f64,
    pub damage: f64,
    /// Calibre in metres (0.406 for a 406 mm shell).
    pub diameter: f64,
    pub ricochet_angle: Option<f64>,
    pub ricochet_always: Option<f64>,
    pub pen_he: Option<f64>,
    pub pen_sap: Option<f64>,
    pub burn_chance: Option<f64>,
    pub overmatch: Option<i64>,
    pub fuse_time: Option<f64>,
    pub range: Option<f64>,
    pub flood_chance: Option<f64>,
    pub visibility: Option<f64>,
    pub alpha_damage: Option<f64>,
    pub deep_water: bool,
    pub ap: Option<ApInfo>,
    /// Torpedo fields (v15.7 `projectiles.<name>`).
    pub arming_distance: Option<f64>,
    pub depth: Option<f64>,
    pub splash_armor_coeff: Option<f64>,
    pub splash_cube_size: Option<f64>,
    pub underwater_splash_damage_multiplier: Option<f64>,
    /// Ping damage coefficient (`damageCoeffMaxPing`).
    pub damage_coeff_max_ping: Option<f64>,
    pub acoustic_detection: Option<AcousticDetectionInfo>,
    pub maneuver_dist: Option<f64>,
    /// Acoustic detection range (`alertDist`).
    pub alert_dist: Option<f64>,
    pub can_hit_classes: Vec<String>,
    /// Depth-charge ammo fields (v15.7 `projectiles.<name>`, type
    /// `DepthCharge`). `sinkSpeed` mirrors the game's `bulletSpeed` raw value.
    pub sink_speed: Option<f64>,
    pub detonation_depth: Option<f64>,
    pub splash_radius: Option<f64>,
    pub fire_chance: Option<f64>,
    pub flood_generation: Option<bool>,
    /// Depth range -> damage coefficient segments (`pointsOfDamage`).
    pub points_of_damage: Vec<(f64, f64)>,
    pub ignore_classes: Vec<String>,
    pub explosive_power: Option<f64>,
    pub integral_power: Option<f64>,
    pub fall_distance: Option<f64>,
    pub fall_time: Option<f64>,
    /// Buoyancy state -> damage coefficient (`buoyancyToDamageCoeff`).
    pub buoyancy_to_damage_coeff: Vec<(String, f64)>,
    // Ballistics fields (every projectile, zero gaps).
    pub air_drag: Option<f64>,
    pub arming_threshold: Option<f64>,
    pub cap_normalize_max_angle: Option<f64>,
    pub explosion_radius: Option<f64>,
    pub krupp: Option<f64>,
    pub shell_cap: Option<bool>,
    pub underwater_dist_factor: Option<f64>,
    pub underwater_penetration_factor: Option<f64>,
    pub water_drag: Option<f64>,
    /// Dispersion distance parameters (`distParams`, 4 values) + tile size.
    pub dist_params: Vec<f64>,
    pub dist_tile: Option<f64>,
}

impl ProjectileInfo {
    /// Calibre in millimetres (convenience for the penetration model).
    #[must_use]
    pub fn calibre_mm(&self) -> f64 {
        self.diameter * 1000.0
    }
}

