//! Recent-overview processing.

use serde_json::{Map, Value};

use super::guard::guard;
use crate::models::{RecentDay, RecentOverview};


/// Parse `/wows/account/statsbydate/` into a recent overview. The API
/// returns cumulative daily totals, so per-day values are derived from the
/// differences between consecutive days.
#[must_use]
pub fn parse_recent_overview(json: &Value, account_id: u64) -> Option<RecentOverview> {
    let empty = Value::Object(Map::new());
    let data = guard(json, "data", &empty);
    let account = data.get(account_id.to_string())?;

    struct Record {
        battles: i64,
        wins: i64,
        damage: i64,
    }
    let mut records: Vec<(String, Record)> = account
        .as_object()?
        .iter()
        .filter_map(|(date, entry)| {
            let pvp = entry.get("pvp")?;
            Some((
                date.clone(),
                Record {
                    battles: pvp.get("battles").and_then(Value::as_i64).unwrap_or(0),
                    wins: pvp.get("wins").and_then(Value::as_i64).unwrap_or(0),
                    damage: pvp
                        .get("damage_dealt")
                        .and_then(Value::as_i64)
                        .unwrap_or(0),
                },
            ))
        })
        .collect();
    records.sort_by(|a, b| a.0.cmp(&b.0));

    let mut days = Vec::new();
    let mut previous: Option<&Record> = None;
    for (date, record) in &records {
        if let Some(previous) = previous {
            let battles = record.battles - previous.battles;
            let wins = record.wins - previous.wins;
            let damage = record.damage - previous.damage;
            if battles > 0 {
                days.push(RecentDay {
                    date: date.clone(),
                    battles,
                    winrate: wins as f64 / battles as f64 * 100.0,
                    avg_damage: damage as f64 / battles as f64,
                });
            }
        }
        previous = Some(record);
    }

    if days.is_empty() {
        return None;
    }
    let total_battles: i64 = days.iter().map(|d| d.battles).sum();
    let avg_winrate = days.iter().map(|d| d.winrate).sum::<f64>() / days.len() as f64;
    let avg_damage = days.iter().map(|d| d.avg_damage).sum::<f64>() / days.len() as f64;
    Some(RecentOverview {
        days,
        total_battles,
        avg_winrate,
        avg_damage,
    })
}
