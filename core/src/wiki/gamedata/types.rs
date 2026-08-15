//! Game-data type definitions (one typed struct per top-level section).

use std::collections::HashMap;

use serde_json::Value;

use crate::wiki::modifiers::ModifierSet;

/// One consumable slot entry (`ships.<id>.consumables`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumableInfo {
    pub name: String,
    pub r#type: String,
}

/// One ship entry (`ships.<id>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShipInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub year: String,
    pub paper_ship: bool,
    pub index: String,
    pub tier: i64,
    pub region: String,
    pub r#type: String,
    pub region_id: String,
    pub type_id: String,
    pub group: String,
    pub cost_xp: i64,
    pub cost_gold: i64,
    pub cost_cr: i64,
    pub consumables: Vec<Vec<ConsumableInfo>>,
    pub next_ships: Vec<u64>,
    pub permoflages: Vec<String>,
    /// Raw module/component trees; the sub-shapes are game-data specific and
    /// kept opaque for forward compatibility.
    pub modules: Value,
    pub components: Value,
}

/// One commander skill (`commandSkills.<class>[tier][column]`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommanderSkill {
    pub name: String,
    pub tier: i64,
    pub column: i64,
    pub description: String,
    pub icon: String,
}

/// One ability/consumable entry (`abilities.<id>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbilityInfo {
    pub id: u64,
    pub nation: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub filter: String,
    pub r#type: String,
    /// Raw modifier map (`abilities`), kept opaque.
    pub abilities: Value,
    /// Raw upgrade alter map (`alter`).
    pub alter: Value,
}

/// One achievement entry (`achievements.<id>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementInfo {
    pub id: u64,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub r#type: Vec<String>,
    /// Raw constants map.
    pub constants: Value,
}

/// One modernization (module upgrade) entry (`modernizations.<key>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModernizationInfo {
    pub key: String,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub slot: i64,
    pub cost_cr: i64,
    pub levels: Vec<i64>,
    pub r#types: Vec<String>,
    pub nations: Vec<String>,
    pub ships: Vec<u64>,
    pub excludes: Vec<u64>,
    pub modifiers: ModifierSet,
}

/// One flag / signal exterior (`exteriors.<key>` with type `Flags`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagInfo {
    pub key: String,
    pub name: String,
    pub description: String,
    pub cost_cr: i64,
    pub modifiers: ModifierSet,
}

/// One commander skill (`skills.<key>`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillInfo {
    pub key: String,
    pub name: String,
    pub description: String,
    /// Skill tier per ship class (`tier: { "Cruiser": 3 }`).
    pub tiers: HashMap<String, i64>,
    pub modifiers: ModifierSet,
    /// Trigger condition (`LogicTrigger.triggerType`), empty when passive.
    pub trigger_type: String,
    /// Modifiers applied while the trigger condition is active.
    pub trigger_modifiers: ModifierSet,
}

/// The parsed `wowsinfo.json` datasets.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameData {
    pub ships: HashMap<u64, ShipInfo>,
    pub abilities: HashMap<u64, AbilityInfo>,
    pub achievements: HashMap<u64, AchievementInfo>,
    /// Ship class -> tiers -> columns of commander skills.
    pub command_skills: HashMap<String, Vec<Vec<CommanderSkill>>>,
    /// Shells, bombs and torpedoes keyed by projectile name.
    pub projectiles: HashMap<String, crate::wiki::projectile::ProjectileInfo>,
    /// Carrier aircraft keyed by aircraft name.
    pub aircraft: HashMap<String, crate::wiki::aircraft::AircraftInfo>,
    /// Module upgrades keyed by modernization name.
    pub modernizations: HashMap<String, ModernizationInfo>,
    /// Signal flags (exteriors with type `Flags`).
    pub flags: Vec<FlagInfo>,
    /// Commander skills keyed by skill name.
    pub skills: HashMap<String, SkillInfo>,
    /// Exteriors (camos/skins/flags) keyed by key, mapped to their name IDS.
    pub exteriors: HashMap<String, String>,
}

