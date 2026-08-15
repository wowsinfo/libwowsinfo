//! Warship filtering helpers (tier labels, colours, filters).

use std::collections::{HashMap, HashSet};

use crate::models::{EncyclopediaShip, ShipStats};

/// `getTierList` in `WarshipTool.ts`.
pub const TIER_LIST: [&str; 11] = [
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "★",
];

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

