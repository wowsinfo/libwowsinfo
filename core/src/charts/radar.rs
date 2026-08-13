//! Stats-vs-average radar values for a single ship.

use crate::models::ShipStatLine;

/// The three radar axes, as percentages of the expected/server-average values
/// (100 = exactly at average). Zero when no expected value is known.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RadarValues {
    pub damage: f64,
    pub winrate: f64,
    pub frags: f64,
}

fn pct(actual: f64, expected: f64) -> f64 {
    if expected > 0.0 {
        actual / expected * 100.0
    } else {
        0.0
    }
}

/// Damage / winrate / frags of the ship relative to its expected values, like
/// the community-assistant radar (player per-battle values vs averages).
#[must_use]
pub fn stats_vs_average(ship: &ShipStatLine) -> RadarValues {
    RadarValues {
        damage: pct(ship.avg_dmg, ship.expected_dmg),
        winrate: pct(ship.avg_winrate, ship.expected_winrate),
        frags: pct(ship.avg_frags, ship.expected_frags),
    }
}
