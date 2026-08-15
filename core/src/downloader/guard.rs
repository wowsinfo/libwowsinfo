//! JSON guard / envelope helpers.

use serde_json::{Map, Value};

/// `Guard` in `src/core/util/SafeGuard.js`: walk a dotted path, returning
/// `dval` when anything along the path is null/missing.
#[must_use]
pub fn guard<'a>(json: &'a Value, path: &str, dval: &'a Value) -> &'a Value {
    if path.is_empty() && !json.is_null() {
        return json;
    }
    if path.starts_with('.') || path.ends_with('.') {
        return dval;
    }
    let mut current = json;
    for part in path.split('.') {
        current = match current.get(part) {
            Some(v) if !v.is_null() => v,
            _ => return dval,
        };
    }
    current
}

/// Cleanup shared by `getPR`/`readLocalPR`: drop empty array entries.
#[must_use]
pub fn clean_pr_data(data: Map<String, Value>) -> Map<String, Value> {
    data.into_iter()
        .filter(|(_, v)| !(v.is_array() && v.as_array().is_some_and(|a| a.is_empty())))
        .collect()
}
