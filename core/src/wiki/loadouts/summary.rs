//! Modifier summary formatting.

use crate::wiki::modifiers::{ModifierSet, ModifierValue};
use crate::wiki::LangMap;

fn fmt_percent(value: f64) -> String {
    if value >= 1.0 {
        format!("+{:.0}%", (value - 1.0) * 100.0)
    } else {
        format!("-{:.0}%", (1.0 - value) * 100.0)
    }
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
            if (resolved - 1.0).abs() < f64::EPSILON {
                return None;
            }
            let label = match lang.get_raw(&format!("IDS_PARAMS_MODIFIER_{}", key.to_uppercase()))
            {
                Some(translated) => translated.to_string(),
                None => humanize_modifier_key(key),
            };
            Some(format!("{} {}", label, fmt_percent(resolved)))
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
        "GLShotDelay" => "Torpedo launcher reload",
        "GLAlphaFactor" => "Torpedo damage",
        "torpedoDamageCoeff" => "Torpedo damage",
        "speedCoef" => "Speed",
        "AAAuraDamage" => "AA damage",
        "allConsumableReloadTime" => "Consumable reload",
        "vulnerabilityBurn" => "Fire vulnerability",
        "vulnerabilityFlood" => "Flood vulnerability",
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
            if (resolved - 1.0).abs() < f64::EPSILON {
                return None;
            }
            let label = lang
                .get_raw(&format!("IDS_PARAMS_MODIFIER_{}", key.to_uppercase()))
                .unwrap_or(key)
                .to_string();
            Some(format!("{} {}", label, fmt_percent(resolved)))
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
            if (resolved - 1.0).abs() < f64::EPSILON {
                return None;
            }
            let label = lang
                .get_raw(&format!("IDS_PARAMS_MODIFIER_{}", key.to_uppercase()))
                .unwrap_or(key)
                .to_string();
            Some(format!("{} {}", label, fmt_percent(resolved)))
        })
        .take(3)
        .collect::<Vec<_>>()
        .join(" 路 ")
}

