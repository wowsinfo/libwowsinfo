//! Parser and team-splitting helpers for `tempArenaInfo.json`.

use serde_json::Value;

use super::{ArenaInfo, ArenaVehicle, Team};

/// Parse a `tempArenaInfo.json` document. Returns `None` when the payload is
/// not valid JSON or the file is empty (the RS server answers `[]` until a
/// match starts).
#[must_use]
pub fn parse_arena(json: &Value) -> Option<ArenaInfo> {
    if json.is_null() || json.as_array().is_some() {
        return None;
    }
    serde_json::from_value::<ArenaInfo>(json.clone()).ok()
}

/// Bots/AI ships use `:`-prefixed names in the arena file and are skipped by
/// the app's RS client when resolving player stats.
#[must_use]
pub fn is_bot(name: &str) -> bool {
    name.starts_with(':')
}

/// Team of a vehicle: `relation < 2` means friendly, otherwise enemy.
#[must_use]
pub fn team_of(vehicle: &ArenaVehicle) -> Team {
    if vehicle.relation < 2 {
        Team::Ally
    } else {
        Team::Enemy
    }
}

/// Split the arena's vehicles into friendly and enemy teams, preserving file
/// order. Bots stay in their team (they are still part of the match).
#[must_use]
pub fn teams(arena: &ArenaInfo) -> (Vec<&ArenaVehicle>, Vec<&ArenaVehicle>) {
    let mut ally = Vec::new();
    let mut enemy = Vec::new();
    for vehicle in &arena.vehicles {
        match team_of(vehicle) {
            Team::Ally => ally.push(vehicle),
            Team::Enemy => enemy.push(vehicle),
        }
    }
    (ally, enemy)
}

/// Player stat lookups for an arena: `(account name, ship id)` pairs for every
/// non-bot vehicle, in file order. The shell resolves each pair against the
/// WG API (player search by name, then ship stats) like the RS client.
#[must_use]
pub fn stat_lookups(arena: &ArenaInfo) -> Vec<(String, u64)> {
    arena
        .vehicles
        .iter()
        .filter(|vehicle| !is_bot(&vehicle.name))
        .map(|vehicle| (vehicle.name.clone(), vehicle.ship_id))
        .collect()
}
