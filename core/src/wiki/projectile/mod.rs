//! Projectile parsing (`wowsinfo.json -> projectiles`).
//!
//! Every shell, bomb and torpedo in the game data is keyed by its internal
//! name (for example `PAPA011_Shell_406mm_AP_AP_Mk_8`). The parsed structs
//! feed the wiki's shell cards and the drag-based AP penetration chart.
//! Split into domain submodules so every file stays small.

mod helpers;
mod parse;
#[cfg(test)]
mod tests;
mod types;

pub use parse::parse_projectiles;
pub use types::{ApInfo, ProjectileInfo};

