//! Aircraft-view helpers (type keys + component indirection).

use serde_json::Value;

use crate::wiki::aircraft::AircraftInfo;
use crate::wiki::gamedata::{GameData, ShipInfo};

pub(super) fn aircraft_type_keys(r#type: &str) -> (&'static str, &'static str) {
    match r#type {
        "Fighter" => ("fighterHealth", "planeSpeed"),
        "Dive" => ("diveBomberHealth", "diveBomberSpeedMultiplier"),
        "Bomber" => ("torpedoBomberHealth", "torpedoSpeedMultiplier"),
        "Skip" => ("skipBomberHealth", "skipBomberSpeedMultiplier"),
        _ => ("planeHealthCoeff", "planeSpeed"),
    }
}

/// Resolve a squadron's aircraft key, following one level of indirection
/// through the ship's `components` (some options list component ids that are
/// arrays of aircraft keys instead of the aircraft keys themselves).
pub(super) fn resolve_aircraft<'a>(
    data: &'a GameData,
    ship: &'a ShipInfo,
    key: &str,
) -> Option<&'a AircraftInfo> {
    if let Some(aircraft) = data.aircraft.get(key) {
        return Some(aircraft);
    }
    ship.components
        .get(key)
        .and_then(Value::as_array)
        .and_then(|ids| ids.first())
        .and_then(Value::as_str)
        .and_then(|resolved| data.aircraft.get(resolved))
}
