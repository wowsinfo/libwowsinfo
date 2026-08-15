//! Air-support plane resolution.

use super::detail::AircraftDetail;
use crate::wiki::gamedata::GameData;
use crate::wiki::modifiers::ModifierSet;
use crate::wiki::LangMap;

/// Resolve the air-support plane (scout/fighter) for a ship, if any.
#[must_use]
pub fn air_support_plane(
    data: &GameData,
    lang: &LangMap,
    plane_key: &str,
    mods: &ModifierSet,
    ship_class: &str,
) -> Option<AircraftDetail> {
    data.aircraft
        .get(plane_key)
        .map(|aircraft| AircraftDetail::from_aircraft(lang, data, aircraft, mods, ship_class))
}
