//! Next-ship views.

use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::LangMap;

use super::views::NextShip;

/// Resolve the research-tree entries for a ship.
#[must_use]
pub fn next_ship_views(data: &GameData, lang: &LangMap, ship: &ShipInfo) -> Vec<NextShip> {
    ship.next_ships
        .iter()
        .filter_map(|id| {
            data.ships.get(id).map(|next| NextShip {
                ship_id: *id,
                index: next.index.clone(),
                name: lang.get(&next.name),
                tier: next.tier,
            })
        })
        .collect()
}

