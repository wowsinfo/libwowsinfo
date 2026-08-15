//! Tests for the chart calculations.

use crate::charts::*;
use crate::models::{PlayerStatistics, PvpStats, ShipStatLine, WeaponStats};

fn make_ship(
    ship_id: u64,
    name: &str,
    class: &str,
    battles: i64,
    avg_dmg: f64,
    avg_winrate: f64,
    avg_frags: f64,
    expected_dmg: f64,
    rating: f64,
) -> ShipStatLine {
    ShipStatLine {
        ship_id,
        name: name.to_string(),
        index: String::new(),
        tier: 8,
        r#type: class.to_string(),
        nation: "pan_asia".to_string(),
        icon: String::new(),
        premium: false,
        battles,
        avg_dmg,
        avg_winrate,
        avg_frags,
        rating,
        rating_colour: String::new(),
        rating_comment: String::new(),
        ap: 0.0,
        statistics: PlayerStatistics {
            battles,
            pvp: Some(PvpStats {
                battles,
                xp: (avg_dmg * 10.0) as i64,
                survived_battles: battles / 2,
                main_battery: Some(WeaponStats {
                    shots: 100,
                    hits: 50,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        expected_dmg,
        expected_winrate: 50.0,
        expected_frags: 1.0,
        last_battle_time: 0,
    }
}

#[test]
fn radar_values_compare_against_expected() {
    let ship = make_ship(1, "A", "dd", 10, 40_000.0, 55.0, 1.5, 20_000.0, 2000.0);
    let radar = stats_vs_average(&ship);
    assert_eq!(radar.damage, 200.0);
    assert!((radar.winrate - 110.0).abs() < 1e-9);
    assert_eq!(radar.frags, 150.0);

    let no_expected = make_ship(2, "B", "bb", 5, 10.0, 10.0, 10.0, 0.0, 0.0);
    assert_eq!(stats_vs_average(&no_expected).damage, 0.0);
}

#[test]
fn per_class_averages_are_battles_weighted() {
    let ships = vec![
        make_ship(1, "A", "dd", 10, 10_000.0, 50.0, 1.0, 0.0, 0.0),
        make_ship(2, "B", "dd", 30, 20_000.0, 60.0, 2.0, 0.0, 0.0),
        make_ship(3, "C", "bb", 0, 999.0, 999.0, 999.0, 0.0, 0.0),
    ];
    let averages = per_class_averages(&ships);
    assert_eq!(averages.len(), 1);
    let dd = &averages[0];
    assert_eq!(dd.class, "dd");
    assert_eq!(dd.battles, 40);
    assert_eq!(dd.avg_dmg, 17_500.0);
    assert_eq!(dd.avg_winrate, 57.5);
    assert_eq!(dd.avg_frags, 1.75);
    assert_eq!(dd.survival, 50.0);
    assert_eq!(dd.accuracy, 50.0);
}

#[test]
fn top_ships_orders_by_battles_then_rating() {
    let ships = vec![
        make_ship(1, "A", "dd", 5, 0.0, 0.0, 0.0, 0.0, 100.0),
        make_ship(2, "B", "dd", 20, 0.0, 0.0, 0.0, 0.0, 50.0),
        make_ship(3, "C", "bb", 20, 0.0, 0.0, 0.0, 0.0, 200.0),
        make_ship(4, "D", "bb", 0, 0.0, 0.0, 0.0, 0.0, 999.0),
    ];
    let top = top_ships(&ships, 2);
    assert_eq!(top.len(), 2);
    assert_eq!(top[0].ship_id, 3, "more battles, higher rating wins tie");
    assert_eq!(top[1].ship_id, 2);
}

#[test]
fn mode_distribution_omits_unplayed_modes() {
    let stats = PlayerStatistics {
        battles: 10,
        pvp: Some(PvpStats {
            battles: 10,
            ..Default::default()
        }),
        div2: Some(PvpStats {
            battles: 3,
            ..Default::default()
        }),
        ..Default::default()
    };
    let modes = mode_distribution(&stats);
    assert_eq!(modes.len(), 2);
    assert_eq!(modes[0].mode, "PvP");
    assert_eq!(modes[0].battles, 10);
    assert_eq!(modes[1].mode, "Div2");
    assert_eq!(modes[1].battles, 3);
}
