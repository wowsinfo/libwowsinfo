//! Game-data localisation (`lang.json`).
//!
//! The game data stores every display string behind an `IDS_...` key. The
//! bundled `lang.json` maps those keys to the four shipped languages; the wiki
//! resolves names client-side from the map for the configured language.

use std::collections::HashMap;

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A resolved `IDS_...` -> display string map for one language.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LangMap {
    entries: HashMap<String, String>,
}

/// Display-ready unit suffixes resolved from `lang.json` (e.g. `IDS_KNOT`).
///
/// Latin-script units carry a leading space (`" knots"`, `" s"`) so templates
/// can append them directly; CJK units are bare (`"节"`, `"秒"`) so values
/// read naturally as `10节`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
#[repr(C)]
pub struct LocalizedUnits {
    pub knots: String,
    pub seconds: String,
    pub kilometer: String,
    pub meter: String,
    pub meter_per_second: String,
    pub millimeter: String,
    pub kilogram: String,
}

impl LocalizedUnits {
    /// Resolve the unit suffixes for the current language, falling back to
    /// the game's English forms when a key is missing.
    #[must_use]
    pub fn from_lang(lang: &LangMap) -> Self {
        let suffix = |key: &str, fallback: &str| {
            let value = lang.get_raw(key).unwrap_or(fallback);
            if value.chars().any(is_cjk) {
                value.to_string()
            } else {
                format!(" {value}")
            }
        };
        Self {
            knots: suffix("IDS_KNOT", "knots"),
            seconds: suffix("IDS_SECOND", "s"),
            kilometer: suffix("IDS_KILOMETER", "km"),
            meter: suffix("IDS_METER", "m"),
            meter_per_second: suffix("IDS_METER_SECOND", "m/s"),
            millimeter: suffix("IDS_MILLIMETER", "mm"),
            kilogram: suffix("IDS_KILOGRAMM", "kg"),
        }
    }
}

fn is_cjk(ch: char) -> bool {
    matches!(ch as u32, 0x4E00..=0x9FFF | 0x3400..=0x4DBF | 0xF900..=0xFAFF)
}

impl LangMap {
    /// Look up a localisation key, falling back to the raw key.
    #[must_use]
    pub fn get(&self, key: &str) -> String {
        self.entries
            .get(key)
            .filter(|v| !v.is_empty())
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// Look up a key without any fallback.
    #[must_use]
    pub fn get_raw(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Number of resolved entries (used by tests and diagnostics).
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Build a map from raw key/value pairs (mainly for tests).
    #[must_use]
    pub fn from_entries(entries: impl IntoIterator<Item = (String, String)>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
        }
    }
}

/// Extract one language section from `lang.json` (`{ "<lang>": {...} }`).
#[must_use]
pub fn parse_lang(json: &Value, language: &str) -> LangMap {
    let mut entries = HashMap::new();
    if let Some(map) = json.get(language).and_then(Value::as_object) {
        for (key, value) in map {
            if let Some(text) = value.as_str() {
                entries.insert(key.clone(), text.to_string());
            }
        }
    }
    LangMap { entries }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_one_language() {
        let json = json!({
            "en": {"IDS_HELLO": "Hello", "IDS_EMPTY": "", "IDS_X": 42},
            "ja": {"IDS_HELLO": "Kon'nichiwa"}
        });
        let en = parse_lang(&json, "en");
        assert_eq!(en.get("IDS_HELLO"), "Hello");
        assert_eq!(en.get("IDS_MISSING"), "IDS_MISSING");
        assert_eq!(en.get("IDS_EMPTY"), "IDS_EMPTY");
        assert_eq!(en.len(), 2);
        assert!(en.get_raw("IDS_MISSING").is_none());
    }

    #[test]
    fn unknown_language_is_empty() {
        let json = json!({"en": {"IDS_A": "a"}});
        assert!(parse_lang(&json, "fr").is_empty());
    }

    #[test]
    fn localized_units_follow_language_script() {
        let en = LangMap::from_entries([
            ("IDS_KNOT".to_string(), "knots".to_string()),
            ("IDS_SECOND".to_string(), "s".to_string()),
            ("IDS_KILOMETER".to_string(), "km".to_string()),
            ("IDS_METER".to_string(), "m".to_string()),
            ("IDS_METER_SECOND".to_string(), "m/s".to_string()),
            ("IDS_MILLIMETER".to_string(), "mm".to_string()),
            ("IDS_KILOGRAMM".to_string(), "kg".to_string()),
        ]);
        let en_units = LocalizedUnits::from_lang(&en);
        assert_eq!(en_units.knots, " knots");
        assert_eq!(en_units.seconds, " s");

        let zh = LangMap::from_entries([
            ("IDS_KNOT".to_string(), "节".to_string()),
            ("IDS_SECOND".to_string(), "秒".to_string()),
            ("IDS_KILOMETER".to_string(), "公里".to_string()),
            ("IDS_METER".to_string(), "米".to_string()),
            ("IDS_METER_SECOND".to_string(), "米/秒".to_string()),
            ("IDS_MILLIMETER".to_string(), "毫米".to_string()),
            ("IDS_KILOGRAMM".to_string(), "千克".to_string()),
        ]);
        let zh_units = LocalizedUnits::from_lang(&zh);
        assert_eq!(zh_units.knots, "节");
        assert_eq!(zh_units.seconds, "秒");
        assert_eq!(zh_units.kilometer, "公里");

        // Missing keys fall back to the English forms (space-prefixed).
        let fallback = LocalizedUnits::from_lang(&LangMap::default());
        assert_eq!(fallback.knots, " knots");
        assert_eq!(fallback.meter_per_second, " m/s");
    }
}
