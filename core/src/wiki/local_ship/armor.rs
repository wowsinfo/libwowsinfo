//! Armor digest view model.

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::components::{GunStats, HullStats};

/// One hull-armor thickness group (zone distribution).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ZoneThicknessGroup {
    pub thickness: f64,
    pub count: i64,
}

/// One turret's armor block (turret face + barbette).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TurretArmorView {
    pub name: String,
    pub caliber: f64,
    pub barrels: i64,
    pub armor: f64,
    pub barbette: f64,
}

/// The armor digest for the ship detail screen.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct ArmorView {
    pub zone_count: i64,
    pub max_zone_thickness: f64,
    pub zone_groups: Vec<ZoneThicknessGroup>,
    pub turrets: Vec<TurretArmorView>,
}


/// Build the armor digest from the hull `armor` block plus main-battery
/// turret data. The barbette value is the thickest zone in the turret's
/// barbette group (there is no plate geometry in `wowsinfo.json`).
pub(super) fn armor_view(hull: &Option<HullStats>, main_battery: Option<&GunStats>) -> Option<ArmorView> {
    let armor = hull.as_ref()?.armor.as_ref()?;
    let mut zone_groups: Vec<ZoneThicknessGroup> = Vec::new();
    for zone in &armor.zones {
        match zone_groups
            .iter_mut()
            .find(|group| (group.thickness - zone.thickness).abs() < f64::EPSILON)
        {
            Some(group) => group.count += 1,
            None => zone_groups.push(ZoneThicknessGroup {
                thickness: zone.thickness,
                count: 1,
            }),
        }
    }
    zone_groups.sort_by(|a, b| b.thickness.total_cmp(&a.thickness));
    let turrets = main_battery
        .map(|guns| {
            guns.turrets
                .iter()
                .map(|turret| TurretArmorView {
                    name: turret.name.clone(),
                    caliber: turret.caliber,
                    barrels: turret.barrels,
                    armor: turret.armor,
                    barbette: armor
                        .barbettes
                        .iter()
                        .find(|barbette| barbette.turret == turret.name)
                        .map(|barbette| barbette.max_thickness)
                        .unwrap_or(0.0),
                })
                .collect()
        })
        .unwrap_or_default();
    Some(ArmorView {
        zone_count: armor.zones.len() as i64,
        max_zone_thickness: armor.zones.first().map(|zone| zone.thickness).unwrap_or(0.0),
        zone_groups,
        turrets,
    })
}
