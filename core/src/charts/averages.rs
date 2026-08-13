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
    let mut totals: Vec<(String, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();

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

        if let Some(entry) = totals.iter_mut().find(|(class, _, _, _, _, _, _, _)| class == &ship.r#type)
        {
            entry.1 += battles;
            entry.2 += ship.avg_dmg * battles;
            entry.3 += ship.avg_winrate * battles;
            entry.4 += ship.avg_frags * battles;
            entry.5 += avg_xp * battles;
            entry.6 += survival * battles;
            entry.7 += accuracy * battles;
        } else {
            totals.push((
                ship.r#type.clone(),
                battles,
                ship.avg_dmg * battles,
                ship.avg_winrate * battles,
                ship.avg_frags * battles,
                avg_xp * battles,
                survival * battles,
                accuracy * battles,
            ));
        }
    }

    let mut result: Vec<ClassAverage> = totals
        .into_iter()
        .map(
            |(class, battles, dmg, winrate, frags, xp, survival, accuracy)| ClassAverage {
                class,
                battles: battles as i64,
                avg_dmg: dmg / battles,
                avg_winrate: winrate / battles,
                avg_frags: frags / battles,
                avg_xp: xp / battles,
                survival: survival / battles,
                accuracy: accuracy / battles,
            },
        )
        .collect();
    result.sort_by(|a, b| b.battles.cmp(&a.battles));
    result
}
