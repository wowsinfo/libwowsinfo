//! Warship filtering tests.

use std::collections::HashMap;

use super::core::*;
use crate::models::EncyclopediaShip;

fn ship(
    id: u64,
    name: &str,
    nation: &str,
    r#type: &str,
    tier: i64,
    premium: bool,
) -> EncyclopediaShip {
    EncyclopediaShip {
        ship_id: id,
        name: name.to_string(),
        nation: nation.to_string(),
        r#type: r#type.to_string(),
        tier,
        premium,
        ..Default::default()
    }
}

#[test]
fn tier_labels_match() {
    assert_eq!(get_tier_label(1), "I");
    assert_eq!(get_tier_label(10), "X");
    assert_eq!(get_tier_label(11), "★");
    assert_eq!(get_tier_label(0), "O");
    assert_eq!(get_tier_label(12), "O");
}

#[test]
fn colour_scale_is_red_to_green() {
    // Out-of-range returns the literal `#FF0000` (as in the TS source);
    // computed colours use lowercase hex from `toString(16)`.
    assert_eq!(get_colour_with_range(0.0, -1.0, 100.0), "#FF0000");
    assert_eq!(get_colour_with_range(0.0, 0.0, 100.0), "#ff0000");
    // Halfway: 127.5 -> "7f" via JS toString(16) truncation.
    assert_eq!(get_colour_with_range(0.0, 50.0, 100.0), "#7f7f00");
    assert_eq!(get_colour_with_range(0.0, 100.0, 100.0), "#00ff00");
}

#[test]
fn normalise_maps_encyclopedia_values_and_tiers() {
    let mut enc = crate::models::EncyclopediaInfo::default();
    enc.ship_nations
        .insert("usa".to_string(), "U.S.A.".to_string());
    enc.ship_types
        .insert("dd".to_string(), "Destroyer".to_string());
    let data = normalise(
        &["U.S.A.".to_string()],
        &["Destroyer".to_string()],
        &["VIII".to_string()],
        &enc,
    );
    assert!(data.nation.contains("usa"));
    assert!(data.r#type.contains("dd"));
    assert!(data.tier.contains(&8));
}

#[test]
fn valid_ship_matches_filters() {
    let curr = ship(1, "Montana", "usa", "bb", 10, false);
    let fdata = NormalisedFilter {
        nation: ["usa".to_string()].into_iter().collect(),
        r#type: ["bb".to_string()].into_iter().collect(),
        tier: [10].into_iter().collect(),
    };
    assert!(valid_ship(&curr, "mont", &fdata, false));
    assert!(!valid_ship(&curr, "montana", &fdata, true));
    assert!(!valid_ship(&curr, "yamato", &fdata, false));
}

#[test]
fn filter_warships_sorts_by_tier_then_type() {
    let mut warship = HashMap::new();
    warship.insert(1, ship(1, "A", "usa", "dd", 5, false));
    warship.insert(2, ship(2, "B", "usa", "bb", 10, false));
    warship.insert(3, ship(3, "C", "usa", "bb", 8, true));
    let filter = ShipFilter {
        premium: false,
        name: String::new(),
        nation: vec!["usa".to_string()],
        r#type: vec![],
        tier: vec![],
    };
    let result = filter_warships(&filter, &warship).expect("non-empty filter");
    // `premium === false` means "no premium filter", so the premium ship
    // passes too, sorted tier desc then type: bb(10), bb(8), dd(5).
    assert_eq!(
        result.iter().map(|s| s.ship_id).collect::<Vec<_>>(),
        vec![2, 3, 1]
    );
}

#[test]
fn empty_filter_returns_none() {
    let warship = HashMap::new();
    assert!(filter_warships(&ShipFilter::default(), &warship).is_none());
}
