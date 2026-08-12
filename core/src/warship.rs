//! Port of `src/core/util/WarshipTool.ts`: tier labels, colour scaling and
//! ship filtering.

use std::collections::{HashMap, HashSet};

use crate::models::{EncyclopediaShip, ShipStats};

/// `getTierList` in `WarshipTool.ts`.
pub const TIER_LIST: [&str; 11] = [
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "★",
];

/// `getTierLabel` in `WarshipTool.ts`: returns `"O"` for invalid tiers.
#[must_use]
pub fn get_tier_label(tier: i64) -> String {
    if tier < 1 {
        return "O".to_string();
    }
    TIER_LIST
        .get((tier - 1) as usize)
        .map_or_else(|| "O".to_string(), |t| (*t).to_string())
}

/// `getColourWithRange` in `WarshipTool.ts`: a red-to-green scale.
#[must_use]
pub fn get_colour_with_range(min: f64, curr: f64, max: f64) -> String {
    if curr < min || max <= min {
        return "#FF0000".to_string();
    }
    let scale = ((curr - min) / (max - min)) * 100.0;
    let g = 255.0 * scale / 100.0;
    let r = 255.0 * (100.0 - scale) / 100.0;
    rgb_to_hex(r, g, 0.0)
}

fn component_to_hex(c: f64) -> String {
    let hex = format!("{:x}", c as u32);
    if hex.len() == 1 {
        format!("0{hex}")
    } else {
        hex
    }
}

fn rgb_to_hex(r: f64, g: f64, b: f64) -> String {
    format!(
        "#{}{}{}",
        component_to_hex(r),
        component_to_hex(g),
        component_to_hex(b)
    )
}

/// `getKeyByValue` in `WarshipTool.ts`: first key whose value matches.
#[must_use]
pub fn get_key_by_value<'a>(
    object: &'a HashMap<String, String>,
    value: &str,
) -> Option<&'a String> {
    object
        .iter()
        .find_map(|(k, v)| if v == value { Some(k) } else { None })
}

/// Normalised filter sets (`normalise` in `WarshipTool.ts`).
#[derive(Debug, Clone, Default)]
pub struct NormalisedFilter {
    pub nation: HashSet<String>,
    pub r#type: HashSet<String>,
    pub tier: HashSet<u32>,
}

/// `normalise` in `WarshipTool.ts`: maps display names to encyclopedia IDs and
/// tier labels to numeric tiers.
#[must_use]
pub fn normalise(
    nation: &[String],
    r#type: &[String],
    tier: &[String],
    encyclopedia: &crate::models::EncyclopediaInfo,
) -> NormalisedFilter {
    let mut data = NormalisedFilter::default();
    for i in nation {
        if let Some(key) = get_key_by_value(&encyclopedia.ship_nations, i) {
            data.nation.insert(key.clone());
        }
    }
    for i in r#type {
        if let Some(key) = get_key_by_value(&encyclopedia.ship_types, i) {
            data.r#type.insert(key.clone());
        }
    }
    for i in tier {
        if let Some(index) = TIER_LIST.iter().position(|t| *t == i) {
            data.tier.insert((index + 1) as u32);
        }
    }
    data
}

/// `validShip` in `WarshipTool.ts`.
#[must_use]
pub fn valid_ship(
    curr: &EncyclopediaShip,
    fname: &str,
    fdata: &NormalisedFilter,
    premium: bool,
) -> bool {
    let filter_name = curr.name.to_lowercase().contains(fname) || fname.trim().is_empty();
    let filter_premium = curr.premium == premium || !premium;
    let filter_tier = fdata.tier.contains(&(curr.tier as u32)) || fdata.tier.is_empty();
    let filter_nation = fdata.nation.contains(&curr.nation) || fdata.nation.is_empty();
    let filter_type = fdata.r#type.contains(&curr.r#type) || fdata.r#type.is_empty();
    filter_name && filter_nation && filter_premium && filter_tier && filter_type
}

/// Filter criteria as sent by the wiki UI (`filterShip`'s `data` argument).
#[derive(Debug, Clone, Default)]
pub struct ShipFilter {
    pub premium: bool,
    pub name: String,
    pub nation: Vec<String>,
    pub r#type: Vec<String>,
    pub tier: Vec<String>,
}

impl ShipFilter {
    /// Mirrors the early return in `filterShip`: no filter at all -> `None`.
    #[must_use]
    pub fn is_empty_filter(&self) -> bool {
        !self.premium
            && self.name.is_empty()
            && self.nation.is_empty()
            && self.r#type.is_empty()
            && self.tier.is_empty()
    }
}

/// `filterShip` in `WarshipTool.ts` over player ship stats; ships whose wiki
/// entry is missing are ignored.
#[must_use]
pub fn filter_ships(
    data: &ShipFilter,
    ship_data: &[ShipStats],
    warship: &HashMap<u64, EncyclopediaShip>,
) -> Option<Vec<ShipStats>> {
    if data.is_empty_filter() {
        return None;
    }
    let fname = data.name.to_lowercase();
    let fdata = NormalisedFilter {
        nation: data.nation.iter().cloned().collect(),
        r#type: data.r#type.iter().cloned().collect(),
        tier: HashSet::new(),
    };
    let filtered = ship_data
        .iter()
        .filter(|ship| {
            warship
                .get(&ship.ship_id)
                .is_some_and(|curr| valid_ship(curr, &fname, &fdata, data.premium))
        })
        .cloned()
        .collect();
    Some(filtered)
}

/// `filterShip` in `WarshipTool.ts` over the whole wiki catalogue, sorted by
/// tier desc then type, like the TypeScript version.
#[must_use]
pub fn filter_warships(
    data: &ShipFilter,
    warship: &HashMap<u64, EncyclopediaShip>,
) -> Option<Vec<EncyclopediaShip>> {
    if data.is_empty_filter() {
        return None;
    }
    let fname = data.name.to_lowercase();
    let fdata = NormalisedFilter {
        nation: data.nation.iter().cloned().collect(),
        r#type: data.r#type.iter().cloned().collect(),
        tier: HashSet::new(),
    };
    let mut filtered: Vec<EncyclopediaShip> = warship
        .values()
        .filter(|curr| valid_ship(curr, &fname, &fdata, data.premium))
        .cloned()
        .collect();
    filtered.sort_by(|a, b| {
        if a.tier == b.tier {
            a.r#type.cmp(&b.r#type)
        } else {
            b.tier.cmp(&a.tier)
        }
    });
    Some(filtered)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
