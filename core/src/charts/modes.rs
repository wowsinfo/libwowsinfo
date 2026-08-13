//! Game-mode battle distribution for the pie chart.

use crate::models::{PlayerStatistics, PvpStats};

/// One slice of the game-modes pie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeBattles {
    pub mode: &'static str,
    pub battles: i64,
}

/// Battles split across PvP / Solo / Div2 / Div3 / PvE / Rank. Modes the player
/// never played are omitted, matching the app's chart.
#[must_use]
pub fn mode_distribution(stats: &PlayerStatistics) -> Vec<ModeBattles> {
    let modes = [
        ("PvP", stats.pvp.as_ref()),
        ("Solo", stats.solo.as_ref()),
        ("Div2", stats.div2.as_ref()),
        ("Div3", stats.div3.as_ref()),
        ("PvE", stats.pve.as_ref()),
        ("Rank", stats.rank_solo.as_ref()),
    ];
    modes
        .into_iter()
        .filter_map(|(mode, pvp): (&str, Option<&PvpStats>)| {
            pvp.filter(|pvp| pvp.battles > 0)
                .map(|pvp| ModeBattles { mode, battles: pvp.battles })
        })
        .collect()
}
