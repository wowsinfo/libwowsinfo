//! Anti-air component shapes and parsers.

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::helpers::{as_f64, as_i64};
use super::guns::{parse_weapon, WeaponInfo};

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


/// AA component with its three bands.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AirDefenseStats {
    pub near: Vec<AuraInfo>,
    pub medium: Vec<AuraInfo>,
    pub far: Vec<AuraInfo>,
    pub bubbles: Option<BubbleInfo>,
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

