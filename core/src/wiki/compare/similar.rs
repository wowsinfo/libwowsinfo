//! Similar-ship resolution.

use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::LangMap;

use super::types::SimilarShip;

/// Ships of the same tier and type as `ship` (up to 24, sorted by nation).
#[must_use]
pub fn similar_ships(data: &GameData, lang: &LangMap, ship: &ShipInfo) -> Vec<SimilarShip> {
    let mut out: Vec<SimilarShip> = data
        .ships
        .iter()
        .filter(|(id, candidate)| {
            **id != ship.id
                && candidate.tier == ship.tier
                && candidate.r#type == ship.r#type
                && candidate.paper_ship == ship.paper_ship
        })
        .map(|(id, candidate)| SimilarShip {
            ship_id: *id,
            index: candidate.index.clone(),
            name: lang.get(&candidate.name),
            tier: candidate.tier,
            nation: candidate.region.clone(),
            ship_type: candidate.r#type.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.nation.cmp(&b.nation).then(a.name.cmp(&b.name)));
    out.truncate(24);
    out
}
