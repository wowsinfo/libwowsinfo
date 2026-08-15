//! Build the comparison table for a list of ships.

use super::rows::ship_values;
use super::types::{CompareRow, CompareShipHeader, LocalCompare, ROW_LABELS};
use crate::wiki::gamedata::{GameData, ShipInfo};
use crate::wiki::LangMap;

/// Build the comparison table for `ship_ids` (best effort per ship).
#[must_use]
pub fn build_local_compare(
    data: &GameData,
    lang: &LangMap,
    ship_ids: &[u64],
) -> Option<LocalCompare> {
    if ship_ids.is_empty() {
        return None;
    }
    let ships: Vec<&ShipInfo> = ship_ids
        .iter()
        .filter_map(|id| data.ships.get(id))
        .collect();
    if ships.is_empty() {
        return None;
    }
    let headers = ships
        .iter()
        .map(|ship| CompareShipHeader {
            ship_id: ship.id,
            index: ship.index.clone(),
            name: lang.get(&ship.name),
            tier: ship.tier,
        })
        .collect();
    let columns: Vec<Vec<String>> = ships
        .iter()
        .map(|ship| ship_values(data, lang, ship))
        .collect();
    let rows = ROW_LABELS
        .iter()
        .enumerate()
        .map(|(index, label)| CompareRow {
            label: (*label).to_string(),
            values: columns.iter().map(|column| column[index].clone()).collect(),
        })
        .collect();
    Some(LocalCompare { ships: headers, rows })
}

