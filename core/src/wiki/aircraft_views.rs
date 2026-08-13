//! Carrier aircraft views for the wiki detail screen.

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::aircraft::AircraftInfo;
use super::gamedata::{GameData, ShipInfo};
use super::local_ship::ShellView;
use super::modifiers::ModifierSet;
use super::ship_builder::ModuleSelection;
use super::LangMap;

/// Resolved carrier aircraft detail.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AircraftDetail {
    pub key: String,
    pub name: String,
    pub r#type: String,
    pub health: f64,
    pub total_planes: i64,
    pub speed: f64,
    pub visibility: f64,
    pub attack_count: Option<i64>,
    pub attacker: Option<i64>,
    pub max_aircraft: Option<i64>,
    pub restore_time: Option<f64>,
    pub bomb: Option<ShellView>,
    /// Squadron stats after the selected skill modifiers are applied.
    pub adjusted_health: f64,
    pub adjusted_speed: f64,
}

impl AircraftDetail {
    fn from_aircraft(
        lang: &LangMap,
        data: &GameData,
        aircraft: &AircraftInfo,
        mods: &ModifierSet,
        ship_class: &str,
    ) -> Self {
        let bomb = aircraft
            .bomb_name
            .as_deref()
            .and_then(|key| data.projectiles.get(key))
            .map(|projectile| ShellView::from_projectile(lang, projectile));
        let (health_key, speed_key) = aircraft_type_keys(&aircraft.r#type);
        Self {
            key: aircraft.key.clone(),
            name: lang.get(&aircraft.name),
            r#type: aircraft.r#type.clone(),
            health: aircraft.health,
            total_planes: aircraft.total_planes,
            speed: aircraft.speed,
            visibility: aircraft.visibility,
            attack_count: aircraft.attack_count,
            attacker: aircraft.attacker,
            max_aircraft: aircraft.max_aircraft,
            restore_time: aircraft.restore_time,
            bomb,
            adjusted_health: aircraft.health
                * mods.multiply(ship_class, "planeHealthCoeff")
                * mods.multiply(ship_class, health_key),
            adjusted_speed: aircraft.speed
                * mods.multiply(ship_class, "planeSpeed")
                * mods.multiply(ship_class, speed_key),
        }
    }
}

/// Per-squadron-type modifier keys (health, speed).
fn aircraft_type_keys(r#type: &str) -> (&'static str, &'static str) {
    match r#type {
        "Fighter" => ("fighterHealth", "planeSpeed"),
        "Dive" => ("diveBomberHealth", "diveBomberSpeedMultiplier"),
        "Bomber" => ("torpedoBomberHealth", "torpedoSpeedMultiplier"),
        "Skip" => ("skipBomberHealth", "skipBomberSpeedMultiplier"),
        _ => ("planeHealthCoeff", "planeSpeed"),
    }
}

/// Resolve a squadron's aircraft key, following one level of indirection
/// through the ship's `components` (some options list component ids that are
/// arrays of aircraft keys instead of the aircraft keys themselves).
fn resolve_aircraft<'a>(
    data: &'a GameData,
    ship: &'a ShipInfo,
    key: &str,
) -> Option<&'a super::aircraft::AircraftInfo> {
    if let Some(aircraft) = data.aircraft.get(key) {
        return Some(aircraft);
    }
    ship.components
        .get(key)
        .and_then(Value::as_array)
        .and_then(|ids| ids.first())
        .and_then(Value::as_str)
        .and_then(|resolved| data.aircraft.get(resolved))
}

/// One aircraft squadron slot (fighter / torpedo / dive / skip bombers).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AircraftSlotView {
    pub slot: String,
    pub label: String,
    pub selected: i64,
    pub options: Vec<AircraftOptionView>,
}

/// One squadron option with its resolved aircraft.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct AircraftOptionView {
    pub index: i64,
    pub name: String,
    pub aircraft: Option<AircraftDetail>,
}

/// Build the squadron views for a carrier (or empty for other classes).
#[must_use]
pub fn aircraft_slot_views(
    data: &GameData,
    lang: &LangMap,
    ship: &ShipInfo,
    selection: ModuleSelection,
    mods: &ModifierSet,
    ship_class: &str,
) -> Vec<AircraftSlotView> {
    const SLOTS: [(&str, &str, &str); 4] = [
        ("_Fighter", "fighter", "Fighter"),
        ("_TorpedoBomber", "torpedoBomber", "Torpedo Bombers"),
        ("_DiveBomber", "diveBomber", "Dive Bombers"),
        ("_SkipBomber", "skipBomber", "Skip Bombers"),
    ];
    let selected = |slot: &str| match slot {
        "fighter" => selection.fighter,
        "torpedoBomber" => selection.torpedo_bomber,
        "diveBomber" => selection.dive_bomber,
        "skipBomber" => selection.skip_bomber,
        _ => 0,
    };
    SLOTS
        .iter()
        .filter_map(|(module_key, component_key, label)| {
            let options = ship.modules.get(*module_key)?.as_array()?;
            if options.is_empty() {
                return None;
            }
            let option_views = options
                .iter()
                .enumerate()
                .filter_map(|(index, option)| {
                    let name = option
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    let aircraft = option
                        .get("components")
                        .and_then(|c| c.get(*component_key))
                        .and_then(|ids| ids.as_array())
                        .and_then(|ids| ids.first())
                        .and_then(Value::as_str)
                        .and_then(|key| resolve_aircraft(data, ship, key))
                        .map(|aircraft| {
                            AircraftDetail::from_aircraft(lang, data, aircraft, mods, ship_class)
                        });
                    Some(AircraftOptionView {
                        index: index as i64,
                        name: lang.get(&name),
                        aircraft,
                    })
                })
                .collect();
            Some(AircraftSlotView {
                slot: (*component_key).to_string(),
                label: (*label).to_string(),
                selected: selected(*component_key) as i64,
                options: option_views,
            })
        })
        .collect()
}

/// Resolve the air-support plane (scout/fighter) for a ship, if any.
#[must_use]
pub fn air_support_plane(
    data: &GameData,
    lang: &LangMap,
    plane_key: &str,
    mods: &ModifierSet,
    ship_class: &str,
) -> Option<AircraftDetail> {
    data.aircraft
        .get(plane_key)
        .map(|aircraft| AircraftDetail::from_aircraft(lang, data, aircraft, mods, ship_class))
}
