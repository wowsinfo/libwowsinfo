//! Battles-weighted per-class averages from a player's ship list.

use crate::models::ShipStatLine;

/// Aggregated averages for one ship class.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClassAverage {
    pub class: String,
    pub battles: i64,
    pub avg_dmg: f64,
    pub avg_winrate: f64,
    pub avg_frags: f64,
    pub avg_xp: f64,
    /// Survival rate as a percentage of battles.
    pub survival: f64,
    /// Main-battery hit rate as a percentage of shots.
    pub accuracy: f64,
}

/// Group the player's ships by class and compute battles-weighted averages,
/// mirroring the per-class chart lines in community-assistant. Ships without
/// battles are ignored.
#[must_use]
pub fn per_class_averages(ships: &[ShipStatLine]) -> Vec<ClassAverage> {
    let mut totals: Vec<ClassTotals> = Vec::new();

    for ship in ships {
        if ship.battles <= 0 {
            continue;
        }
        let battles = ship.battles as f64;
        let pvp = ship.statistics.pvp.as_ref();
        let avg_xp = pvp.map(|p| p.xp as f64 / battles).unwrap_or(0.0);
        let survival = pvp
            .map(|p| p.survived_battles as f64 / battles * 100.0)
            .unwrap_or(0.0);
        let accuracy = pvp
            .and_then(|p| p.main_battery.as_ref())
            .filter(|weapon| weapon.shots > 0)
            .map(|weapon| weapon.hits as f64 / weapon.shots as f64 * 100.0)
            .unwrap_or(0.0);

        if let Some(entry) = totals.iter_mut().find(|entry| entry.class == ship.r#type) {
            entry.battles += battles;
            entry.dmg += ship.avg_dmg * battles;
            entry.winrate += ship.avg_winrate * battles;
            entry.frags += ship.avg_frags * battles;
            entry.xp += avg_xp * battles;
            entry.survival += survival * battles;
            entry.accuracy += accuracy * battles;
        } else {
            totals.push(ClassTotals {
                class: ship.r#type.clone(),
                battles,
                dmg: ship.avg_dmg * battles,
                winrate: ship.avg_winrate * battles,
                frags: ship.avg_frags * battles,
                xp: avg_xp * battles,
                survival: survival * battles,
                accuracy: accuracy * battles,
            });
        }
    }

    let mut result: Vec<ClassAverage> = totals
        .into_iter()
        .map(|totals| ClassAverage {
            class: totals.class,
            battles: totals.battles as i64,
            avg_dmg: totals.dmg / totals.battles,
            avg_winrate: totals.winrate / totals.battles,
            avg_frags: totals.frags / totals.battles,
            avg_xp: totals.xp / totals.battles,
            survival: totals.survival / totals.battles,
            accuracy: totals.accuracy / totals.battles,
        })
        .collect();
    result.sort_by_key(|entry| std::cmp::Reverse(entry.battles));
    result
}

/// Per-class accumulation (battles-weighted sums before averaging).
struct ClassTotals {
    class: String,
    battles: f64,
    dmg: f64,
    winrate: f64,
    frags: f64,
    xp: f64,
    survival: f64,
    accuracy: f64,
}
