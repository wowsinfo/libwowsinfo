//! Game-data parsing (`wowsinfo.json` -> typed `GameData`).
//! Split into domain submodules so every file stays small.

mod helpers;
mod parse;
mod types;

pub use parse::parse_game_data;
pub use types::{
    AbilityInfo, AchievementInfo, CommanderSkill, ConsumableInfo, GameData,
    ModernizationInfo, ShipInfo, SkillInfo,
};

