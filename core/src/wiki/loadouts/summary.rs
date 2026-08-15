//! Modifier summary formatting.

use crate::wiki::modifiers::{is_additive_key, ModifierSet, ModifierValue};
use crate::wiki::LangMap;

/// Ship classes used for the per-class localisation variants
/// (`IDS_PARAMS_MODIFIER_<KEY>_<CLASS>`).
const MODIFIER_CLASSES: [&str; 6] = [
    "AirCarrier",
    "Auxiliary",
    "Battleship",
    "Cruiser",
    "Destroyer",
    "Submarine",
];

fn fmt_percent(value: f64) -> String {
    if value >= 1.0 {
        format!("+{:.0}%", (value - 1.0) * 100.0)
    } else {
        format!("-{:.0}%", (1.0 - value) * 100.0)
    }
}

/// Additive keys are counts (charges, bubbles, hangar size), not ratios.
fn fmt_modifier_value(key: &str, value: f64) -> String {
    if is_additive_key(key) {
        format!("{value:+}")
    } else {
        fmt_percent(value)
    }
}

/// A neutral value means "no effect": `1.0` for ratios, `0.0` for counts.
fn is_neutral(key: &str, value: f64) -> bool {
    if is_additive_key(key) {
        value == 0.0
    } else {
        (value - 1.0).abs() < f64::EPSILON
    }
}

/// Resolve the localised label for a modifier key: exact entry first, then the
/// per-class variant for `ship_class` (or any class), then a humanised label.
fn modifier_label(lang: &LangMap, ship_class: Option<&str>, key: &str) -> String {
    let base = format!("IDS_PARAMS_MODIFIER_{}", key.to_uppercase());
    if let Some(label) = lang.get_raw(&base) {
        return label.to_string();
    }
    if let Some(class) = ship_class
        && let Some(label) = lang.get_raw(&format!("{base}_{}", class.to_uppercase())) {
            return label.to_string();
        }
    for class in MODIFIER_CLASSES {
        if let Some(label) = lang.get_raw(&format!("{base}_{}", class.to_uppercase())) {
            return label.to_string();
        }
    }
    humanize_modifier_key(key)
}

/// One friendly line per modifier entry (label + percent), used by the
/// special-ability panel. Falls back to a humanised key label when the game
/// data has no localisation for it.
#[must_use]
pub fn modifier_lines(lang: &LangMap, mods: &ModifierSet) -> Vec<String> {
    mods.entries
        .iter()
        .filter_map(|(key, value)| {
            let resolved = match value {
                ModifierValue::Number(v) => *v,
                ModifierValue::PerShipType(map) => map
                    .values()
                    .copied()
                    .find(|v| (v - 1.0).abs() > f64::EPSILON)
                    .unwrap_or(1.0),
            };
            if is_neutral(key, resolved) {
                return None;
            }
            let label = modifier_label(lang, None, key);
            Some(format!("{} {}", label, fmt_modifier_value(key, resolved)))
        })
        .collect()
}

/// Friendly label for a game modifier key when no localisation exists.
fn humanize_modifier_key(key: &str) -> String {
    let label = match key {
        "GMShotDelay" => "Main gun reload",
        "GMMaxDist" => "Main gun range",
        "GMRotationSpeed" => "Main gun traverse",
        "GMIdealRadius" => "Main gun accuracy",
        "GMAlphaFactor" => "Main gun shell damage",
        "GMAPDamageCoeff" => "AP damage",
        "GMPenetrationCoeffHE" => "HE penetration",
        "GSShotDelay" => "Secondary reload",
        "GSMaxDist" => "Secondary range",
        "GSIdealRadius" => "Secondary accuracy",
        "GSAlphaFactor" => "Secondary shell damage",
        "GSAPDamageCoeff" => "Secondary AP damage",
        "GSPenetrationCoeffHE" => "Secondary HE penetration",
        "GTShotDelay" => "Torpedo reload",
        "GTRotationSpeed" => "Torpedo traverse",
        "GTMaxHP" => "Torpedo HP",
        "GTCritProb" => "Torpedo critical chance",
        "GLShotDelay" => "Torpedo launcher reload",
        "GLAlphaFactor" => "Torpedo damage",
        "torpedoDamageCoeff" => "Torpedo damage",
        "torpedoRangeCoefficient" => "Torpedo range",
        "torpedoSpeedMultiplier" => "Torpedo speed",
        "torpedoDetectionCoefficient" => "Torpedo detectability",
        "torpedoDetectionCoefficientByPlane" => "Torpedo detectability by air",
        "torpedoFullPingDamageCoeff" => "Torpedo full-ping damage",
        "speedCoef" => "Speed",
        "SGRudderPower" => "Rudder power",
        "SGCritRudderTime" => "Rudder critical time",
        "SGRepairTime" => "Rudder repair time",
        "AAAuraDamage" => "AA damage",
        "AAAuraReceiveDamageCoeff" => "AA damage taken",
        "AABubbleDamage" => "AA flak damage",
        "AAExtraBubbles" => "AA extra flak clouds",
        "AAMaxHP" => "AA mount HP",
        "GMDamageCoeff" => "Main gun damage",
        "GMHECSDamageCoeff" => "HE damage",
        "GMCritProb" => "Main gun critical chance",
        "GMMaxHP" => "Main gun HP",
        "GMRepairTime" => "Main gun repair time",
        "GMBigGunVisibilityCoeff" => "Main gun firing detectability",
        "GMHeavyCruiserCaliberDamageCoeff" => "Heavy cruiser gun damage",
        "GSMaxHP" => "Secondary gun HP",
        "GSPriorityTargetIdealRadius" => "Priority target accuracy",
        "consumableCapacityCoeff" => "Consumable capacity",
        "healthHullCoeff" => "Ship HP",
        "healthPerLevel" => "HP per level",
        "healthRegenPercent" => "HP regen",
        "reloadFactor" => "Reload",
        "activeManeuveringReloadCoeff" => "Reload while maneuvering",
        "visibilityFactor" => "Visibility",
        "visibilityDistCoeff" => "Ship detectability range",
        "visibilityForSubmarineCoeff" => "Submarine visibility",
        "planeVisibilityFactor" => "Aircraft detectability",
        "batteryCapacityCoeff" => "Submarine battery capacity",
        "batteryRegenCoeff" => "Submarine battery regen",
        "pingerReloadCoeff" => "Sonar reload",
        "pingerWaveSpeedCoeff" => "Sonar wave speed",
        "hydrophoneWaveSpeedCoeff" => "Hydrophone wave speed",
        "lastChanceReloadCoefficient" => "Reload reduction at low HP",
        "floodChanceFactor" => "Flood chance",
        "floodProb" => "Flood chance",
        "floodTime" => "Flood duration",
        "burnProb" => "Fire chance",
        "burnTime" => "Fire duration",
        "burnChanceFactorHighLevel" => "Fire chance (high tier)",
        "burnChanceFactorLowLevel" => "Fire chance (low tier)",
        "fireResistanceEnabled" => "Fire resistance",
        "vulnerabilityBurn" => "Fire vulnerability",
        "vulnerabilityFlood" => "Flood vulnerability",
        "engineForwardUpTime" => "Engine boost duration",
        "engineForwardForsagePower" => "Engine boost power",
        "engineForwardForsageMaxSpeed" => "Engine boost speed",
        "engineBackwardUpTime" => "Reverse boost duration",
        "engineBackwardForsagePower" => "Reverse boost power",
        "engineBackwardForsageMaxSpeed" => "Reverse boost speed",
        "engineRepairTime" => "Engine repair time",
        "engineCritProb" => "Engine critical chance",
        "artilleryAlertEnabled" => "Artillery alert",
        "artilleryAlertMinDistance" => "Artillery alert distance",
        "nearEnemyIntuitionEnabled" => "Enemy intuition",
        "nearRLSEnabled" => "Nearby radar warning",
        "priorityTargetEnabled" => "Priority target",
        "softCriticalEnabled" => "Soft critical hits",
        "excludedConsumables" => "Excluded consumables",
        "restoreForsage" => "Engine boost restore",
        "burnChanceGMGSMultiplier" => "Gun fire chance",
        "allConsumableReloadTime" => "Consumable reload",
        "artilleryKruppMultiplier" => "Krupp",
        "additionalMissilesRageModeOnly" => "Extra missiles (rage)",
        _ => return humanize_camel_key(key),
    };
    label.to_string()
}

/// Split a camelCase key into title-cased words (`GMMaxDist` -> "GM Max Dist").
fn humanize_camel_key(key: &str) -> String {
    let mut out = String::new();
    for (index, ch) in key.chars().enumerate() {
        if index > 0 && ch.is_uppercase() {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}

/// Format a modifier set into a short summary for the UI (up to 3 entries).
#[must_use]
pub fn modifier_summary(lang: &LangMap, ship_class: &str, mods: &ModifierSet) -> String {
    mods.entries
        .iter()
        .filter_map(|(key, value)| {
            let resolved = match value {
                ModifierValue::Number(v) => *v,
                ModifierValue::PerShipType(map) => map.get(ship_class).copied().unwrap_or(1.0),
            };
            if is_neutral(key, resolved) {
                return None;
            }
            let label = modifier_label(lang, Some(ship_class), key);
            Some(format!("{} {}", label, fmt_modifier_value(key, resolved)))
        })
        .take(3)
        .collect::<Vec<_>>()
        .join(" 路 ")
}

/// Summarise a modifier set for a wiki page where no ship class is selected:
/// per-class values fall back to the first class that actually changes the
/// stat (so the upgrade stays understandable outside a ship build).
#[must_use]
pub fn modifier_summary_any(lang: &LangMap, mods: &ModifierSet) -> String {
    mods.entries
        .iter()
        .filter_map(|(key, value)| {
            let resolved = match value {
                ModifierValue::Number(v) => *v,
                ModifierValue::PerShipType(map) => map
                    .values()
                    .copied()
                    .find(|v| (v - 1.0).abs() > f64::EPSILON)
                    .unwrap_or(1.0),
            };
            if is_neutral(key, resolved) {
                return None;
            }
            let label = modifier_label(lang, None, key);
            Some(format!("{} {}", label, fmt_modifier_value(key, resolved)))
        })
        .take(3)
        .collect::<Vec<_>>()
        .join(" 路 ")
}

