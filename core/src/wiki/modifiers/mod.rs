//! Generic modifier engine for commander skills, module upgrades and flags.
//!
//! Wargaming attaches a free-form modifier map to skills, modernizations and
//! exteriors: each key maps to either a plain number (all ship classes) or a
//! per-class dict. Values are multiplicative on top of the base stats, and a
//! few conditional keys (spotted/unspotted triggers, low-HP reload) are
//! resolved with the UI state. This mirrors the Flutter two `Modifiers` model
//! without hard-coding every key.
//! Split into domain submodules so every file stays small.

mod adjusted;
mod apply;
#[cfg(test)]
mod tests;
mod value;

pub use adjusted::AdjustedStats;
pub use apply::apply_modifiers;
pub use value::{is_additive_key, parse_modifiers, ModifierSet, ModifierValue};

