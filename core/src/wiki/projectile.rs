//! Projectile parsing (`wowsinfo.json -> projectiles`).
//!
//! Every shell, bomb and torpedo in the game data is keyed by its internal
//! name (for example `PAPA011_Shell_406mm_AP_AP_Mk_8`). The parsed structs
//! feed the wiki's shell cards and the drag-based AP penetration chart.

use std::collections::HashMap;

use serde_json::Value;

/// Armor-piercing ballistic block (`projectiles.<name>.ap`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApInfo {
    pub diameter_m: f64,
    pub weight_kg: f64,
    pub drag: f64,
    pub velocity: f64,
    pub krupp: f64,
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
}

impl ProjectileInfo {
    /// Calibre in millimetres (convenience for the penetration model).
    #[must_use]
    pub fn calibre_mm(&self) -> f64 {
        self.diameter * 1000.0
    }
}

fn f64_field(json: &Value, key: &str) -> f64 {
    json.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn opt_f64(json: &Value, key: &str) -> Option<f64> {
    json.get(key)
        .filter(|v| !v.is_null())
        .and_then(Value::as_f64)
}

fn opt_i64(json: &Value, key: &str) -> Option<i64> {
    json.get(key)
        .filter(|v| !v.is_null())
        .and_then(Value::as_i64)
}

fn str_field(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

/// Parse the `projectiles` section into a map keyed by projectile name.
#[must_use]
pub fn parse_projectiles(json: &Value) -> HashMap<String, ProjectileInfo> {
    let mut out = HashMap::new();
    let Some(map) = json.as_object() else {
        return out;
    };
    for (key, value) in map {
        let ap = value.get("ap").filter(|v| !v.is_null()).map(|ap| ApInfo {
            diameter_m: f64_field(ap, "diameter"),
            weight_kg: f64_field(ap, "weight"),
            drag: f64_field(ap, "drag"),
            velocity: f64_field(ap, "velocity"),
            krupp: f64_field(ap, "krupp"),
        });
        out.insert(
            key.clone(),
            ProjectileInfo {
                key: key.clone(),
                r#type: str_field(value, "type"),
                nation: str_field(value, "nation"),
                name: str_field(value, "name"),
                ammo_type: str_field(value, "ammoType"),
                speed: f64_field(value, "speed"),
                weight: f64_field(value, "weight"),
                damage: f64_field(value, "damage"),
                diameter: f64_field(value, "diameter"),
                ricochet_angle: opt_f64(value, "ricochetAngle"),
                ricochet_always: opt_f64(value, "ricochetAlways"),
                pen_he: opt_f64(value, "penHE"),
                pen_sap: opt_f64(value, "penSAP"),
                burn_chance: opt_f64(value, "burnChance"),
                overmatch: opt_i64(value, "overmatch"),
                fuse_time: opt_f64(value, "fuseTime"),
                range: opt_f64(value, "range"),
                flood_chance: opt_f64(value, "floodChance"),
                visibility: opt_f64(value, "visibility"),
                alpha_damage: opt_f64(value, "alphaDamage"),
                deep_water: value
                    .get("deepWater")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                ap,
            },
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_ap_and_he_shells() {
        let json = json!({
            "PAPA011_Shell_406mm_AP_AP_Mk_8": {
                "type": "Artillery", "nation": "USA",
                "name": "IDS_PAPA011_SHELL_406MM_AP_AP_MK_8",
                "ammoType": "AP", "speed": 701.0, "weight": 1225.0,
                "damage": 13100.0, "ricochetAngle": 45.0,
                "ricochetAlways": 60.0, "diameter": 0.406,
                "ap": {"diameter": 0.406, "weight": 1225.0, "drag": 0.352,
                       "velocity": 701.0, "krupp": 2598.0},
                "overmatch": 28, "fuseTime": 0.033
            },
            "PAPA002_Shell_203mm_HE_HC_Mk_25": {
                "type": "Artillery", "nation": "USA",
                "name": "IDS_PAPA002_SHELL_203MM_HE_HC_MK_25",
                "ammoType": "HE", "speed": 823.0, "weight": 118.0,
                "damage": 2800.0, "penHE": 34.0, "burnChance": 0.14,
                "diameter": 0.203
            }
        });
        let map = parse_projectiles(&json);
        assert_eq!(map.len(), 2);
        let ap = &map["PAPA011_Shell_406mm_AP_AP_Mk_8"];
        assert_eq!(ap.ammo_type, "AP");
        assert_eq!(ap.overmatch, Some(28));
        assert_eq!(ap.calibre_mm(), 406.0);
        let ap_block = ap.ap.as_ref().expect("ap block");
        assert_eq!(ap_block.krupp, 2598.0);
        assert_eq!(ap_block.drag, 0.352);
        let he = &map["PAPA002_Shell_203mm_HE_HC_Mk_25"];
        assert_eq!(he.pen_he, Some(34.0));
        assert_eq!(he.burn_chance, Some(0.14));
        assert!(he.ap.is_none());
    }

    #[test]
    fn parses_torpedoes() {
        let json = json!({
            "PAPT001_Torpedo_533mm_Mk_15": {
                "type": "Torpedo", "nation": "USA",
                "name": "IDS_PAPT001_TORPEDO_533MM_MK_15",
                "speed": 55.0, "visibility": 1.1, "range": 305.0,
                "floodChance": 190.0, "alphaDamage": 32100.0,
                "damage": 900.0, "deepWater": false
            }
        });
        let map = parse_projectiles(&json);
        let torp = &map["PAPT001_Torpedo_533mm_Mk_15"];
        assert_eq!(torp.r#type, "Torpedo");
        assert_eq!(torp.range, Some(305.0));
        assert_eq!(torp.flood_chance, Some(190.0));
        assert_eq!(torp.visibility, Some(1.1));
        assert_eq!(torp.alpha_damage, Some(32100.0));
    }

    #[test]
    fn tolerant_of_missing_fields() {
        let json = json!({ "weird": {"name": "IDS_X"} });
        let map = parse_projectiles(&json);
        let entry = &map["weird"];
        assert_eq!(entry.damage, 0.0);
        assert!(entry.ap.is_none());
        assert!(entry.pen_he.is_none());
    }
}
