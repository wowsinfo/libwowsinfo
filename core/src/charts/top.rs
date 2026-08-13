//! Top ships by battles, the list behind the "top ten" chart lines.

use crate::models::ShipStatLine;

/// One entry in the top-ships list.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TopShip {
    pub ship_id: u64,
    pub name: String,
    pub tier: i64,
    pub r#type: String,
    pub battles: i64,
    pub rating: f64,
}

/// The `limit` most-played ships, ordered by battles (ties by rating), like the
/// app's top-ten charts.
#[must_use]
pub fn top_ships(ships: &[ShipStatLine], limit: usize) -> Vec<TopShip> {
    let mut result: Vec<TopShip> = ships
        .iter()
        .filter(|ship| ship.battles > 0)
        .map(|ship| TopShip {
            ship_id: ship.ship_id,
            name: ship.name.clone(),
            tier: ship.tier,
            r#type: ship.r#type.clone(),
            battles: ship.battles,
            rating: ship.rating,
        })
        .collect();
    result.sort_by(|a, b| b.battles.cmp(&a.battles).then(b.rating.total_cmp(&a.rating)));
    result.truncate(limit);
    result
}
