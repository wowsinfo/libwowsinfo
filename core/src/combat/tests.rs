//! Tests for ballistics and team-advantage scoring.

use super::*;

fn shell() -> Shell {
    // A realistic battleship AP shell (406 mm, ~1225 kg).
    Shell {
        mass_kg: 1225.0,
        diameter_m: 0.406,
        drag_coefficient: 0.25,
        muzzle_velocity: 800.0,
    }
}

#[test]
fn air_density_at_sea_level_is_standard() {
    let density = air_density(0.0);
    assert!((density - 1.225).abs() < 0.01, "density was {density}");
    assert!(air_density(10_000.0) < density);
    assert_eq!(air_density(50_000.0), 0.0, "no atmosphere above 44km");
}

#[test]
fn trajectory_is_monotonic_in_range_and_falls_back() {
    let points = simulate_trajectory(&shell(), 20.0, 0.0, 0.05, 60.0);
    assert!(points.len() > 2);
    for pair in points.windows(2) {
        assert!(pair[1].x >= pair[0].x);
        assert!(pair[1].time > pair[0].time);
    }
    let last = points.last().expect("points");
    assert!(last.y <= 0.0, "shell must land, last y = {}", last.y);
    assert!(last.x > 0.0);
}

#[test]
fn solve_for_range_finds_a_landing_angle() {
    let range = 8_000.0;
    let angle = solve_for_range(&shell(), range, 0.05).expect("reachable range");
    let landed = simulate_trajectory(&shell(), angle, 0.0, 0.05, 120.0)
        .last()
        .expect("points")
        .x;
    assert!((landed - range).abs() < 200.0, "landed at {landed}, wanted {range}");
}

#[test]
fn unreachable_range_returns_none() {
    assert!(solve_for_range(&shell(), 1_000_000.0, 0.05).is_none());
    assert!(solve_for_range(&shell(), 0.0, 0.05).is_none());
}

fn fleet(dd: u64, cl: u64, bb: u64, ss: u64, cv: u64, hp: f64) -> FleetState {
    let class = |alive: u64| FleetClass {
        alive,
        hp: hp * alive as f64,
        max_hp: hp * alive as f64,
    };
    FleetState {
        destroyers: class(dd),
        cruisers: class(cl),
        battleships: class(bb),
        submarines: class(ss),
        carriers: class(cv),
    }
}

#[test]
fn even_battle_is_even() {
    let battle = BattleState {
        team0: fleet(3, 3, 3, 1, 2, 100.0),
        team1: fleet(3, 3, 3, 1, 2, 100.0),
        score0: 500.0,
        score1: 500.0,
        win_score: 1000.0,
        cap_income0: 0.6,
        cap_income1: 0.6,
        time_left: 600.0,
    };
    let result = evaluate(&battle);
    assert!((result.total.0 - result.total.1).abs() < 1e-9);
    assert_eq!(result.level, AdvantageLevel::Even);
}

#[test]
fn fleet_power_is_proportional_and_class_weighted() {
    // 6 destroyers vs 6 battleships: destroyers are heavier per ship.
    let battle = BattleState {
        team0: fleet(6, 0, 0, 0, 0, 100.0),
        team1: fleet(0, 0, 6, 0, 0, 100.0),
        ..Default::default()
    };
    let result = evaluate(&battle);
    assert!(result.fleet_power.0 > result.fleet_power.1);
    assert!((result.fleet_power.0 + result.fleet_power.1 - 10.0).abs() < 1e-9);
}

#[test]
fn score_gap_is_reflected_in_projection() {
    let battle = BattleState {
        team0: fleet(1, 1, 1, 0, 1, 100.0),
        team1: fleet(1, 1, 1, 0, 1, 100.0),
        score0: 800.0,
        score1: 500.0,
        win_score: 1000.0,
        cap_income0: 0.6,
        cap_income1: 0.6,
        time_left: 600.0,
    };
    let result = evaluate(&battle);
    assert!(result.score_projection.0 > result.score_projection.1);
    assert!(result.total.0 > result.total.1);
}

#[test]
fn eliminated_team_is_absolute() {
    let battle = BattleState {
        team0: fleet(0, 0, 0, 0, 0, 100.0),
        team1: fleet(1, 1, 1, 0, 1, 100.0),
        ..Default::default()
    };
    let result = evaluate(&battle);
    assert!(result.team_eliminated);
    assert_eq!(result.total, (0.0, 25.0));
    assert_eq!(result.level, AdvantageLevel::Absolute);
}
