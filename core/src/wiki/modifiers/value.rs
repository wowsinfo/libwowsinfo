//! Modifier values and parsing.

use std::collections::HashMap;

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A modifier value: common to all classes or per ship class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Facet)]
#[repr(C)]
pub enum ModifierValue {
    Number(f64),
    PerShipType(HashMap<String, f64>),
}

impl ModifierValue {
    /// Resolve the value for one ship class (defaults to 1.0 = no effect).
    #[must_use]
    pub fn for_class(&self, ship_class: &str) -> f64 {
        match self {
            ModifierValue::Number(value) => *value,
            ModifierValue::PerShipType(map) => map.get(ship_class).copied().unwrap_or(1.0),
        }
    }
}

/// An ordered modifier set (order matters for the merge rule).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ModifierSet {
    pub entries: Vec<(String, ModifierValue)>,
}

impl ModifierSet {
    /// Merge another set into this one. Numbers multiply; additive keys
    /// (`Additional`, `Extra`) are summed so counts like "+1 consumable" work.
    #[must_use]
    pub fn merged(mut self, other: &ModifierSet) -> Self {
        for (key, value) in &other.entries {
            if let Some((_, existing)) = self.entries.iter_mut().find(|(k, _)| k == key) {
                *existing = merge_values(existing, value, is_additive_key(key));
                continue;
            }
            self.entries.push((key.clone(), value.clone()));
        }
        self
    }

    /// Product of every entry with `key`, resolved for `ship_class`.
    #[must_use]
    pub fn multiply(&self, ship_class: &str, key: &str) -> f64 {
        self.entries
            .iter()
            .filter(|(k, _)| k == key)
            .map(|(_, value)| value.for_class(ship_class))
            .product::<f64>()
    }

    /// Sum of every entry with `key` (used for additive counts).
    #[must_use]
    pub fn sum(&self, ship_class: &str, key: &str) -> f64 {
        self.entries
            .iter()
            .filter(|(k, _)| k == key)
            .map(|(_, value)| value.for_class(ship_class))
            .sum::<f64>()
    }

    /// True when the set contains at least one entry for `key`.
    #[must_use]
    pub fn has(&self, key: &str) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }
}

/// True for keys whose values are additive counts rather than ratios
/// (`+1 consumable`, `+2 flak clouds`), where a value of `1.0` is a real
/// change and `0.0` is the neutral value.
#[must_use]
pub fn is_additive_key(key: &str) -> bool {
    key.contains("Additional")
        || key.contains("Extra")
        || key == "additionalConsumables"
        // Absolute-value modifiers from the game params: counts, distances
        // and replacement times are additive, not ratios.
        || matches!(
            key,
            "dcNumPacksBonus"
                | "planeHealthPerLevel"
                | "prioritySectorStrengthBonus"
                | "ignorePTZBonus"
                | "uwCoeffBonus"
                | "visionXRayTorpedoDist"
                | "fighterAimingTime"
                | "torpedoBomberAimingTime"
                | "skipBomberAimingTime"
        )
}

fn merge_values(a: &ModifierValue, b: &ModifierValue, additive: bool) -> ModifierValue {
    match (a, b) {
        (ModifierValue::Number(a), ModifierValue::Number(b)) => {
            ModifierValue::Number(if additive { a + b } else { a * b })
        }
        (ModifierValue::PerShipType(a), ModifierValue::PerShipType(b)) => {
            let mut out = a.clone();
            for (class, value) in b {
                let entry = out.entry(class.clone()).or_insert(1.0);
                *entry = if additive { *entry + *value } else { *entry * *value };
            }
            ModifierValue::PerShipType(out)
        }
        _ => a.clone(),
    }
}

/// Parse a game-data `modifiers` map into a set.
#[must_use]
pub fn parse_modifiers(json: &Value) -> ModifierSet {
    let mut entries = Vec::new();
    let Some(map) = json.as_object() else {
        return ModifierSet { entries };
    };
    for (key, value) in map {
        let parsed = if let Some(number) = value.as_f64() {
            ModifierValue::Number(number)
        } else if let Some(classes) = value.as_object() {
            let per_class: HashMap<String, f64> = classes
                .iter()
                .filter_map(|(class, v)| v.as_f64().map(|number| (class.clone(), number)))
                .collect();
            ModifierValue::PerShipType(per_class)
        } else {
            continue;
        };
        entries.push((key.clone(), parsed));
    }
    ModifierSet { entries }
}
