//! Game-data localisation (`lang.json`).
//!
//! The game data stores every display string behind an `IDS_...` key. The
//! bundled `lang.json` maps those keys to the four shipped languages; the wiki
//! resolves names client-side from the map for the configured language.

use std::collections::HashMap;

use serde_json::Value;

/// A resolved `IDS_...` -> display string map for one language.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LangMap {
    entries: HashMap<String, String>,
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
}
