//! Port of `src/core/util/PersonalRating.ts`: personal rating calculation.

use std::collections::HashMap;

use crate::{
    models::{PrEntry, ShipStats},
    util::round_to,
};

/// `calRating` in `PersonalRating.ts` (formula by Wiochi, wows-numbers.com).
#[must_use]
pub fn cal_rating(
    actual_dmg: f64,
    expected_dmg: f64,
    actual_wins: f64,
    expected_wins: f64,
    actual_frags: f64,
    expected_frags: f64,
) -> f64 {
    let r_dmg = actual_dmg / expected_dmg;
    let r_wins = actual_wins / expected_wins;
    let r_frags = actual_frags / expected_frags;

    let n_dmg = ((r_dmg - 0.4) / (1.0 - 0.4)).max(0.0);
    let n_frags = ((r_frags - 0.1) / (1.0 - 0.1)).max(0.0);
    let n_wins = ((r_wins - 0.7) / (1.0 - 0.7)).max(0.0);

    let rating = round_to(Some(700.0 * n_dmg + 300.0 * n_frags + 150.0 * n_wins), 0);
    rating.min(9999.0)
}

/// `getOverallRating` in `PersonalRating.ts`: computes per-ship rating/AP and
/// the player's overall rating from their ship stats and the PR table.
#[must_use]
pub fn get_overall_rating(ships: &mut [ShipStats], pr: &HashMap<u64, PrEntry>) -> f64 {
    let mut actual_dmg = 0.0;
    let mut expected_dmg = 0.0;
    let mut actual_wins = 0.0;
    let mut expected_wins = 0.0;
    let mut actual_frags = 0.0;
    let mut expected_frags = 0.0;

    for ship in ships.iter_mut() {
        ship.rating = -1.0;
        ship.ap = 0.0;

        let Some(pvp) = ship.pvp.as_ref() else {
            continue;
        };
        let Some(overall) = pr.get(&ship.ship_id) else {
            continue;
        };
        if pvp.battles == 0 {
            continue;
        }

        let curr_avg_dmg = pvp.damage_dealt as f64 / pvp.battles as f64;
        let curr_winrate = pvp.wins as f64 / pvp.battles as f64 * 100.0;
        let curr_frags = pvp.frags as f64 / pvp.battles as f64;

        ship.avg_dmg = curr_avg_dmg;
        ship.avg_winrate = curr_winrate;
        ship.avg_frags = curr_frags;

        actual_dmg += curr_avg_dmg;
        actual_wins += curr_winrate;
        actual_frags += curr_frags;
        expected_dmg += overall.average_damage_dealt;
        expected_wins += overall.win_rate;
        expected_frags += overall.average_frags;

        let rating = cal_rating(
            curr_avg_dmg,
            overall.average_damage_dealt,
            curr_winrate,
            overall.win_rate,
            curr_frags,
            overall.average_frags,
        );
        ship.rating = rating;
        ship.ap = get_ap(rating, pvp.battles);
    }

    cal_rating(
        actual_dmg,
        expected_dmg,
        actual_wins,
        expected_wins,
        actual_frags,
        expected_frags,
    )
}

/// `getAP` in `PersonalRating.ts`.
#[must_use]
pub fn get_ap(rating: f64, battle: i64) -> f64 {
    if rating == -1.0 || battle == 0 {
        0.0
    } else {
        round_to(Some((10.0f64.max(battle as f64).log10()) * rating), 0)
    }
}

/// `getRatingRange` in `PersonalRating.ts`.
pub const RATING_RANGE: [f64; 9] = [
    0.0, 750.0, 1100.0, 1350.0, 1550.0, 1750.0, 2100.0, 2450.0, 9999.0,
];

/// `getRatingIndex` in `PersonalRating.ts`: first range boundary strictly
/// greater than the rating; unknown (0) for missing or past-the-end ratings.
#[must_use]
pub fn get_rating_index(rating: Option<f64>) -> usize {
    let Some(rating) = rating else { return 0 };
    RATING_RANGE.iter().position(|r| rating < *r).unwrap_or(0)
}

/// `getColourList` in `PersonalRating.ts`.
pub const COLOUR_LIST: [&str; 10] = [
    "#607D8B", "#D32F2F", "#FF9800", "#FFB300", "#7CB342", "#388E3C", "#03A9F4", "#9C27B0",
    "#673AB7", "black",
];

/// `getColour` in `PersonalRating.ts`.
#[must_use]
pub fn get_colour(rating: Option<f64>) -> &'static str {
    let colours = COLOUR_LIST;
    colours
        .get(get_rating_index(rating))
        .copied()
        .unwrap_or("#607D8B")
}

/// Default English rating labels. The TypeScript app reads these from
/// `value/lang.ts`; shells may override them for other locales.
pub const RATING_LABELS: [&str; 9] = [
    "Unknown",
    "Bad",
    "Below Average",
    "Average",
    "Good",
    "Very Good",
    "Great",
    "Unicum",
    "Super Unicum",
];

/// `getComment` in `PersonalRating.ts`: `"{label} (+{diff})"`.
#[must_use]
pub fn get_comment(rating: f64) -> String {
    let index = get_rating_index(Some(rating));
    let comment = RATING_LABELS[index];
    let range = RATING_RANGE[index];
    let mut diff = range - rating;
    if range == 9999.0 {
        // TypeScript computes `diff = range - rating` first, then overrides
        // the top bucket to measure from 2450.
        diff = rating - 2450.0;
    }
    format!("{comment} (+{diff})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cal_rating_baseline_is_1150() {
        // Equal actual and expected values: 700 + 300 + 150 = 1150.
        assert_eq!(cal_rating(100.0, 100.0, 50.0, 50.0, 1.0, 1.0), 1150.0);
    }

    #[test]
    fn cal_rating_floors_negative_components() {
        // Everything below the minimum normalisation bounds -> 0.
        assert_eq!(cal_rating(0.0, 100.0, 0.0, 50.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn cal_rating_caps_at_9999() {
        assert_eq!(
            cal_rating(1_000_000.0, 100.0, 100.0, 50.0, 100.0, 1.0),
            9999.0
        );
    }

    #[test]
    fn ap_formula() {
        assert_eq!(get_ap(-1.0, 100), 0.0);
        assert_eq!(get_ap(1150.0, 0), 0.0);
        // log10(100) * 1150 = 2300
        assert_eq!(get_ap(1150.0, 100), 2300.0);
        // battle below 10 uses 10: log10(10) * 1150 = 1150
        assert_eq!(get_ap(1150.0, 5), 1150.0);
    }

    #[test]
    fn rating_index_boundaries() {
        assert_eq!(get_rating_index(None), 0);
        assert_eq!(get_rating_index(Some(0.0)), 1);
        assert_eq!(get_rating_index(Some(749.0)), 1);
        assert_eq!(get_rating_index(Some(750.0)), 2);
        assert_eq!(get_rating_index(Some(1100.0)), 3);
        assert_eq!(get_rating_index(Some(2450.0)), 8);
        assert_eq!(get_rating_index(Some(9999.0)), 0);
    }

    #[test]
    fn colour_matches_rating() {
        assert_eq!(get_colour(None), "#607D8B");
        assert_eq!(get_colour(Some(500.0)), "#D32F2F");
        assert_eq!(get_colour(Some(3000.0)), "#673AB7");
    }

    #[test]
    fn comment_format() {
        assert_eq!(get_comment(1500.0), "Good (+50)");
        assert_eq!(get_comment(749.0), "Bad (+1)");
        assert_eq!(get_comment(3000.0), "Super Unicum (+550)");
    }

    #[test]
    fn overall_rating_aggregates_and_writes_ship_fields() {
        let mut pr = HashMap::new();
        pr.insert(
            1,
            PrEntry {
                average_damage_dealt: 50_000.0,
                average_frags: 1.0,
                win_rate: 50.0,
            },
        );

        let mut ships = vec![ShipStats {
            ship_id: 1,
            battles: 10,
            wins: 5,
            damage_dealt: 500_000,
            frags: 10,
            pvp: Some(crate::models::PvpStats {
                battles: 10,
                wins: 5,
                damage_dealt: 500_000,
                frags: 10,
            }),
            rating: 0.0,
            ap: 0.0,
            avg_dmg: 0.0,
            avg_winrate: 0.0,
            avg_frags: 0.0,
        }];

        let overall = get_overall_rating(&mut ships, &pr);
        // Matching expected values -> 1150.
        assert_eq!(overall, 1150.0);
        assert_eq!(ships[0].avg_dmg, 50_000.0);
        assert_eq!(ships[0].avg_winrate, 50.0);
        assert_eq!(ships[0].avg_frags, 1.0);
        assert_eq!(ships[0].rating, 1150.0);
        assert_eq!(ships[0].ap, get_ap(1150.0, 10));
    }

    #[test]
    fn overall_rating_skips_unknown_and_empty() {
        let pr = HashMap::new();
        let mut ships = vec![
            ShipStats {
                ship_id: 999,
                pvp: Some(crate::models::PvpStats {
                    battles: 10,
                    wins: 5,
                    damage_dealt: 500_000,
                    frags: 10,
                }),
                ..Default::default()
            },
            ShipStats {
                ship_id: 1,
                pvp: Some(crate::models::PvpStats {
                    battles: 0,
                    wins: 0,
                    damage_dealt: 0,
                    frags: 0,
                }),
                ..Default::default()
            },
        ];
        assert_eq!(get_overall_rating(&mut ships, &pr), 0.0);
        assert_eq!(ships[0].rating, -1.0);
        assert_eq!(ships[0].ap, 0.0);
    }
}
